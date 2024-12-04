use crate::utils;

pub fn run() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> usize {
    let reports = get_reports(1);
    let safe_reports = reports
        .into_iter()
        .filter(|nums| {
            // Order nums in ascending order in order to use the same function on both cases
            // Because the first and the last numbers are always bigger or smaller then each
            // other, we can see if it needs to be flipped by comparing first and last
            let nums: Vec<&u32> = if nums[0] > nums[nums.len() - 1] {
                nums.into_iter().rev().collect()
            } else {
                nums.into_iter().collect() // just so the types are the same
            };

            // Check if every pair, which was split with the windows functions, will follow
            // the rule of being smaller than the other (as the numbers are always ascending
            // after the sort) and also differ by max of 3 digits
            nums.windows(2).all(check_pair)
        })
        .collect::<Vec<Vec<u32>>>();

    safe_reports.iter().for_each(|v| println!("{:?}", v));
    safe_reports.len()
}

fn part2() -> usize {
    let reports = get_reports(2);
    let safe_reports = reports
        .into_iter()
        .filter(|nums| {
            // See Part 1 explanation
            let nums: Vec<&u32> = if nums[0] > nums[nums.len() - 1] {
                nums.into_iter().rev().collect()
            } else {
                nums.into_iter().collect()
            };

            let valid = nums.windows(2).all(check_pair);

            if valid {
                println!("Safe: {:?}", nums);
                return true;
            }

            // We continue only if the first check failed
            for i in 0..nums.len() {
                // We remove one number at a time from the original list and check
                // whether by removing this number the updated list (which is always
                // missing one of the original numbers) will pass the condition
                let cleaned = nums
                    .iter()
                    .enumerate() // convert to enum in order to get index
                    .filter(|(x, _)| *x != i) // filter out unwated index
                    .map(|(_, x)| *x) // map back from enum (index and value) to only value
                    .collect::<Vec<&u32>>();
                if cleaned.windows(2).all(check_pair) {
                    println!("Now safe ({}): {:?}", i, nums);
                    return true;
                }
            }

            println!("Not safe: {:?}", nums);
            false
        })
        .collect::<Vec<Vec<u32>>>();
    safe_reports.iter().for_each(|v| println!("{:?}", v));
    safe_reports.len()
}

fn check_pair(pair: &[&u32]) -> bool {
    pair[0] < pair[1] && pair[1] - pair[0] <= 3
}

fn get_reports(part: u8) -> Vec<Vec<u32>> {
    let lines = utils::read_lines(2, part);
    lines
        .into_iter()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>()
}
