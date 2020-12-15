use rust_2020::read_lines;
use std::convert::TryInto;

#[derive(Debug, Copy, Clone)]
struct Ship {
    x: i32,
    y: i32,
    facing: Direction,
    waypoint: Waypoint,
}
impl Ship {
    fn new() -> Ship {
        Ship {
            x: 0,
            y: 0,
            facing: Direction::E,
            waypoint: Waypoint::new(),
        }
    }
    fn distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}
#[derive(Debug, Copy, Clone)]
struct Waypoint {
    x: i32,
    y: i32,
}
impl Waypoint {
    fn new() -> Waypoint {
        Waypoint { x: 10, y: 1 }
    }
    fn rotate_l(&mut self, num: u8) {
        for _ in 0..num {
            /*
             * +,+ -> -,+
             * +,- -> +,+
             * -,+ -> -,-
             * -,- -> +,-
             */
            let x = self.x;
            self.x = -self.y;
            self.y = x
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    N,
    S,
    E,
    W,
}
impl Direction {
    fn rotate_l(self, num: u8) -> Self {
        let mut d = self;
        for _ in 0..num {
            d = match d {
                Direction::N => Direction::W,
                Direction::S => Direction::E,
                Direction::E => Direction::N,
                Direction::W => Direction::S,
            }
        }
        d
    }
}

#[derive(Debug, Copy, Clone)]
enum Command {
    N(i32),
    S(i32),
    E(i32),
    W(i32),
    L(u8),
    R(u8),
    F(i32),
}
impl std::str::FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let i: i32 = s[1..].parse().unwrap();
        match s.chars().next().unwrap() {
            'N' => Ok(Self::N(i)),
            'S' => Ok(Self::S(i)),
            'E' => Ok(Self::E(i)),
            'W' => Ok(Self::W(i)),
            'L' => Ok(Self::L(((i / 90) % 4).try_into().unwrap())),
            'R' => Ok(Self::R(((i / 90) % 4).try_into().unwrap())),
            'F' => Ok(Self::F(i)),
            _ => Err("Invalid Command!".to_string()),
        }
    }
}

fn main() {
    let mut ship1 = Ship::new();
    let mut ship2 = Ship::new();
    for c in read_lines("puzzle-input/day12.txt").map(|s| s.parse::<Command>().unwrap()) {
        process_ship_command(&mut ship1, c);
        process_waypoint_command(&mut ship2, c)
    }
    println!("Part 1: {}\nPart 2: {}", ship1.distance(), ship2.distance());
}

fn process_ship_command(ship: &mut Ship, command: Command) {
    match if let Command::F(i) = command {
        match ship.facing {
            Direction::N => Command::N(i),
            Direction::S => Command::S(i),
            Direction::E => Command::E(i),
            Direction::W => Command::W(i),
        }
    } else if let Command::R(i) = command {
        Command::L(4 - i)
    } else {
        command
    } {
        Command::N(i) => ship.y += i,
        Command::S(i) => ship.y -= i,
        Command::E(i) => ship.x += i,
        Command::W(i) => ship.x -= i,
        Command::L(i) => ship.facing = ship.facing.rotate_l(i),
        _ => panic!("Impossible!"),
    }
}

fn process_waypoint_command(ship: &mut Ship, command: Command) {
    match if let Command::R(i) = command {
        Command::L(4 - i)
    } else {
        command
    } {
        Command::N(i) => ship.waypoint.y += i,
        Command::S(i) => ship.waypoint.y -= i,
        Command::E(i) => ship.waypoint.x += i,
        Command::W(i) => ship.waypoint.x -= i,
        Command::L(i) => ship.waypoint.rotate_l(i),
        Command::F(i) => {
            for _ in 0..i {
                ship.x += ship.waypoint.x;
                ship.y += ship.waypoint.y;
            }
        }
        _ => panic!("Impossible!"),
    }
}
