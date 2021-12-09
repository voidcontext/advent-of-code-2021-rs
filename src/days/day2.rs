use crate::submarine::{self, Submarine};


pub fn calc_position_mul(cmds: &mut dyn Iterator<Item=submarine::Command>) -> i32 {
    let submarine = cmds.fold(Submarine::new(), |s, cmd| s.run(cmd));
    submarine.depth * submarine.horizontal
}
