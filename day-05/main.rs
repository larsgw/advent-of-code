use std::fs;
mod intcode;

fn parse_number (string: &str) -> i64 {
	string.parse::<i64>().expect("cannot parse string as int")
}

fn main () {
	let file = fs::read_to_string("./input.txt").expect("unable to download file");
	let data = file.trim().split(",").map(parse_number).collect();

	println!(
		"star 5-1: {:?}
star 5-2: {:?}",
		intcode::run(&data, &vec![1]),
        intcode::run(&data, &vec![5])
	);
}
