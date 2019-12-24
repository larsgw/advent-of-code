use std::fs;
use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug, Eq, PartialEq)]
enum State {
    Dead,
    Alive
}

#[derive(Default)]
struct Automaton {
    points: HashMap<(u64, u64), State>,
    size: u64,
    checksums: HashSet<u64>,
    recursive: false
}

impl From<&str> for Automaton {
    fn from (map: &str) -> Self {
        let mut points = HashMap::new();

        let mut x = 0;
        let mut y = 0;
        for char in map.chars() {
            match char {
                '.' => {
                    points.insert((x, y), State::Dead);
                    x += 1;
                },
                '#' => {
                    points.insert((x, y), State::Alive);
                    x += 1;
                },
                '\n' => {
                    x = 0;
                    y += 1;
                },
                _ => {
                    unreachable!();
                }
            }
        }

        Automaton {
            points,
            size: y + 1,
            ..Default::default()
        }
    }
}

impl fmt::Display for Automaton {
    fn fmt (&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut display = String::new();
        for y in 0..self.size {
            for x in 0..self.size {
                display.push(match self.points[&(x, y)] {
                    State::Alive => '#',
                    State::Dead => '.'
                })
            }
            if y == 0 {
                display += &format!(" {}", self.get_checksum());
            }
            display.push('\n');
        }
        write!(f, "{}", display)
    }
}

impl Automaton {
    fn get_checksum (&self) -> u64 {
        let mut sum = 0;
        for ((x, y), state) in &self.points {
            if *state == State::Alive {
                sum += u64::pow(2, ((y * self.size) + x) as u32);
            }
        }
        return sum
    }

    fn get_surrounding_live_count (&self, (x, y): &(u64, u64)) -> u64 {
        let count = vec![
            *x != 0 && self.points[&(x - 1, *y)] == State::Alive,
            *x != self.size - 1 && self.points[&(x + 1, *y)] == State::Alive,
            *y != 0 && self.points[&(*x, y - 1)] == State::Alive,
            *y != self.size - 1 && self.points[&(*x, y + 1)] == State::Alive
        ];

        count.iter().map(|&n| n as u64).sum()
    }

    fn step (&mut self) {
        let mut changes = Vec::new();

        for (position, state) in &self.points {
            let count = self.get_surrounding_live_count(position);
            if *state == State::Alive && count != 1 {
                changes.push((*position, State::Dead))
            } else if *state == State::Dead && (count == 1 || count == 2) {
                changes.push((*position, State::Alive))
            }
        }

        for (position, state) in changes {
            self.points.insert(position, state);
        }
    }

    fn step_until_repeat (&mut self) -> u64 {
        loop {
            let checksum = self.get_checksum();
            if !self.checksums.insert(checksum) {
                return checksum;
            }

            self.step();
        }
    }
}

fn main () {
    let file = fs::read_to_string("./input.txt").expect("unable to download file").trim();

    let mut normal = Automaton::from(file);
    println!("star 22-1: {}", normal.step_until_repeat());
}
