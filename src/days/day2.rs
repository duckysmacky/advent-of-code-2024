use crate::utils;

pub fn run() {
    println!("Part 1: {}", part1());
    // println!("Part 2: {}", part2());
}

fn part1() -> usize {
    let reports = get_reports(1);
    let safe_reports = reports.into_iter()
        .filter(|nums| {
            nums.windows(2).all(|x| (x[0] < x[1] && x[1] - x[0] <= 3))
            || nums.windows(2).all(|x| (x[1] < x[0] && x[0] - x[1] <= 3))
        })
        .collect::<Vec<Vec<u32>>>();
    safe_reports.iter().for_each(|v| println!("{:?}", v));
    safe_reports.len()
}

fn get_reports(part: u8) -> Vec<Vec<u32>> {
    let lines = utils::read_lines(2, part);
    lines.into_iter()
        .map(|line| line.split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>()
        )
        .collect::<Vec<Vec<u32>>>()
}