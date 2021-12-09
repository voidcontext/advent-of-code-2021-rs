use std::{fs::File, io::{BufRead, BufReader}};

pub fn read_i32_list(filename: &str) -> Vec<i32> {
    let file = File::open(filename).unwrap();
    BufReader::new(file)
        .lines()
        .into_iter()
        .map(|line| {
            line.unwrap().trim().parse().unwrap()
        }).collect()
}

