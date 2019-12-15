use std::fs;
use std::collections::HashMap;
use std::cmp;
mod intcode;

enum Direction {
    North,
    East,
    South,
    West
}

impl Into<i64> for &Direction {
    fn into (self) -> i64 {
        match self {
            Direction::North => 1,
            Direction::East => 4,
            Direction::South => 2,
            Direction::West => 3
        }
    }
}

impl Into<&str> for &Direction {
    fn into (self) -> &'static str {
        match self {
            Direction::North => "^^",
            Direction::East => ">>",
            Direction::South => "vv",
            Direction::West => "<<"
        }
    }
}

fn turn_left (direction: &Direction) -> Direction {
    match direction {
        Direction::North => Direction::West,
        Direction::East => Direction::North,
        Direction::South => Direction::East,
        Direction::West => Direction::South
    }
}

fn turn_right (direction: &Direction) -> Direction {
    match direction {
        Direction::North => Direction::East,
        Direction::East => Direction::South,
        Direction::South => Direction::West,
        Direction::West => Direction::North
    }
}

fn move_drone (pos: &(i64, i64), direction: &Direction) -> (i64, i64) {
    match direction {
        Direction::North => (pos.0, pos.1 - 1),
        Direction::East => (pos.0 + 1, pos.1),
        Direction::South => (pos.0, pos.1 + 1),
        Direction::West => (pos.0 - 1, pos.1)
    }
}

fn find_dimensions (map: &HashMap<(i64, i64), i64>) -> ((i64, i64), (i64, i64)) {
    let mut min = (0, 0);
    let mut max = (0, 0);
    for (x, y) in map.keys() {
        min.0 = cmp::min(*x, min.0);
        min.1 = cmp::min(*y, min.1);
        max.0 = cmp::max(*x, max.0);
        max.1 = cmp::max(*y, max.1);
    }
    (min, max)
}

fn display (map: &HashMap<(i64, i64), i64>, up: bool) {
    let (min, max) = find_dimensions(map);
    let mut display = String::new();
    if up {
        display += "\x1B[";
        display += &((max.1 - min.1 + 2).to_string() + "A");
    }

    for y in min.1..=max.1 {
        for x in min.0..=max.0 {
            if (x, y) == (0, 0) {
                display += "\x1B[32m\x1B[42mSS\x1B[0m"
            } else {
                match map.get(&(x, y)) {
                    Some(0) => display += "\x1B[47m##\x1B[0m",
                    Some(1) => display += "..",
                    Some(2) => display += "\x1B[33m\x1B[43mEE\x1B[0m",
                    Some(3) => display += "\x1B[34m\x1B[44mOO\x1B[0m",
                    None => display += "  ",
                    Some(_) => panic!("unknown status")
                }
            }
        }
        display += "\n";
    }

    println!("{}", display);
}

fn build_map (tape: &Vec<i64>) -> HashMap<(i64, i64), i64> {
    let mut map = HashMap::new();
    let mut state = intcode::ProgramState {
        tape: tape.to_vec(),
        input: Vec::new(),
        tape_index: 0,
        input_index: 0,
        relative_base: 0
    };
    let mut position = (0, 0);
    let mut direction = Direction::North;

    loop {
        state.input.push((&direction).into());
        let checking = move_drone(&position, &direction);
        let ahead = intcode::step(&mut state).unwrap();
        map.insert(checking, ahead);

        match ahead {
            0 => direction = turn_left(&direction),
            1 | 2 => {
                position = checking;

                if position == (0, 0) {
                    break
                }

                let right_direction = turn_right(&direction);
                let right = map.get(&move_drone(&position, &right_direction));
                if right.is_none() || *right.unwrap() == 1 {
                    direction = right_direction;
                }
            },
            _ => panic!("unknown status")
        }
    }

    map
}

fn find_system (map: &HashMap<(i64, i64), i64>) -> (i64, (i64, i64)) {
    let mut moves = HashMap::new();
    let mut move_number = 0;
    let mut position = (0, 0);
    let mut direction = Direction::North;

    loop {
        let checking = move_drone(&position, &direction);

        match map[&checking] {
            0 => direction = turn_left(&direction),
            1 => {
                position = checking;

                if moves.contains_key(&position) {
                    move_number = moves[&position];
                } else {
                    move_number += 1;
                }
                moves.insert(position, move_number);

                let right_direction = turn_right(&direction);
                let right = map.get(&move_drone(&position, &right_direction));
                if right.is_none() || *right.unwrap() == 1 {
                    direction = right_direction;
                }
            },
            2 => return (move_number + 1, checking),
            _ => panic!("unknown status")
        }
    }
}

fn fill_oxygen (map: &mut HashMap<(i64, i64), i64>, system: &(i64, i64)) -> u64 {
    let mut minutes = 0;
    let mut leading_pos = vec![*system];

    loop {
        display(&map, true);

        let mut new = Vec::new();
        for pos in leading_pos {
            static DIRECTIONS: [Direction; 4] = [Direction::North, Direction::West, Direction::East, Direction::South];
            for dir in DIRECTIONS.iter() {
                let next = move_drone(&pos, &dir);
                let check = map.get(&next);
                if check.is_some() && *check.unwrap() == 1 {
                    map.insert(next, 3);
                    new.push(next);
                }
            }
        }

        if new.len() == 0 {
            return minutes
        } else {
            leading_pos = new;
            minutes += 1;
        }
    }
}

fn parse_number (string: &str) -> i64 {
	string.parse::<i64>().expect("cannot parse string as int")
}

fn main () {
	let file = fs::read_to_string("./input.txt").expect("unable to download file");
	let data = file.trim().split(",").map(parse_number).collect();
    let mut map = build_map(&data);
    let (moves, system) = find_system(&map);
    display(&map, false);

	println!(
		"star 15-1: {}
star 15-2: {}",
        moves,
        fill_oxygen(&mut map, &system)
	);
}
