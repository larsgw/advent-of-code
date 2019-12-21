use std::fs;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Body {
    position: (i64, i64, i64),
    velocity: (i64, i64, i64)
}

impl From<&str> for Body {
    fn from (string: &str) -> Self {
        let pairs = string[1..(string.len() - 1)].split(", ").collect::<Vec<&str>>();
        let x = pairs[0][2..].parse::<i64>().expect("cannot parse value");
        let y = pairs[1][2..].parse::<i64>().expect("cannot parse value");
        let z = pairs[2][2..].parse::<i64>().expect("cannot parse value");
        Body {
            position: (x, y, z),
            velocity: (0, 0, 0)
        }
    }
}

impl Body {
    fn get_total_energy (self) -> u64 {
        let pot = self.position.0.abs() + self.position.1.abs() + self.position.2.abs();
        let kin = self.velocity.0.abs() + self.velocity.1.abs() + self.velocity.2.abs();
        (pot * kin) as u64
    }

    fn get_velocity_diff (&self, other: &Body) -> (i64, i64, i64) {
        let x = (other.position.0 - self.position.0).signum();
        let y = (other.position.1 - self.position.1).signum();
        let z = (other.position.2 - self.position.2).signum();
        (x, y, z)
    }
}

fn step (bodies: &mut Vec<Body>) {
    let len = bodies.len();
    for i in 0..len {
        for j in 0..len {
            if i != j {
                let (x, y, z) = bodies[i].get_velocity_diff(&bodies[j]);
                let body = &mut bodies[i];
                body.velocity.0 += x;
                body.velocity.1 += y;
                body.velocity.2 += z;
            }
        }
    }

    for body in bodies {
        body.position.0 += body.velocity.0;
        body.position.1 += body.velocity.1;
        body.position.2 += body.velocity.2;
    }
}

fn steps (bodies: &mut Vec<Body>, n_steps: u64) {
    for _ in 0..n_steps {
        step(bodies)
    }
}

fn calculate_energy (bodies: &Vec<Body>, n_steps: u64) -> u64 {
    let mut state = bodies.to_vec();
    steps(&mut state, n_steps);

    state.iter().map(|body| body.get_total_energy()).sum()
}

fn calculate_steps (bodies: &Vec<Body>) -> u64 {
    let mut state = bodies.to_vec();
    let mut states = std::collections::HashSet::new();
    let mut steps = 0;
    println!("");
    loop {
        let hash = state.to_vec();
        step(&mut state);
        steps += 1;
        println!("\x1B[1A{} steps", steps);
        if states.contains(&hash) {
            return steps
        } else {
            states.insert(hash);
        }
    }
}

fn main() {
    let file = fs::read_to_string("./input.txt").expect("unable to download file");
    let data = file.trim().split("\n").map(|str| Body::from(str)).collect::<Vec<Body>>();

    println!("star 12-1: {}", calculate_energy(&data, 1000));
    println!("star 12-2: {}", calculate_steps(&data));
}
