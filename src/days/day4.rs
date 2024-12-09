use regex::Regex;

use crate::utils;

pub fn run() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> u32 {
    let grid = get_grid(1);
    let pattern = Regex::new(r"(XMAS)|(SAMX)").unwrap();
	let mut count: u32 = 0;

	// Vertical
	for y in 0..grid.len() - 3 {
		for x in 0..grid[y].len() {
			let word = format!("{}{}{}{}", grid[y][x], grid[y+1][x], grid[y+2][x], grid[y+3][x]);
            if pattern.is_match(&word) {
                count += 1;
            }
		}
	}

	// Horizontal
	for y in 0..grid.len() {
		for x in 0..grid[y].len() - 3 {
			let word = format!("{}{}{}{}", grid[y][x], grid[y][x+1], grid[y][x+2], grid[y][x+3]);
            if pattern.is_match(&word) {
                count += 1;
            }
        }
	}

	// Down diagonal
	for y in 0..grid.len() - 3 {
		for x in 0..grid[y].len() - 3 {
			let word = format!("{}{}{}{}", grid[y][x], grid[y+1][x+1], grid[y+2][x+2], grid[y+3][x+3]);
			if pattern.is_match(&word) {
                count += 1;
            }
		}
	}

	// Up diagonal
	for y in 0..grid.len() - 3 {
		for x in 3..grid[y].len() {
			let word = format!("{}{}{}{}", grid[y][x], grid[y+1][x-1], grid[y+2][x-2], grid[y+3][x-3]);
			if pattern.is_match(&word) {
                count += 1;
            }
		}
	}

    count
}

fn part2() ->u32 {
    let grid = get_grid(2);
	let pattern = Regex::new(r"(MSAMS)|(SMASM)|(MMASS)|(SSAMM)").unwrap();
	let mut count: u32 = 0;

	for y in 1..grid.len() - 1 {
		for x in 1..grid[y].len() - 1 {
			let word = format!("{}{}{}{}{}",
				grid[y - 1][x - 1], grid[y - 1][x + 1], grid[y][x], grid[y + 1][x - 1], grid[y + 1][x + 1]
			);
			if pattern.is_match(&word) {
				count += 1;
			}
		}
	}

	count
}

fn get_grid(part: u8) -> Vec<Vec<char>> {
    let lines = utils::read_lines(4, part);
    lines.into_iter()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}