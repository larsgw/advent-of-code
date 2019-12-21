use std::fs;
mod intcode;

fn string_to_intcode (string: &str) -> Vec<i64> {
    string.bytes().map(|byte| byte as i64).collect::<Vec<i64>>()
}

fn intcode_to_string (output: &Vec<i64>) -> String {
    output.iter().map(|&byte| byte as u8 as char).collect::<String>()
}

fn star_20_1 (tape: &Vec<i64>) -> Option<i64> {
    let input = string_to_intcode("NOT T T
AND A T
AND B T
AND C T
NOT T J
AND D J
WALK
");
    let output = intcode::run(tape, &input);
    let last = output[output.len() - 1];

    println!("{}", intcode_to_string(&output));

    if last < 256 {
        None
    } else {
        Some(last)
    }
}

fn star_20_2 (tape: &Vec<i64>) -> Option<i64> {
    let input = string_to_intcode("NOT T J
AND A J
AND B J
AND C J
NOT J J
AND D J
OR I T
OR F T
AND E T
OR H T
AND T J
RUN
");
    let output = intcode::run(tape, &input);
    let last = output[output.len() - 1];

    println!("{}", intcode_to_string(&output));

    if last < 128 {
        None
    } else {
        Some(last)
    }
}

fn parse_number (string: &str) -> i64 {
	string.parse::<i64>().expect("cannot parse string as int")
}

fn main () {
	let file = fs::read_to_string("./input.txt").expect("unable to download file");
	let data = file.trim().split(",").map(parse_number).collect::<Vec<i64>>();

	println!("star 20-1: {:?}", star_20_1(&data));
    println!("star 20-2: {:?}", star_20_2(&data));
}
