fn convert_pointer (pointer: i32) -> usize {
    let new_pointer = if pointer >= 0 {
        Some(pointer as usize)
    } else {
        None
    };

    new_pointer.expect(&format!("pointer out of bounds: {}", pointer))
}

fn get_value (state: &ProgramState, pointer: i32, mode: i32) -> i32 {
    let immediate_value = state.tape[convert_pointer(pointer)];
    match mode {
        0 => get_value(state, immediate_value, 1),
        1 => immediate_value,
        2 => get_value(state, immediate_value + state.relative_base, 1),
        _ => panic!("unkown parameter mode")
    }
}

fn get_values (state: &ProgramState, pointer: i32, number: (i32, i32), modes: i32) -> Vec<i32> {
    let mut values = Vec::new();

    for i in 0..number.0 {
        let mode = (modes / 10_i32.pow(i as u32)) % 10;
        values.push(get_value(state, pointer + i, mode));
    }

    for i in 0..number.1 {
        values.push(get_value(state, pointer + number.0 + i, 1));
    }

    values
}

pub struct ProgramState {
    tape: Vec<i32>,
    input: Vec<i32>,
    tape_index: i32,
    input_index: usize,
    relative_base: i32
}

pub fn step (state: &mut ProgramState) -> Option<i32> {
    loop {
        let instruction = get_value(&state, state.tape_index, 1);
        state.tape_index += 1;

        let opcode = instruction % 100;
        let modes = instruction / 100;

        // (input_params, output_params)
        let parameters = match opcode {
            1 | 2 => (2, 1),
            3 => (0, 1),
            4 => (1, 0),
            5 | 6 => (2, 0),
            7 | 8 => (2, 1),
            9 => (1, 0),
            99 | _ => (0, 0)
        };

        println!("{} {} {}", instruction, opcode, modes);
        let values = get_values(&state, state.tape_index, parameters, modes);
        state.tape_index += parameters.0 + parameters.1;

        match opcode {
            1 => {
                let target = convert_pointer(values[2]);
                state.tape[target] = values[0] + values[1]
            },
            2 => {
                let target = convert_pointer(values[2]);
                state.tape[target] = values[0] * values[1]
            },
            3 => {
                let target = convert_pointer(values[0]);
                state.tape[target] = state.input[state.input_index];
                state.input_index += 1
            },
            4 => {
                return Some(values[0])
            },
            5 => {
                if values[0] != 0 {
                    state.tape_index = values[1]
                }
            },
            6 => {
                if values[0] == 0 {
                    state.tape_index = values[1]
                }
            },
            7 => {
                let target = convert_pointer(values[2]);
                state.tape[target] = (values[0] < values[1]) as i32
            },
            8 => {
                let target = convert_pointer(values[2]);
                state.tape[target] = (values[0] == values[1]) as i32
            },
            9 => {
                state.relative_base += values[0]
            },
            99 => return None,
            _ => panic!("unkown opcode")
        }
    }
}

pub fn run (intcode: &Vec<i32>, input: &Vec<i32>) -> Vec<i32> {
    let mut output = Vec::new();
    let mut state = ProgramState {
        tape: intcode.to_vec(),
        input: input.to_vec(),
        tape_index: 0,
        input_index: 0,
        relative_base: 0
    };

    loop {
        let step_output = step(&mut state);
        if step_output.is_some() {
            output.push(step_output.unwrap())
        } else {
            break
        }
    }

    output
}
