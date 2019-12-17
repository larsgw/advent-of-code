use std::fs;

fn get_multiplier (index: usize, factor: usize) -> i32 {
    static MULTIPLIERS: [i32; 4] = [0, 1, 0, -1];
    let index = ((index + 1) / (factor + 1)) % 4;
    MULTIPLIERS[index]
}

fn phase_signal (input: &Vec<u32>) -> Vec<u32> {
    let mut output = Vec::new();
    for i in 0..input.len() {
        let sum: i32 = input
            .iter()
            .enumerate()
            .map(|(j, &digit)| (digit as i32) * get_multiplier(j, i))
            .sum();
        output.push(sum.abs() as u32 % 10)
    }
    output
}

fn star_16_1 (input: &Vec<u32>, n: usize) -> String {
    let mut output = input.to_vec();
    for _ in 0..n {
        output = phase_signal(&output)
    }
    output[0..8].iter().map(|digit| digit.to_string()).collect::<Vec<String>>().join("")
}

fn parse_number (char: char) -> u32 {
	char.to_digit(10).expect("cannot parse char as int")
}

fn main () {
	let file = fs::read_to_string("./input.txt").expect("unable to download file");
	let data = file.trim().chars().map(parse_number).collect::<Vec<u32>>();

	println!(
		"star 16-1: {}
star 16-2: ",
        star_16_1(&data, 100)
	);
}
