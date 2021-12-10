pub enum Command {
    Forward(i32),
    Up(i32),
    Down(i32),
}

#[derive(Debug)]
pub struct AimedSubmarine {
    pub aim: i32,
    pub horizontal: i32,
    pub depth: i32,
}

impl AimedSubmarine {
    pub fn new() -> AimedSubmarine {
        AimedSubmarine {
            aim: 0,
            horizontal: 0,
            depth: 0,
        }
    }

    pub fn run(self, cmd: &Command) -> AimedSubmarine {
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

    pub fn run(self, cmd: &Command) -> BuggySubmarine {
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
