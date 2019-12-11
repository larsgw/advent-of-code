use std::fs;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::collections::VecDeque;

fn get_coords (map: &Vec<char>) -> Vec<(i32, i32)> {
    let mut points = Vec::new();
    let mut x = 0;
    let mut y = 0;

    for char in map {
        match char {
            '.' => {
                x += 1;
            },
            '#' => {
                points.push((x, y));
                x += 1;
            },
            '\n' => {
                x = 0;
                y += 1;
            },
            _ => panic!("unknown char on map")
        }
    }

    points
}

#[derive(Eq, PartialEq, Hash, Debug)]
struct LineOfSight(i32, i32);

impl LineOfSight {
    fn reduce (&self) -> LineOfSight {
        let factor = gcd(self.0.abs(), self.1.abs());
        LineOfSight(self.0 / factor, self.1 / factor)
    }

    fn from (point: &(i32, i32), base: &(i32, i32)) -> Self {
        let x = point.0 - base.0;
        let y = point.1 - base.1;
        LineOfSight(x, y)
    }
}

impl PartialOrd for LineOfSight {
    fn partial_cmp (&self, other: &Self) -> Option<Ordering> {
        if self.reduce() == other.reduce() {
            Some(get_distance(self).cmp(&get_distance(other)))
        } else {
            let a = get_angle(self);
            let b = get_angle(other);
            a.partial_cmp(&b)
        }
    }
}

fn get_angle (line: &LineOfSight) -> f32 {
    if line.0 == 0 && line.1 < 0 {
        -std::f32::consts::PI
    } else {
        (-line.0 as f32).atan2(line.1 as f32)
    }
}

fn get_distance (line: &LineOfSight) -> i32 {
    line.0.abs() + line.1.abs()
}

fn gcd (a: i32, b: i32) -> i32 {
    if a == 0 && b == 0 {
        1
    } else if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn calculate_base_value (base: &(i32, i32), points: &Vec<(i32, i32)>) -> usize {
    let mut lines_of_sight = HashSet::new();
    for point in points {
        if point != base {
            lines_of_sight.insert(LineOfSight::from(&point, &base).reduce());
        }
    }
    lines_of_sight.len()
}

fn find_optimal_base (points: &Vec<(i32, i32)>) -> (i32, i32) {
    points
        .iter()
        .max_by_key(|&point| calculate_base_value(point, &points))
        .unwrap()
        .to_owned()
}

fn find_nth_laser_asteroid (points: &Vec<(i32, i32)>, base: &(i32, i32), n: usize) -> i32 {
    let mut sorted = points
        .to_vec()
        .iter()
        .map(|point| LineOfSight::from(point, base))
        .collect::<Vec<LineOfSight>>();
    let base_index = sorted.iter().position(|line| line.0 == 0 && line.1 == 0).unwrap();
    sorted.remove(base_index);
    sorted.sort_by(|a, b| a.partial_cmp(&b).unwrap());

    let mut queue = VecDeque::from(sorted);
    let mut previous = LineOfSight(0, 0);
    let mut destroyed_counter = 0;
    let mut queue_counter = 0;

    loop {
        if queue_counter == 0 {
            previous = LineOfSight(0, 0);
            queue_counter = queue.len()
        }

        let line = queue.pop_front().unwrap();
        queue_counter -= 1;
        if line.reduce() == previous.reduce() {
            queue.push_back(line);
        } else {
            let point = (line.0 + base.0, line.1 + base.1);
            destroyed_counter += 1;

            if destroyed_counter == n {
                return point.0 * 100 + point.1
            }

            previous = line;
        }
    }
}

fn main () {
	let file = fs::read_to_string("./input.txt").expect("unable to download file");
	let data = file.chars().collect();

    let points = get_coords(&data);
    let base = find_optimal_base(&points);

    println!(
		"star 10-1: {}
star 10-2: {}",
        calculate_base_value(&base, &points),
		find_nth_laser_asteroid(&points, &base, 200)
	);
}
