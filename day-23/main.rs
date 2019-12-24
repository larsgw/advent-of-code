use std::fs;
use std::convert::TryInto;
mod intcode;

#[derive(Debug, Copy, Clone)]
struct PacketContent (i64, i64);
#[derive(Debug, Copy, Clone)]
struct Packet {
    reciever: i64,
    content: PacketContent
}

impl From<[i64; 3]> for Packet {
    fn from (chunk: [i64; 3]) -> Self {
        Packet {
            reciever: chunk[0],
            content: PacketContent (chunk[1], chunk[2])
        }
    }
}

#[derive(Default)]
struct Computer {
    address: i64,
    memory: intcode::ProgramState,
    queue: Vec<i64>
}

impl Computer {
    fn init (&mut self) {
        self.queue.push(self.address);
    }

    fn recieve_packet (&mut self, packet: &PacketContent) {
        self.queue.push(packet.0);
        self.queue.push(packet.1);
    }

    fn get_output (&mut self, input: i64) -> Vec<i64> {
        intcode::step_input(&mut self.memory, Some(input))
    }

    fn run_queue_items (&mut self, n: usize) -> Vec<i64> {
        let mut output = Vec::new();
        for _ in 0..n {
            let x = self.queue.remove(0);
            output.append(&mut self.get_output(x));
        }
        output
    }

    fn run_input_event (&mut self) -> Vec<i64> {
        if self.queue.len() == 0 {
            self.queue.push(-1)
        }

        self.run_queue_items(std::cmp::min(self.queue.len(), 2))
    }
}

struct Network {
    computers: Vec<Computer>,
    last_nat_packet: Option<Packet>,
    all_idle: bool
}

impl Network {
    fn init (&mut self) {
        for computer in &mut self.computers {
            computer.init();
        }
    }

    fn new (tape: &Vec<i64>, size: i64) -> Self {
        let computers = (0..size).map(|address| Computer {
            address: address as i64,
            memory: intcode::ProgramState {
                tape: tape.to_vec(),
                ..Default::default()
            },
            ..Default::default()
        }).collect::<Vec<Computer>>();

        Network {
            computers,
            last_nat_packet: None,
            all_idle: false
        }
    }

    fn send_packet (&mut self, packet: Packet) {
        let computer = &mut self.computers[packet.reciever as usize];
        computer.recieve_packet(&packet.content);
    }

    fn run_event_loop (&mut self) {
        let mut all_packets: Vec<Packet> = Vec::new();
        let mut all_idle = true;

        for computer in &mut self.computers {
            if computer.queue.len() != 0 { all_idle = false }

            let output = computer.run_input_event();
            let mut packets: Vec<Packet> = output
                .chunks(3)
                .map(|chunk| {
                    let sized_chunk: [i64; 3] = chunk.try_into().expect("unexpected remainder");
                    Packet::from(sized_chunk)
                })
                .collect::<Vec<Packet>>();

            if computer.queue.len() != 0 { all_idle = false }
            all_packets.append(&mut packets);
        }

        if all_idle && all_packets.len() == 0 {
            self.all_idle = true
        }

        for packet in all_packets {
            if packet.reciever == 255 {
                self.last_nat_packet = Some(packet);
            } else {
                self.send_packet(packet);
            }
        }
    }
}

fn star_23_1 (tape: &Vec<i64>) -> i64 {
    let mut network = Network::new(tape, 50);
    network.init();

    loop {
        network.run_event_loop();
        if network.last_nat_packet.is_some() {
            return network.last_nat_packet.unwrap().content.1;
        }
    }
}

fn star_23_2 (tape: &Vec<i64>) -> i64 {
    let mut network = Network::new(tape, 50);
    network.init();

    let mut wakeup_calls = std::collections::HashSet::new();
    loop {
        network.run_event_loop();

        if network.all_idle {
            let packet = &network.last_nat_packet.unwrap();
            network.last_nat_packet = None;
            if !wakeup_calls.insert(packet.content.1) {
                return packet.content.1
            }
            network.all_idle = false;
            network.send_packet(Packet {
                reciever: 0,
                content: packet.content
            });
        }
    }
}

fn parse_number (string: &str) -> i64 {
	string.parse::<i64>().expect("cannot parse string as int")
}

fn main () {
	let file = fs::read_to_string("./input.txt").expect("unable to download file");
	let data = file.trim().split(",").map(parse_number).collect::<Vec<i64>>();

	println!("star 23-1: {:?}", star_23_1(&data));
    println!("star 23-2: {:?}", star_23_2(&data));
}
