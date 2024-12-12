use crate::utils;

pub fn run() {
    // println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> usize {
    let disk_map = get_disk_map(1);
    let mut file_i = 0;
    let mut map = disk_map.chunks(2)
        .map(|pair| {
            let (file_c, space_c) = (pair[0], *pair.get(1).unwrap_or(&0));
            let files = (0..file_c).map(|_| file_i).collect::<Vec<i32>>();
            let spaces = (0..space_c).map(|_| -1).collect::<Vec<i32>>();
            file_i += 1;
            [files, spaces].into_iter().flatten().collect::<Vec<i32>>()
        })
        .flatten()
        .collect::<Vec<i32>>();
    
    println!("Block map: {:?}", map);

    let mut last_i = 0;
    for j in (0..map.len()).rev() {
        if map[j] == -1 {
            continue;
        }

        if last_i > j {
            break;
        }

        for i in last_i..map.len() {
            if map[i] != -1 {
                continue;
            }

            if i > j {
                break;
            }

            map[i] = map[j];
            map[j] = -1;
            last_i = i;
            break;
        }
    }

    map.into_iter()
        .enumerate()
        .map(|(i, x)| i * if x != -1 { x as usize } else { 0 })
        .sum()
}

fn part2() -> usize {
    let disk_map = get_disk_map(0);
    let mut file_i = 0;
    let mut map = disk_map.chunks(2)
        .map(|pair| {
            let (file_c, space_c) = (pair[0], *pair.get(1).unwrap_or(&0));
            let files = (0..file_c).map(|_| file_i).collect::<Vec<i32>>();
            let spaces = (0..space_c).map(|_| -1).collect::<Vec<i32>>();
            file_i += 1;
            [files, spaces].into_iter().flatten().collect::<Vec<i32>>()
        })
        .flatten()
        .collect::<Vec<i32>>();
    
    println!("Block map: {:?}", map);

    let mut spaces: Vec<(usize, usize)> = vec![];
    let mut space_start = 0;
    let mut space_len = 0;
    for i in 0..map.len() {
        if map[i] != -1 {
            if space_start != 0 {
                spaces.push((space_start, space_len));
            }

            space_start = 0;
            space_len = 0;
            continue;
        }

        if space_start == 0 {
            space_start = i;
        } 

        space_len += 1;
    }

    println!("Spaces: {:?}", spaces);

    let mut files: Vec<(usize, usize)> = vec![];
    let mut file_index = -1;
    let mut file_len = 0;
    for j in (0..map.len()).rev() {
        if map[j] != file_index || map[j] == -1 {
            if file_index != -1 {
                files.push((file_index.try_into().unwrap(), file_len));
                file_index = -1;
                file_len = 0;
            }
        }

        if file_index == -1 && map[j] != -1 {
            file_index = map[j];
            file_len = 0;
        }

        file_len += 1;
    }

    println!("Files: {:?}", files);

    for i in 0..files.len() {
        let (file_number, file_len) = files[i];

        let mut j = 0;
        while j < spaces.len() {
            let (space_start, space_len) = spaces[j];

            if space_len >= file_len {
                for offset in 0..file_len {
                    map[space_start + offset] = file_number as i32;
                    map[i - offset] = -1;
                }

                if space_len > file_len {
                    spaces[j] = (space_start + file_len, space_len - file_len);
                } else {
                    spaces.remove(j);
                }
            }

            j += 1;
        }

        println!("{} {}\n{:?}\n", j, i, map.iter()
            .map(|x| x.to_string())
            .map(|x| if x == "-1" {
                    ".".to_string()
                } else {
                    x
            })
            .collect::<String>());
    }

    map.into_iter()
        .enumerate()
        .map(|(i, x)| i * if x != -1 { x as usize } else { 0 })
        .sum()
}

fn get_disk_map(part: u8) -> Vec<u8> {
    let lines = utils::read_lines(9, part);
    lines[0].chars()
        .map(|c| c.to_digit(10).unwrap() as u8 )
        .collect::<Vec<u8>>()
}
