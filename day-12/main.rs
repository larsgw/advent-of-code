use std::fs;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Body {
    x: (i64, i64),
    y: (i64, i64),
    z: (i64, i64)
}

impl From<&str> for Body {
    fn from (string: &str) -> Self {
        let pairs = string[1..(string.len() - 1)].split(", ").collect::<Vec<&str>>();
        let x = pairs[0][2..].parse::<i64>().expect("cannot parse value");
        let y = pairs[1][2..].parse::<i64>().expect("cannot parse value");
        let z = pairs[2][2..].parse::<i64>().expect("cannot parse value");
        Body {
            x: (x, 0),
            y: (y, 0),
            z: (z, 0)
        }
    }
}

impl Body {
    fn get_total_energy (self) -> u64 {
        let pot = self.x.0.abs() + self.y.0.abs() + self.z.0.abs();
        let kin = self.x.1.abs() + self.y.1.abs() + self.z.1.abs();
        (pot * kin) as u64
    }

    fn get_velocity_diff (&self, other: &Body) -> (i64, i64, i64) {
        let x = (other.x.0 - self.x.0).signum();
        let y = (other.y.0 - self.y.0).signum();
        let z = (other.z.0 - self.z.0).signum();
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
                body.x.1 += x;
                body.y.1 += y;
                body.z.1 += z;
            }
        }
    }

    for body in bodies {
        body.x.0 += body.x.1;
        body.y.0 += body.y.1;
        body.z.0 += body.z.1;
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

fn gcd (a: u64, b: u64) -> u64 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm (a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

fn calculate_steps (bodies: &Vec<Body>) -> u64 {
    let len = bodies.len();
    let mut state = bodies.to_vec();
    let mut steps = (None, None, None);
    let mut counter = 0;
    println!("");

    loop {
        step(&mut state);
        counter += 1;
        println!("\x1B[1A{} steps", counter);

        let mut same = (true, true, true);
        for i in 0..len {
            let a = state[i];
            let b = bodies[i];
            if a.x != b.x { same.0 = false; }
            if a.y != b.y { same.1 = false; }
            if a.z != b.z { same.2 = false; }
        }

        if steps.0.is_none() && same.0 { println!("\x1B[1Ax in {} steps\n", counter); steps.0 = Some(counter); }
        if steps.1.is_none() && same.1 { println!("\x1B[1Ay in {} steps\n", counter); steps.1 = Some(counter); }
        if steps.2.is_none() && same.2 { println!("\x1B[1Az in {} steps\n", counter); steps.2 = Some(counter); }

        if steps.0.is_some() && steps.1.is_some() && steps.2.is_some() {
            println!("\x1B[2A");
            return lcm(lcm(steps.0.unwrap(), steps.1.unwrap()), steps.2.unwrap())
        }
    }
}

fn main() {
    let file = fs::read_to_string("./input.txt").expect("unable to download file");
    let data = file.trim().split("\n").map(|str| Body::from(str)).collect::<Vec<Body>>();

    println!("star 12-1: {}", calculate_energy(&data, 1000));
    println!("star 12-2: {}", calculate_steps(&data));
}
