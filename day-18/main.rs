use std::fs;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Right,
    Left,
    Down
}

static DIRECTIONS: [Direction; 4] = [Direction::Up, Direction::Right, Direction::Down, Direction:: Left];

impl Direction {
    fn turn_around (&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right
        }
    }

    fn move_position (&self, pos: &Position) -> Position {
        match self {
            Direction::Up => Position(pos.0, pos.1 - 1),
            Direction::Right => Position(pos.0 + 1, pos.1),
            Direction::Down => Position(pos.0, pos.1 + 1),
            Direction::Left => Position(pos.0 - 1, pos.1)
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
struct Position (u64, u64);

struct Maze {
    paths: HashSet<Position>,
    objects: HashMap<Position, char>,
    start: Position,
    keys: usize
}

impl From<String> for Maze {
    fn from (string: String) -> Self {
        let mut paths = HashSet::new();
        let mut objects = HashMap::new();
        let mut start = Position(0, 0);
        let mut keys = 0;
        let mut x = 1;
        let mut y = 1;

        for char in string.chars() {
            let pos = Position(x, y);
            match char {
                '#' => {},
                '\n' => { x = 1; y += 1; },
                '.' => {
                    paths.insert(pos);
                },
                '@' => {
                    paths.insert(pos);
                    start = pos;
                },
                _ => {
                    paths.insert(pos);

                    objects.insert(pos, char);
                    if char.is_ascii_lowercase() {
                        keys += 1;
                    }
                }
            }

            if char != '\n' {
                x += 1;
            }
        }

        Maze { paths, objects, start, keys }
    }
}

#[derive(Debug)]
struct Searcher {
    pos: Position,
    dir: Option<Direction>,
    dist: usize,
    keys: Vec<char>
}

fn solve_maze (maze: &Maze) -> Option<usize> {
    let mut searchers = VecDeque::new();

    searchers.push_back(Searcher {
        pos: maze.start,
        dir: None,
        dist: 0,
        keys: Vec::new()
    });

    println!("");

    loop {
        if searchers.is_empty() {
            return None
        }

        let searcher = searchers.pop_front().unwrap();

        if searcher.keys.len() == maze.keys {
            return Some(searcher.dist)
        } else {
            for dir in &DIRECTIONS {

                if searcher.dir.is_some() && *dir == searcher.dir.unwrap().turn_around() {
                    continue
                }

                let new_pos = dir.move_position(&searcher.pos);
                let mut new_dir = Some(*dir);
                let mut new_keys = searcher.keys.to_vec();

                if !maze.paths.contains(&new_pos) {
                    continue
                }

                if maze.objects.contains_key(&new_pos) {
                    let object = maze.objects[&new_pos];
                    println!("\x1B[1A{: >5} searchers {: >4} dist {: >2} keys on {}   ", searchers.len() + 1, searcher.dist, searcher.keys.len(), object);

                    if object.is_ascii_lowercase() && !new_keys.contains(&object) {
                        new_keys.push(object);
                        new_dir = None;
                    } else if object.is_ascii_uppercase() && !new_keys.contains(&object.to_ascii_lowercase()) {
                        continue
                    }
                }

                searchers.push_back(Searcher {
                    pos: new_pos,
                    dir: new_dir,
                    dist: searcher.dist + 1,
                    keys: new_keys
                });
            }
        }
    }
}

fn main () {
	let file = fs::read_to_string("./input.txt").expect("unable to download file");
	let data = Maze::from(file);

	println!("star 18-1: {:?}", solve_maze(&data));
    // println!("star 18-2: {:?}", solve_maze(&data, true));
}
