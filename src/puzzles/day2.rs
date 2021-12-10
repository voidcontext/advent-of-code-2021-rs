use crate::{
    file_utils,
    submarine::{self, *},
};

pub fn solve() {
    let day2_input = file_utils::read_submarine_commands("data/day2.input");
    println!("Day2 A Result {}", calc_position_mul(&day2_input));
    println!("Day2 B Result {}", calc_position_aimed(&day2_input));
}

fn calc_position_mul(cmds: &[submarine::Command]) -> i32 {
    let submarine = cmds.iter().fold(BuggySubmarine::new(), |s, cmd| s.run(cmd));
    submarine.depth * submarine.horizontal
}

fn calc_position_aimed(cmds: &[submarine::Command]) -> i32 {
    let submarine = cmds.iter().fold(AimedSubmarine::new(), |s, cmd| s.run(cmd));
    submarine.depth * submarine.horizontal
}
