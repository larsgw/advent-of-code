use std::fs;
use std::collections::HashMap;
use std::cmp;
mod intcode;

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Left,
    Down
}

impl Direction {
    fn from (arrow: &i64) -> Option<Self> {
        match *arrow {
            60 => Some(Direction::Left),
            62 => Some(Direction::Right),
            94 => Some(Direction::Up),
            118 => Some(Direction::Down),
            _ => None
        }
    }

    fn turn_left (&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down
        }
    }

    fn turn_right (&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up
        }
    }

    fn move_coords (&self, pos: &(i64, i64)) -> (i64, i64) {
        match self {
            Direction::Up => (pos.0, pos.1 - 1),
            Direction::Right => (pos.0 + 1, pos.1),
            Direction::Down => (pos.0, pos.1 + 1),
            Direction::Left => (pos.0 - 1, pos.1)
        }
    }
}

fn read_camera (code: &Vec<i64>, input: &Vec<i64>) -> HashMap<(i64, i64), i64> {
    let mut picture = HashMap::new();
    let chars = intcode::run(code, input);

    let mut x = 0;
    let mut y = 0;
    for int in chars {
        match int {
            10 => {
                y += 1;
                x = 0;
            },
            _ => {
                picture.insert((x, y), int);
                x += 1;
            }
        }
    }

    picture
}

fn draw_picture (picture: &HashMap<(i64, i64), i64>) {
    let mut string = String::new();

    let mut max = (0, 0);
    for (x, y) in picture.keys() {
        max.0 = cmp::max(*x, max.0);
        max.1 = cmp::max(*y, max.1);
    }

    for y in 0..=max.1 {
        for x in 0..=max.0 {
            let char = *picture.get(&(x, y)).unwrap_or(&20) as u8 as char;
            string.push(char);
            string.push(char);
        }
        string += "\n";
    }

    println!("{}", string);
}

fn is_scaffold (picture: &HashMap<(i64, i64), i64>, pos: &(i64, i64)) -> bool {
    let code = picture.get(pos);

    if code.is_none() {
        // edge of the map
        return false
    }

    match *code.unwrap() {
        35 | 60 | 62 | 94 | 118 => true,
        46 | 88 | _ => false
    }
}

fn is_intersection (picture: &HashMap<(i64, i64), i64>, pos: &(i64, i64)) -> bool {
    is_scaffold(picture, pos) &&
    is_scaffold(picture, &Direction::Up.move_coords(pos)) &&
    is_scaffold(picture, &Direction::Right.move_coords(pos)) &&
    is_scaffold(picture, &Direction::Down.move_coords(pos)) &&
    is_scaffold(picture, &Direction::Left.move_coords(pos))
}

fn star_17_1 (code: &Vec<i64>) -> i64 {
    let picture = read_camera(code, &Vec::new());
    let mut align_params = 0;

    for pos in picture.keys() {
        if is_intersection(&picture, &pos) {
            align_params += pos.0 * pos.1
        }
    }

    align_params
}

fn find_path (picture: &HashMap<(i64, i64), i64>) -> Vec<String> {
    let mut pos = (0, 0);
    let mut direction = Direction::Up;

    for (start_pos, code) in picture.iter() {
        let start_direction = Direction::from(code);
        if start_direction.is_some() {
            direction = start_direction.unwrap();
            pos = *start_pos;
        }
    }

    let mut instructions = Vec::new();
    let mut straight_moves = 0;
    loop {
        let new_pos = direction.move_coords(&pos);
        if is_scaffold(picture, &new_pos) {
            pos = new_pos;
            straight_moves += 1;
        } else {
            if straight_moves > 0 {
                instructions.push(straight_moves.to_string());
                straight_moves = 0;
            }

            let left = direction.turn_left();
            let right = direction.turn_right();
            if is_scaffold(picture, &left.move_coords(&pos)) {
                direction = left;
                instructions.push(String::from("L"));
            } else if is_scaffold(picture, &right.move_coords(&pos)) {
                direction = right;
                instructions.push(String::from("R"));
            } else {
                break
            }
        }
    }

    instructions
}

fn star_17_2 (code: &Vec<i64>) -> i64 {
    let picture = read_camera(code, &Vec::new());
    draw_picture(&picture);
    let path = find_path(&picture);

    // TODO

    let mut tape = code.to_vec();
    tape[0] = 2;

    let instructions = format!(
        "{}\n{}\n{}\n{}\n{}\n",
        "A,B,A,C,B,C,B,A,C,B",
        "L,10,L,6,R,10",
        "R,6,R,8,R,8,L,6,R,8",
        "L,10,R,8,R,8,L,10",
        "n"
    )
        .chars()
        .map(|char| char as i64)
        .collect::<Vec<i64>>();

    intcode::run(&tape, &instructions).pop().unwrap()
}

fn parse_number (string: &str) -> i64 {
	string.parse::<i64>().expect("cannot parse string as int")
}

fn main () {
	let file = fs::read_to_string("./input.txt").expect("unable to download file");
	let data = file.trim().split(",").map(parse_number).collect::<Vec<i64>>();

	println!(
		"star 17-1: {}
star 17-2: {}",
        star_17_1(&data),
        star_17_2(&data)
	);
}
