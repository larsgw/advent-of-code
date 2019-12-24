fn convert_pointer (pointer: i64) -> usize {
    let new_pointer = if pointer >= 0 {
        Some(pointer as usize)
    } else {
        None
    };

    new_pointer.expect(&format!("pointer out of bounds: {}", pointer))
}

fn check_tape_length (state: &mut ProgramState, index: usize) {
    if index >= state.tape.len() {
        state.tape.resize(index + 1, 0);
    }
}

fn get_value (state: &mut ProgramState, pointer: i64, mode: i64) -> i64 {
    let converted_pointer = convert_pointer(pointer);
    check_tape_length(state, converted_pointer);

    let immediate_value = state.tape[converted_pointer];
    match mode {
        0 => get_value(state, immediate_value, 1),
        1 => immediate_value,
        2 => get_value(state, immediate_value + state.relative_base, 1),
        _ => panic!("unknown parameter mode")
    }
}

fn get_pointer (state: &mut ProgramState, pointer: i64, mode: i64) -> usize {
    let pointer_value = get_value(state, pointer, 1);
    let absolute_pointer = match mode {
        0 | 1 => pointer_value,
        2 => pointer_value + state.relative_base,
        _ => panic!("unknown pointer mode")
    };
    let converted_pointer = convert_pointer(absolute_pointer);
    check_tape_length(state, converted_pointer);

    converted_pointer
}

fn get_mode (modes: i64, i: i64) -> i64 {
    (modes / 10_i64.pow(i as u32)) % 10
}

fn get_parameters (state: &mut ProgramState, pointer: i64, number: (i64, i64), modes: i64) -> (Vec<i64>, Vec<usize>) {
    let mut values = (Vec::new(), Vec::new());

    for i in 0..number.0 {
        values.0.push(get_value(state, pointer + i, get_mode(modes, i)));
    }

    for i in number.0..(number.0 + number.1) {
        values.1.push(get_pointer(state, pointer + i, get_mode(modes, i)));
    }

    values
}

#[derive(Default)]
pub struct ProgramState {
    pub tape: Vec<i64>,
    pub input: Vec<i64>,
    pub tape_index: i64,
    pub input_index: usize,
    pub relative_base: i64
}

impl ProgramState {
    fn next_opcode (&self) -> i64 {
        self.tape[convert_pointer(self.tape_index)] % 100
    }

    fn needs_input (&self) -> bool {
        self.next_opcode() == 3 && self.input_index >= self.input.len()
    }

    fn is_done (&self) -> bool {
        self.next_opcode() == 99
    }
}

fn do_instruction (state: &mut ProgramState) -> Option<i64> {
    let instruction = get_value(state, state.tape_index, 1);
    state.tape_index += 1;

    let opcode = instruction % 100;
    let modes = instruction / 100;

    // number of (values, pointers)
    let parameters = match opcode {
        1 | 2 => (2, 1),
        3 => (0, 1),
        4 => (1, 0),
        5 | 6 => (2, 0),
        7 | 8 => (2, 1),
        9 => (1, 0),
        99 | _ => (0, 0)
    };

    let (values, pointers) = get_parameters(state, state.tape_index, parameters, modes);
    state.tape_index += parameters.0 + parameters.1;

    match opcode {
        1 => {
            state.tape[pointers[0]] = values[0] + values[1]
        },
        2 => {
            state.tape[pointers[0]] = values[0] * values[1]
        },
        3 => {
            state.tape[pointers[0]] = state.input[state.input_index];
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
            state.tape[pointers[0]] = (values[0] < values[1]) as i64
        },
        8 => {
            state.tape[pointers[0]] = (values[0] == values[1]) as i64
        },
        9 => {
            state.relative_base += values[0]
        },
        99 => {},
        _ => panic!("unkown opcode")
    }

    None
}

#[allow(dead_code)]
pub fn step (state: &mut ProgramState) -> Option<i64> {
    let mut output = None;
    while output.is_none() && !state.is_done() {
        output = do_instruction(state);
    }
    output
}

#[allow(dead_code)]
pub fn step_input (state: &mut ProgramState, input: Option<i64>) -> Vec<i64> {
    if input.is_some() { state.input.push(input.unwrap()); }
    let mut output = Vec::new();
    while !state.needs_input() && !state.is_done() {
        let result = do_instruction(state);
        if result.is_some() {
            output.push(result.unwrap());
        }
    }
    output
}

#[allow(dead_code)]
pub fn run (intcode: &Vec<i64>, input: &Vec<i64>) -> Vec<i64> {
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
