pub enum Command {
    Forward(i32),
    Up(i32),
    Down(i32)
}

#[derive(Debug)]
pub struct Submarine {
    pub horizontal: i32,
    pub depth: i32
}

impl Submarine {
    pub fn new() -> Submarine {
        Submarine {
            horizontal: 0,
            depth: 0,
        }
    }

    pub fn run(self, cmd: Command) -> Submarine {
        match cmd {
            Command::Forward(h) => Submarine { horizontal: self.horizontal + h, depth: self.depth },
            Command::Up(d) => Submarine { horizontal: self.horizontal, depth: self.depth - d},
            Command::Down(d) => Submarine { horizontal: self.horizontal, depth: self.depth + d},
        }
    }
}

