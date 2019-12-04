use std::fs;
use std::cmp;

fn calculate_required_fuel (mass: i32) -> i32 {
	(mass / 3) - 2
}

fn calculate_required_fuel_recursively (mass: i32) -> i32 {
	let mut fuel_mass = 0;
	let mut added_mass = mass;
	while added_mass > 0 {
		added_mass = cmp::max(calculate_required_fuel(added_mass), 0);
		fuel_mass += added_mass
	}

	fuel_mass
}

fn parse_number (string: &str) -> i32 {
	string.parse::<i32>().expect("cannot parse string as int")
}

fn calculate_module_fuel (modules: &Vec<i32>, predicate: &dyn Fn(i32) -> i32) -> i32 {
	modules.iter().map(|&x| predicate(x)).sum()
}

fn main () {
	let file = fs::read_to_string("./input.txt").expect("unable to download file");
	let data = file.trim().split("\n").map(parse_number).collect();

	println!(
		"star 1-1: {}
star 1-2: {}",
		calculate_module_fuel(&data, &calculate_required_fuel),
		calculate_module_fuel(&data, &calculate_required_fuel_recursively)
	);
}
