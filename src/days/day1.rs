use std::collections::HashMap;

use crate::utils;

pub fn run() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> u32 {
    let (mut left, mut right) = get_lists(1).into();
    left.sort();
    right.sort();

    let mut diff_sum = 0;
    for i in 0..left.len() {
        diff_sum += left[i].max(right[i]) - left[i].min(right[i]);
    }
    diff_sum
}

fn part2() -> u32 {
    let (left, right) = get_lists(2).into();
    let mut occurences: HashMap<u32, u32> = HashMap::new();
    for i in 0..right.len() {
        let previous = occurences.get(&right[i]).unwrap_or(&0);
        occurences.insert(right[i], previous + 1);
    }

    let sim_score = left
        .into_iter()
        .map(|x| x * occurences.get(&x).unwrap_or(&0))
        .reduce(|val, x| val + x);
    sim_score.unwrap()
}

fn get_lists(part: u8) -> [Vec<u32>; 2] {
    let lines = utils::read_lines(1, part);
    let line_count = lines.len();
    let mut lists = [Vec::new(), Vec::new()];

    for i in 0..line_count {
        let pair = lines[i]
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        for j in 0..pair.len() {
            lists[j].push(pair[j]);
        }
    }
    lists
}
