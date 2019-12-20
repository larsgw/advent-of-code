use std::fs;
mod intcode;

struct Map <'a> {
    tape: &'a Vec<i64>,
    y: usize
}

impl <'a> Map <'a> {
    fn new (tape: &'a Vec<i64>) -> Self {
        Map {
            tape,
            y: 0
        }
    }
}

impl Iterator for Map <'_> {
    type Item = (usize, usize);

    fn next (&mut self) -> Option<(usize, usize)> {
        let mut offset = None;
        let mut width = 0;

        let mut x = 0;
        loop {
            let coord = intcode::run(
                self.tape,
                &vec![x as i64, self.y as i64]
            ).pop().unwrap_or(0);
            x += 1;

            if coord == 1 {
                width += 1;
                if offset.is_none() {
                    offset = Some(x);
                }
            } else if x >= self.y * 5 || offset.is_some() {
                break
            }
        }

        println!("\x1B[1AChecking {} {:?} {}", self.y, offset, width);
        self.y += 1;
        Some((offset.unwrap_or(0), width))
    }
}

fn star_19_1 (tape: &Vec<i64>) -> usize {
    println!("");
    Map::new(tape).take(50).map(|(_, width)| width).sum()
}

fn star_19_2 (tape: &Vec<i64>) -> Option<usize> {
    static SIZE: usize = 100;
    let iter = Map::new(tape).enumerate();
    let mut map = Vec::<(usize, usize)>::new();

    for (y, end) in iter {
        if end.1 >= SIZE && y >= SIZE {
            let start = map[y - SIZE];
            if (start.0 + start.1 - end.0) >= SIZE {
                return Some(start.0 * 10000 + y)
            }
        }

        map.push(end);
    }

    None
}

fn parse_number (string: &str) -> i64 {
	string.parse::<i64>().expect("cannot parse string as int")
}

fn main () {
	let file = fs::read_to_string("./input.txt").expect("unable to download file");
	let data = file.trim().split(",").map(parse_number).collect::<Vec<i64>>();

	println!("star 19-1: {}", star_19_1(&data));
    println!("star 19-2: {:?}", star_19_2(&data));
}
