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
    // fn turn_left (&self) -> Self {
    //     match self {
    //         Direction::Up => Direction::Left,
    //         Direction::Right => Direction::Up,
    //         Direction::Down => Direction::Right,
    //         Direction::Left => Direction::Down
    //     }
    // }
    //
    // fn turn_right (&self) -> Self {
    //     match self {
    //         Direction::Up => Direction::Right,
    //         Direction::Right => Direction::Down,
    //         Direction::Down => Direction::Left,
    //         Direction::Left => Direction::Up
    //     }
    // }

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

#[derive(Debug)]
struct Portal {
    pos: Position,
    dir: Direction,
    inner: bool
}

struct Maze {
    portals: HashMap<String, Vec<Portal>>,
    paths: HashSet<Position>,
    markers: HashMap<Position, String>
}

fn find_marker (pos: &Position, letters: &HashMap<Position, char>) -> Option<(String, Direction)> {
    for dir in &DIRECTIONS {
        let a = dir.move_position(pos);
        let b = dir.move_position(&a);

        if letters.contains_key(&a) && letters.contains_key(&b) {
            let chars = vec![letters[&a], letters[&b]];
            let name = match *dir {
                Direction::Left | Direction::Up => chars.iter().rev().collect::<String>(),
                Direction::Down | Direction::Right => chars.iter().collect::<String>()
            };
            return Some((name, dir.turn_around()))
        }
    }

    None
}

fn is_inner (coord: u64, size: u64) -> bool {
    2 < coord && (coord < size - 3)
}

impl From<String> for Maze {
    fn from (string: String) -> Self {
        let mut letters = HashMap::new();
        let mut paths = HashSet::new();
        let mut x = 0;
        let mut y = 0;
        let mut size = (0, 0);

        for char in string.chars() {
            let pos = Position(x, y);
            match char {
                '.' => { paths.insert(pos); },
                '\n' => {
                    size.0 = std::cmp::max(size.0, x);
                    size.1 = y;
                    x = 0;
                    y += 1;
                },
                '#' | ' ' => {},
                _ => { letters.insert(pos, char); }
            }

            if char != '\n' {
                x += 1;
            }
        }

        let mut portals = HashMap::new();
        let mut markers = HashMap::new();

        for path in &paths {
            let marker = find_marker(path, &letters);
            if marker.is_some() {
                let (name, dir) = marker.unwrap();
                let inner = is_inner(path.0, size.0) && is_inner(path.1, size.1);
                portals
                    .entry(name.to_string())
                    .or_insert_with(|| Vec::new())
                    .push(Portal { pos: *path, dir, inner });
                markers.insert(*path, name);
            }
        }

        Maze {
            portals,
            markers,
            paths
        }
    }
}

#[derive(Debug)]
struct Searcher {
    pos: Position,
    dir: Direction,
    dist: usize,
    level: usize
}

impl Searcher {
    fn move_through_portal (&self, target: &Portal, recursive: bool) -> Option<Searcher> {
        // portals on the outer ring (so portals where the target is inner)
        // in the outermost level do not work in recursive mode
        if recursive && self.level == 0 && target.inner {
            return None
        }

        let new_level = if recursive == false {
            self.level
        } else if target.inner {
            self.level - 1
        } else {
            self.level + 1
        };

        Some(Searcher {
            pos: target.pos,
            dir: target.dir,
            dist: self.dist + 1,
            level: new_level
        })
    }
}

fn solve_maze (maze: &Maze, recursive: bool) -> usize {
    let mut searchers = VecDeque::new();

    let start = &maze.portals["AA"][0];
    let end = &maze.portals["ZZ"][0];
    searchers.push_back(Searcher {
        pos: start.pos,
        dir: start.dir,
        dist: 0,
        level: 0
    });

    println!("");

    loop {
        if searchers.is_empty() {
            break
        }

        let searcher = searchers.pop_front().unwrap();

        if searcher.pos == end.pos && searcher.level == 0 {
            return searcher.dist
        } else {
            let mut moved = false;
            for new_dir in &DIRECTIONS {
                let new_pos = new_dir.move_position(&searcher.pos);
                if *new_dir != searcher.dir.turn_around() && maze.paths.contains(&new_pos) {
                    moved = true;
                    searchers.push_back(Searcher {
                        pos: new_pos,
                        dir: *new_dir,
                        dist: searcher.dist + 1,
                        level: searcher.level
                    });
                }
            }

            if !moved && maze.markers.contains_key(&searcher.pos) {
                let portal_name = &maze.markers[&searcher.pos];
                println!("\x1B[1A{: >5} searchers {: >4} dist {:>3} level on {}   ", searchers.len() + 1, searcher.dist, searcher.level, portal_name);
                for portal in &maze.portals[portal_name] {
                    if portal.pos != searcher.pos {
                        let new_searcher = searcher.move_through_portal(&portal, recursive);
                        if new_searcher.is_some() {
                            searchers.push_back(new_searcher.unwrap());
                            break
                        }
                    }
                }
            }
        }
    }

    0
}

fn main () {
	let file = fs::read_to_string("./input.txt").expect("unable to download file");
	let data = Maze::from(file);

	println!("star 20-1: {}", solve_maze(&data, false));
    println!("star 20-2: {:?}", solve_maze(&data, true));
}
