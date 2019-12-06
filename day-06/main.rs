use std::fs;
use std::collections::HashMap;
use std::cmp::max;

fn make_chain <'a> (orbits: &'a HashMap<String, String>, satellite: &'a str) -> Vec<&'a str> {
    let center = orbits.get(satellite).expect("orbit not found");
    let mut chain = if center == "COM" {
        Vec::new()
    } else {
        make_chain(orbits, center)
    };
    chain.push(satellite);
    chain
}

fn calculate_orbit_checksum (orbits: &HashMap<String, String>) -> usize {
    let mut sum = 0;
    for satellite in orbits.keys() {
        let orbit_chain = make_chain(orbits, satellite);
        sum += orbit_chain.len()
    }
    sum
}

fn calculate_minimal_transfers (orbits: &HashMap<String, String>, a: String, b: String) -> usize {
    let chain_a = make_chain(&orbits, &a);
    let chain_b = make_chain(&orbits, &b);
    let len_a = chain_a.len();
    let len_b = chain_b.len();
    let max_index = max(len_a, len_b) - 1;

    for i in 0..max_index {
        if chain_a[i] != chain_b[i] {
            return len_a + len_b - 2 * i - 2;
        }
    }

    0
}

fn parse_orbit (source: &str) -> (String, String) {
    let orbit: Vec<&str> = source.split(')').collect();
    (orbit[1].to_string(), orbit[0].to_string())
}

fn main () {
	let file = fs::read_to_string("./input.txt").expect("unable to download file");
	let data: HashMap<String, String> = file.trim().split('\n').map(parse_orbit).collect();

	println!(
		"star 6-1: {}
star 6-2: {}",
		calculate_orbit_checksum(&data),
        calculate_minimal_transfers(&data, String::from("YOU"), String::from("SAN"))
	);
}
