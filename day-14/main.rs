use std::fs;
use std::collections::HashMap;

#[derive(Debug)]
struct Chemical <'a> {
    name: &'a str,
    quantity: u64
}

impl <'a> From<&'a str> for Chemical<'a> {
    fn from (string: &'a str) -> Self {
        let parts = string.split(" ").collect::<Vec<&str>>();
        Chemical {
            quantity: parts[0].parse::<u64>().expect("cannot parse integer"),
            name: parts[1]
        }
    }
}

struct Reaction <'a> {
    input: Vec<Chemical<'a>>,
    output: Chemical<'a>
}

impl <'a> From<&'a str> for Reaction<'a> {
    fn from (string: &'a str) -> Self {
        let parts = string.split(" => ").collect::<Vec<&str>>();
        let input = parts[0].split(", ").map(|chemical| Chemical::from(chemical)).collect();
        Reaction {
            input,
            output: Chemical::from(parts[1])
        }
    }
}

fn calculate_needed_ore <'a> (reactions: &'a HashMap<&str, Reaction>, leftovers: &mut HashMap<&'a str, u64>, product_name: &'a str, quantity: u64) -> u64 {
    if product_name == "ORE" {
        quantity
    } else if leftovers.contains_key(product_name) {
        let leftover = leftovers.remove(product_name).unwrap();
        if quantity > leftover {
            calculate_needed_ore(reactions, leftovers, product_name, quantity - leftover)
        } else if leftover > quantity {
            leftovers.insert(product_name, leftover - quantity);
            0
        } else {
            0
        }
    } else {
        let mut total_ore = 0;
        let reaction = reactions.get(product_name).unwrap();
        let factor = 1 + ((quantity - 1) / reaction.output.quantity);

        for chemical in &reaction.input {
            let ore = calculate_needed_ore(reactions, leftovers, chemical.name, chemical.quantity * factor);
            total_ore += ore;
        }

        let leftover_product = reaction.output.quantity * factor - quantity;
        if leftover_product > 0 {
            *leftovers.entry(product_name).or_insert(0) += leftover_product;
        }

        total_ore
    }
}

fn calculate_maximum_fuel <'a> (reactions: &'a HashMap<&str, Reaction>, maximum_ore: u64) -> u64 {
    let mut lower_bound = 1;
    let mut upper_bound = maximum_ore;

    loop {
        let fuel = (lower_bound + upper_bound) / 2;
        let ore = calculate_needed_ore(reactions, &mut HashMap::new(), "FUEL", fuel);
        if ore < maximum_ore {
            if lower_bound == fuel {
                break
            } else {
                lower_bound = fuel;
            }
        } else {
            upper_bound = fuel
        }
    }

    lower_bound
}

fn main () {
	let file = fs::read_to_string("./input.txt").expect("unable to download file");
	let data = file.trim().split('\n').map(|reaction| Reaction::from(reaction)).collect::<Vec<Reaction>>();
    let mut lookup = HashMap::new();
    for reaction in data {
        lookup.insert(reaction.output.name, reaction);
    }

	println!(
		"star 14-1: {}
star 14-2: {}",
		calculate_needed_ore(&lookup, &mut HashMap::new(), "FUEL", 1),
        calculate_maximum_fuel(&lookup, 1_000_000_000_000)
	);
}
