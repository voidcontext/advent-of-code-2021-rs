use crate::file_utils;

pub fn solve() {
    let day2_input = read_submarine_commands("data/day2.input");
    println!("Day2 A Result {}", calc_position_mul(&day2_input));
    println!("Day2 B Result {}", calc_position_aimed(&day2_input));
}

fn read_submarine_commands(filename: &str) -> Vec<Command> {
    file_utils::read_string_list(filename)
        .into_iter()
        .map(|line| {
            let tokens = line.split(' ').collect::<Vec<&str>>();
            let cmd = tokens[0];
            let arg = tokens[1];

            match cmd {
                "forward" => Command::Forward(arg.parse().unwrap()),
                "down" => Command::Down(arg.parse().unwrap()),
                "up" => Command::Up(arg.parse().unwrap()),
                _ => panic!("Unknown cmd: {}", cmd),
            }
        })
        .collect()
}

fn calc_position_mul(cmds: &[Command]) -> i32 {
    let submarine = cmds.iter().fold(BuggySubmarine::new(), |s, cmd| s.run(cmd));
    submarine.depth * submarine.horizontal
}

fn calc_position_aimed(cmds: &[Command]) -> i32 {
    let submarine = cmds.iter().fold(AimedSubmarine::new(), |s, cmd| s.run(cmd));
    submarine.depth * submarine.horizontal
}

enum Command {
    Forward(i32),
    Up(i32),
    Down(i32),
}

#[derive(Debug)]
struct AimedSubmarine {
    pub aim: i32,
    pub horizontal: i32,
    pub depth: i32,
}

impl AimedSubmarine {
    fn new() -> AimedSubmarine {
        AimedSubmarine {
            aim: 0,
            horizontal: 0,
            depth: 0,
        }
    }

    fn run(self, cmd: &Command) -> AimedSubmarine {
        match cmd {
            Command::Forward(h) => AimedSubmarine {
                horizontal: self.horizontal + h,
                depth: self.depth + (self.aim * h),
                ..self
            },
            Command::Up(d) => AimedSubmarine {
                aim: self.aim - d,
                ..self
            },
            Command::Down(d) => AimedSubmarine {
                aim: self.aim + d,
                ..self
            },
        }
    }
}

#[cfg(test)]
mod aimed_submarine_tests {
    use super::*;

    #[test]
    fn test_aimed_pos() {
        let sub = AimedSubmarine::new();

        let cmds = [
            Command::Forward(5),
            Command::Down(5),
            Command::Forward(8),
            Command::Up(3),
            Command::Down(8),
            Command::Forward(2),
        ];

        let moved = cmds.iter().fold(sub, |sub, cmd| sub.run(cmd));

        assert_eq!(moved.horizontal, 15);
        assert_eq!(moved.depth, 60);
    }
}

#[derive(Debug)]
pub struct BuggySubmarine {
    pub horizontal: i32,
    pub depth: i32,
}

impl BuggySubmarine {
    pub fn new() -> BuggySubmarine {
        BuggySubmarine {
            horizontal: 0,
            depth: 0,
        }
    }

    fn run(self, cmd: &Command) -> BuggySubmarine {
        match cmd {
            Command::Forward(h) => BuggySubmarine {
                horizontal: self.horizontal + h,
                depth: self.depth,
            },
            Command::Up(d) => BuggySubmarine {
                horizontal: self.horizontal,
                depth: self.depth - d,
            },
            Command::Down(d) => BuggySubmarine {
                horizontal: self.horizontal,
                depth: self.depth + d,
            },
        }
    }
}
