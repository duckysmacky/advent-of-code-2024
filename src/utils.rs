use std::{
    fs::File,
    io::{BufRead, BufReader},
    process,
};

pub fn read_lines(day: u8, part: u8) -> Vec<String> {
    let path = format!("files/day{}/{}.txt", day, part);
    let file = File::open(&path).unwrap_or_else(|err| {
        println!("Unable to open \"{}\": {}", path, err);
        process::exit(1);
    });

    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|l| l.unwrap_or(String::new()))
        .collect::<Vec<String>>()
}
