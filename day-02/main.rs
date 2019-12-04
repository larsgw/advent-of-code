use std::fs;

fn check_pointer (pointer: i32) -> Option<usize> {
    if pointer >= 0 {
        Some(pointer as usize)
    } else {
        None
    }
}

fn convert_pointer (pointer: i32) -> usize {
    check_pointer(pointer).expect(&format!("pointer out of bounds: {}", pointer))
}

fn get_value (tape: &Vec<i32>, pointer: i32) -> i32 {
    tape[convert_pointer(pointer)]
}

fn deref_pointer (tape: &Vec<i32>, pointer_pointer: i32) -> i32 {
    let pointer = get_value(tape, pointer_pointer);
    get_value(tape, pointer)
}

fn run_intcode (intcode: &Vec<i32>, noun: i32, verb: i32) -> i32 {
    let mut tape = intcode.to_vec();
    tape[1] = noun;
    tape[2] = verb;
    let mut cursor = 0;

    loop {
        let opcode = get_value(&tape, cursor);
        if opcode == 99 {
            break
        }

        let a = deref_pointer(&tape, cursor + 1);
        let b = deref_pointer(&tape, cursor + 2);
        let result = convert_pointer(get_value(&tape, cursor + 3));

        match opcode {
            1 => tape[result] = a + b,
            2 => tape[result] = a * b,
            _ => panic!("unkown opcode")
        }

        cursor += 4
    }

    tape[0]
}

fn simulate_noun_verb (intcode: &Vec<i32>, target: i32) -> Option<i32> {
    for noun in 0..99 {
        for verb in 0..99 {
            if run_intcode(intcode, noun, verb) == target {
                return Some(100 * noun + verb)
            }
        }
    }
    None
}

fn parse_number (string: &str) -> i32 {
	string.parse::<i32>().expect("cannot parse string as int")
}

fn main () {
	let file = fs::read_to_string("./input.txt").expect("unable to download file");
	let data = file.trim().split(",").map(parse_number).collect();

	println!(
		"star 2-1: {}
star 2-2: {}",
		run_intcode(&data, 12, 2),
        simulate_noun_verb(&data, 19690720).expect("no solution found")
	);
}
