use std::fs;
mod intcode;

fn permutate (elements: Vec<i64>) -> Vec<Vec<i64>> {
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

fn calculate_output (code: &Vec<i64>, settings: &Vec<i64>) -> i64 {
    let mut output = 0;

    for phase in settings {
        output = intcode::run(code, &vec![phase.to_owned().to_owned(), output])[0];
    }

    output
}

fn find_optimal_settings (code: &Vec<i64>) -> i64 {
    let possible_phases = (0..5).collect();
    let possible_settings = permutate(possible_phases);
    possible_settings.iter().map(|settings| calculate_output(code, settings)).max().unwrap()
}

fn calculate_feedback_loop_output (code: &Vec<i64>, settings: &Vec<i64>) -> i64 {
    let mut states = Vec::new();

    for phase_setting in settings {
        let tape = code.to_owned();
        let input = vec![*phase_setting];
        states.push(intcode::ProgramState {
            tape,
            input,
            tape_index: 0,
            input_index: 0,
            relative_base: 0
        })
    }

    let mut i = 0;
    let loop_size = states.len();
    let mut last_output = 0;
    loop {
        let state = &mut states[i];
        state.input.push(last_output);

        let output = intcode::step(state);

        if output.is_some() {
            last_output = output.unwrap()
        } else {
            return last_output
        }

        i = (i + 1) % loop_size;
    }
}

fn find_optimal_feedback_loop_settings (code: &Vec<i64>) -> i64 {
    let possible_phases = (5..10).collect();
    let possible_settings = permutate(possible_phases);
    possible_settings.iter().map(|settings| calculate_feedback_loop_output(code, settings)).max().unwrap()
}

fn parse_number (string: &str) -> i64 {
	string.parse::<i64>().expect("cannot parse string as int")
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
