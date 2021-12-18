use std::{
    fs::{read_to_string, File},
    io::{BufRead, BufReader},
};

use crate::submarine::Command;

pub fn read_i32_list(filename: &str) -> Vec<i32> {
    let file = File::open(filename).unwrap();

    BufReader::new(file)
        .lines()
        .into_iter()
        .map(|line| line.unwrap().trim().parse().unwrap())
        .collect()
}

pub fn read_and_parse<T, E>(filename: &str, parser: fn(&str) -> Result<T, E>) -> Result<T, E> {
    let content: String = read_to_string(filename).unwrap();

    parser(&content)
}

pub fn read_string_list(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();

    BufReader::new(file)
        .lines()
        .into_iter()
        .map(|line| line.unwrap().trim().to_owned())
        .collect()
}

pub fn read_submarine_commands(filename: &str) -> Vec<Command> {
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
                _ => panic!("Unknown cmd: {}", cmd),
            }
        })
        .collect()
}
