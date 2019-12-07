use std::fs;
mod intcode;

fn permutate (elements: Vec<i32>) -> Vec<Vec<i32>> {
    let mut permutations = Vec::new();
    for element in &elements {
        let mut other_elements = (&elements).to_vec();
        other_elements.retain(|x| x != element);
        if other_elements.is_empty() {
            permutations.push(vec![*element])
        } else {
            for mut permutation in permutate(other_elements) {
                permutation.push(*element);
                permutations.push(permutation.to_vec())
            }
        }
    }
    permutations
}

fn calculate_output (code: &Vec<i32>, settings: &Vec<i32>) -> i32 {
    let mut output = 0;

    for phase in settings {
        output = intcode::run(code, &vec![phase.to_owned().to_owned(), output])[0];
    }

    output
}

fn find_optimal_settings (code: &Vec<i32>) -> i32 {
    let possible_phases = (0..5).collect();
    let possible_settings = permutate(possible_phases);
    possible_settings.iter().map(|settings| calculate_output(code, settings)).max().unwrap()
}

struct State {
    tape: Vec<i32>,
    input: Vec<i32>,
    tape_index: i32,
    input_index: usize
}

fn calculate_feedback_loop_output (code: &Vec<i32>, settings: &Vec<i32>) -> i32 {
    let mut states = Vec::new();

    for phase_setting in settings {
        let tape = code.to_owned();
        let input = vec![*phase_setting];
        states.push(State {
            tape,
            input,
            tape_index: 0,
            input_index: 0
        })
    }

    let mut i = 0;
    let loop_size = states.len();
    let mut last_output = 0;
    loop {
        let state = &mut states[i];
        state.input.push(last_output);

        let output = intcode::step(
            &mut state.tape,
            &mut state.input,
            &mut state.tape_index,
            &mut state.input_index
        );

        if output.is_some() {
            last_output = output.unwrap()
        } else {
            return last_output
        }

        i = (i + 1) % loop_size;
    }
}

fn find_optimal_feedback_loop_settings (code: &Vec<i32>) -> i32 {
    let possible_phases = (5..10).collect();
    let possible_settings = permutate(possible_phases);
    possible_settings.iter().map(|settings| calculate_feedback_loop_output(code, settings)).max().unwrap()
}

fn parse_number (string: &str) -> i32 {
	string.parse::<i32>().expect("cannot parse string as int")
}

fn main () {
	let file = fs::read_to_string("./input.txt").expect("unable to download file");
	let data = file.trim().split(",").map(parse_number).collect();

	println!(
		"star 7-1: {:?}
star 7-2: {:?}",
		find_optimal_settings(&data),
        find_optimal_feedback_loop_settings(&data)
	);
}
