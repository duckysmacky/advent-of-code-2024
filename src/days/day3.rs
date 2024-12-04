use regex::Regex;

use crate::utils;

pub fn run() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> u32 {
    let memory = get_memory(1);
    let mul_pattern = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    mul_pattern
        .captures_iter(&memory)
        .map(|c| c.extract())
        .map(|(_, [x, y])| x.parse::<u32>().unwrap() * y.parse::<u32>().unwrap())
        .reduce(|val, x| val + x)
        .unwrap()
}

fn part2() -> u32 {
    let memory = get_memory(2);
    let memory = format!("do(){}don't()", memory);
    let mul_pattern = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let enabled_pattern = Regex::new(r"(?:do\(\))(.*?)(?:don't\(\))").unwrap();

    enabled_pattern
        .captures_iter(&memory)
        .map(|c| c.extract())
        .map(|(_, [segment])| {
            mul_pattern
                .captures_iter(segment)
                .map(|c| c.extract())
                .map(|(_, [x, y])| x.parse::<u32>().unwrap() * y.parse::<u32>().unwrap())
                .reduce(|val, x| val + x)
                .unwrap()
        })
        .reduce(|val, x| val + x)
        .unwrap()
}

fn get_memory(part: u8) -> String {
    let lines = utils::read_lines(3, part);
    lines.join("")
}
