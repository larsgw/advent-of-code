use std::fs;
use std::cmp;
use std::fmt;
use std::collections::HashMap;
mod intcode;

#[derive(Default)]
struct Game {
    ball: (i64, i64),
    paddle: (i64, i64),
    tiles: HashMap<(i64, i64), i64>,
    size: (i64, i64)
}

impl Game {
    fn count_tiles (&self, tile_type: i64) -> i64 {
        let mut sum = 0;
        for tile in self.tiles.values() {
            if *tile == tile_type {
                sum += 1;
            }
        }
        sum
    }

    fn update_tile (&mut self, tile: &[i64]) {
        self.tiles.insert((tile[0], tile[1]), tile[2]);

        match tile[2] {
            3 => { self.paddle = (tile[0], tile[1]); },
            4 => { self.ball = (tile[0], tile[1]); },
            _ => {}
        };
    }

    fn update (&mut self, output: Vec<i64>) {
        let tiles = output.chunks(3);
        for tile in tiles {
            self.update_tile(tile);
        }
    }
}

impl From<Vec<i64>> for Game {
    fn from (output: Vec<i64>) -> Self {
        let mut game: Game = Default::default();
        let tiles = output.chunks(3);

        let mut size = (0, 0);
        for tile in tiles {
            size.0 = cmp::max(size.0, tile[0]);
            size.1 = cmp::max(size.1, tile[1]);
            game.update_tile(tile);
        }

        game.size = size;

        game
    }
}

impl fmt::Display for Game {
    fn fmt (&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut display = String::new();

        for y in 0..=self.size.1 {
            for x in 0..=self.size.0 {
                let char = match self.tiles.get(&(x, y)) {
                    Some(1) => '#',
                    Some(2) => '+',
                    Some(3) => '=',
                    Some(4) => 'o',
                    Some(0) | _ => ' '
                };
                display.push(char);
            }

            if y == 0 {
                display += &format!(" {:?}", self.tiles.get(&(-1, 0)));
            }

            display += &" ".repeat(20);
            display.push('\n');
        }

        write!(f, "{}", display)
    }
}

fn star_13_1 (tape: &Vec<i64>) -> i64 {
    let game = Game::from(intcode::run(tape, &vec![]));
    game.count_tiles(2)
}

fn play_game (tape: &Vec<i64>) -> Option<i64> {
    let mut state = intcode::ProgramState {
        tape: tape.to_vec(),
        ..Default::default()
    };
    state.tape[0] = 2;

    let mut game = Game::from(intcode::step_input(&mut state, None));
    println!("{}", game);

    loop {
        let prev_size = game.size.1;
        let position = (game.ball.0 - game.paddle.0).signum();
        game.update(intcode::step_input(&mut state, Some(position)));

        println!("\x1B[{}A{}", prev_size + 2, game);

        if game.count_tiles(2) == 0 {
            let score = game.tiles.get(&(-1, 0));
            return match score {
                Some(_) => Some(*score.unwrap()),
                None => None
            }
        }
    }
}

fn parse_number (string: &str) -> i64 {
	string.parse::<i64>().expect("cannot parse string as int")
}

fn main () {
	let file = fs::read_to_string("./input.txt").expect("unable to download file");
	let data = file.trim().split(",").map(parse_number).collect();

	println!("star 13-1: {}", star_13_1(&data));
    println!("star 13-2: {:?}", play_game(&data));
}
