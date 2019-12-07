fn convert_pointer (pointer: i32) -> usize {
    let new_pointer = if pointer >= 0 {
        Some(pointer as usize)
    } else {
        None
    };

    new_pointer.expect(&format!("pointer out of bounds: {}", pointer))
}

fn get_value (tape: &Vec<i32>, pointer: i32, mode: i32) -> i32 {
    let immediate_value = tape[convert_pointer(pointer)];
    if mode == 0 {
        get_value(tape, immediate_value, 1)
    } else {
        immediate_value
    }
}

fn get_values (tape: &Vec<i32>, pointer: i32, number: (i32, i32), modes: i32) -> Vec<i32> {
    let mut values = Vec::new();

    for i in 0..number.0 {
        let mode = modes & 10_i32.pow(i as u32);
        values.push(get_value(tape, pointer + i, mode));
    }

    for i in 0..number.1 {
        values.push(get_value(tape, pointer + number.0 + i, 1));
    }

    values
}

pub fn run (intcode: &Vec<i32>, input: &Vec<i32>) -> Vec<i32> {
    let mut tape = intcode.to_vec();
    let mut output = Vec::new();
    let mut input_index = 0;
    let mut cursor = 0;

    loop {
        let instruction = get_value(&tape, cursor, 1);
        cursor += 1;

        let opcode = instruction % 100;
        let modes = instruction / 100;

        // (input_params, output_params)
        let parameters = match opcode {
            1 | 2 | 7 | 8 => (2, 1),
            5 | 6 => (2, 0),
            3 => (0, 1),
            4 => (1, 0),
            99 | _ => (0, 0)
        };

        let values = get_values(&tape, cursor, parameters, modes);
        cursor += parameters.0 + parameters.1;

        match opcode {
            1 => {
                let target = convert_pointer(values[2]);
                tape[target] = values[0] + values[1]
            },
            2 => {
                let target = convert_pointer(values[2]);
                tape[target] = values[0] * values[1]
            },
            3 => {
                let target = convert_pointer(values[0]);
                tape[target] = input[input_index];
                input_index += 1
            },
            4 => {
                output.push(values[0])
            },
            5 => {
                if values[0] != 0 {
                    cursor = values[1]
                }
            },
            6 => {
                if values[0] == 0 {
                    cursor = values[1]
                }
            },
            7 => {
                let target = convert_pointer(values[2]);
                tape[target] = (values[0] < values[1]) as i32
            },
            8 => {
                let target = convert_pointer(values[2]);
                tape[target] = (values[0] == values[1]) as i32
            },
            99 => break,
            _ => panic!("unkown opcode")
        }
    }

    output
}
