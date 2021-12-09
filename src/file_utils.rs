use std::{fs::File, io::{BufRead, BufReader}};

use crate::submarine::{Command};

pub fn read_i32_list(filename: &str) -> Vec<i32> {
    let file = File::open(filename).unwrap();

    BufReader::new(file)
        .lines()
        .into_iter()
        .map(|line| {
            line.unwrap().trim().parse().unwrap()
        })
        .collect()
}


pub fn read_submarine_commands(filename: &str) -> impl Iterator<Item=Command> + '_ {
    let file = File::open(filename).unwrap();

    BufReader::new(file)
        .lines()
        .into_iter()
        .map(|line| {
            let l = line.unwrap();
            let tokens = l.split(' ').collect::<Vec<&str>>();
            let cmd = tokens[0];
            let arg = tokens[1];

            match cmd {
                "forward" => Command::Forward(arg.parse().unwrap()),
                "down" => Command::Down(arg.parse().unwrap()),
                "up" => Command::Up(arg.parse().unwrap()),
                _ => panic!("Unknown cmd: {}", cmd)
            }
        })
}
