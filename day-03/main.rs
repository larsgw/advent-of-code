use std::fs;
use std::collections::HashSet;

// Thanks to Egon Willighagen (@egonw) for thinking of implementing this as sets of points

fn make_set_from_path (wire: &Vec<(i32, i32)>) -> HashSet<&(i32, i32)> {
    let mut set = HashSet::new();
    for point in wire {
        set.insert(point);
    }
    set
}

fn find_intersections (wires: &Vec<Vec<(i32, i32)>>) -> Vec<&(i32, i32)> {
    let mut intersections = Vec::new();
    let a = make_set_from_path(&wires[0]);
    let b = make_set_from_path(&wires[1]);

    for point in a.intersection(&b) {
        intersections.push(*point)
    }

    intersections
}

fn get_distance (point: &(i32, i32), _: &Vec<Vec<(i32, i32)>>) -> i32 {
    point.0.abs() + point.1.abs()
}

fn get_index_of (vector: &Vec<(i32, i32)>, element: &(i32, i32)) -> Option<usize> {
    for i in 0..vector.len() {
        if vector[i] == *element {
            return Some(i)
        }
    }
    None
}

fn get_timing (point: &(i32, i32), paths: &Vec<Vec<(i32, i32)>>) -> i32 {
    paths.iter().map(|path| get_index_of(path, point).unwrap() as i32 + 1).sum()
}

fn find_specific_intersection (wires: &Vec<Vec<(i32, i32)>>, comparator: &dyn Fn(&(i32, i32), &Vec<Vec<(i32, i32)>>) -> i32) -> i32 {
    let intersections = find_intersections(wires);
    let point = intersections.iter().min_by(|&a, &b| comparator(a, wires).cmp(&comparator(b, wires))).unwrap();

    comparator(point, wires)
}

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left
}

#[derive(Debug)]
struct PathPart {
    direction: Direction,
    distance: i32
}

impl From<&str> for PathPart {
    fn from (string: &str) -> Self {
        let parts = string.split_at(1);
        let direction = match parts.0 {
            "U" => Direction::Up,
            "R" => Direction::Right,
            "D" => Direction::Down,
            "L" => Direction::Left,
            _ => panic!("unknown direction")
        };
        PathPart {
            direction,
            distance: parts.1.parse::<i32>().expect("cannot parse string as int")
        }
    }
}

#[derive(Debug)]
struct Path {
    parts: Vec<PathPart>,
    part_index: i32,
    part_distance: i32,
    x: i32,
    y: i32
}

impl Iterator for Path {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<(i32, i32)> {
        if self.part_index as usize >= self.parts.len() {
            return None
        }

        let part = &self.parts[self.part_index as usize];
        match part.direction {
            Direction::Up => self.y -= 1,
            Direction::Right => self.x += 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1
        };

        if self.part_distance == part.distance - 1 {
            self.part_distance = 0;
            self.part_index += 1;
        } else {
            self.part_distance += 1;
        }

        Some((self.x, self.y))
    }
}

impl From<&str> for Path {
    fn from (string: &str) -> Self {
        let parts = string.split(',').map(|x| PathPart::from(x)).collect();
        Path {
            parts: parts,
            part_index: 0,
            part_distance: 0,
            x: 0,
            y: 0
        }
    }
}

fn main () {
	let file = fs::read_to_string("./input.txt").expect("unable to download file");
    let data: Vec<Vec<(i32, i32)>> = file.trim().split('\n').map(|x| Path::from(x).collect()).collect();

	println!(
		"star 3-1: {}
star 3-2: {}",
		find_specific_intersection(&data, &get_distance),
		find_specific_intersection(&data, &get_timing),
	);
}
