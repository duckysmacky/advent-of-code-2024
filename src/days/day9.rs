use crate::utils;

pub fn run() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> usize {
    let disk_map = get_disk_map(1);
    let mut file_i = 0;
    let mut block_map = disk_map.chunks(2)
        .map(|pair| {
            let (file_c, space_c) = (pair[0], *pair.get(1).unwrap_or(&0));
            let files = (0..file_c).map(|_| file_i).collect::<Vec<i32>>();
            let spaces = (0..space_c).map(|_| -1).collect::<Vec<i32>>();
            file_i += 1;
            [files, spaces].into_iter().flatten().collect::<Vec<i32>>()
        })
        .flatten()
        .collect::<Vec<i32>>();
    
    println!("Block map: {:?}", block_map);

    while block_map.contains(&-1) {
        for i in 0..block_map.len() {
            if block_map[i] == -1 {
                for j in (0..block_map.len()).rev() {
                    if block_map[j] != -1 {
                        block_map[i] = block_map[j];
                        block_map.remove(j);
                        println!("{:?}", block_map);
                        break
                    }
                }
                break
            }
        }
    }

    block_map.into_iter()
        .enumerate()
        .map(|(i, x)| i * (x as usize))
        .sum()
}

fn part2() -> u32 {
    0
}

fn get_disk_map(part: u8) -> Vec<u8> {
    let lines = utils::read_lines(9, part);
    lines[0].chars()
        .map(|c| c.to_digit(10).unwrap() as u8 )
        .collect::<Vec<u8>>()
}