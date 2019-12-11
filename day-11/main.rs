use std::fs;
use std::collections::HashMap;
use std::cmp;
mod intcode;

enum Color {
    White,
    Black
}

impl From<i64> for Color {
    fn from (number: i64) -> Self {
        match number {
            0 => Color::Black,
            1 => Color::White,
            _ => panic!("unknown color")
        }
    }
}

impl Into<i64> for &Color {
    fn into (self) -> i64 {
        match self {
            Color::Black => 0,
            Color::White => 1
        }
    }
}

enum Direction {
    Up,
    Right,
    Down,
    Left
}

fn turn_left (direction: &Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Left,
        Direction::Right => Direction::Up,
        Direction::Down => Direction::Right,
        Direction::Left => Direction::Down
    }
}

fn turn_right (direction: &Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up
    }
}

fn move_robot (position: &mut (i64, i64), direction: &Direction) {
    match direction {
        Direction::Up => position.1 -= 1,
        Direction::Right => position.0 += 1,
        Direction::Down => position.1 += 1,
        Direction::Left => position.0 -= 1,
    }
}

fn paint_panels (intcode: &Vec<i64>, start_color: Color) -> HashMap<(i64, i64), Color> {
    let mut panels = HashMap::new();
    let mut position = (0, 0);
    let mut direction = Direction::Up;

    let mut state = intcode::ProgramState {
        tape: intcode.to_vec(),
        input: Vec::new(),
        tape_index: 0,
        input_index: 0,
        relative_base: 0
    };

    panels.insert(position, start_color);

    loop {
        let panel = panels.get(&position).unwrap_or(&Color::Black);
        state.input.push(panel.into());
        let color = intcode::step(&mut state);
        let turn = intcode::step(&mut state);

        if color.is_none() {
            return panels
        }

        panels.insert(position, Color::from(color.unwrap()));
        direction = if turn.unwrap() == 1 {
            turn_right(&direction)
        } else {
            turn_left(&direction)
        };
        move_robot(&mut position, &direction);
    }
}

fn count_painted_panels (intcode: &Vec<i64>) -> usize {
    let panels = paint_panels(intcode, Color::Black);
    panels.len()
}

fn display_panels (intcode: &Vec<i64>) -> String {
    let panels = paint_panels(intcode, Color::White);
    let mut min = (0, 0);
    let mut max = (0, 0);

    for point in panels.keys() {
        min.0 = cmp::min(min.0, point.0);
        min.1 = cmp::min(min.1, point.1);
        max.0 = cmp::max(max.0, point.0);
        max.1 = cmp::max(max.1, point.1);
    }

    let mut display = String::new();

    for y in min.1..=max.1 {
        for x in min.0..=max.0 {
            let color = panels.get(&(x, y)).unwrap_or(&Color::Black);
            match color {
                Color::White => display += "█",
                Color::Black => display += "░"
            }
        }
        display += "\n"
    }

    display
}

fn parse_number (string: &str) -> i64 {
	string.parse::<i64>().expect("cannot parse string as int")
}

fn main () {
	let file = fs::read_to_string("./input.txt").expect("unable to download file");
	let data = file.trim().split(",").map(parse_number).collect();

    println!(
		"star 11-1: {:?}
star 11-2:
{}",
		count_painted_panels(&data),
        display_panels(&data)
	);
}
