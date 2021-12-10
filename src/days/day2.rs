use crate::submarine::{self, *};

pub fn calc_position_mul(cmds: &[submarine::Command]) -> i32 {
    let submarine = cmds.iter().fold(BuggySubmarine::new(), |s, cmd| s.run(cmd));
    submarine.depth * submarine.horizontal
}

pub fn calc_position_aimed(cmds: &[submarine::Command]) -> i32 {
    let submarine = cmds.iter().fold(AimedSubmarine::new(), |s, cmd| s.run(cmd));
    submarine.depth * submarine.horizontal
}
