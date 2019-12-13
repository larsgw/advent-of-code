use std::fs;
mod intcode;

fn count_block_tiles (tape: &Vec<i64>) -> i64 {
    let output = intcode::run(tape, &vec![]);
    let tiles = output.chunks(3);
    let mut sum = 0;
    for tile in tiles {
        if tile[2] == 2 {
            sum += 1;
        }
    }
    sum
}

fn parse_number (string: &str) -> i64 {
	string.parse::<i64>().expect("cannot parse string as int")
}

fn main () {
	let file = fs::read_to_string("./input.txt").expect("unable to download file");
	let data = file.trim().split(",").map(parse_number).collect();

	println!(
		"star 13-1: {}",
		count_block_tiles(&data)
	);
}
