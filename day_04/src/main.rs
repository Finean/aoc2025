use std::cmp::{max, min};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input = read_file("input.txt");
    part_1(input.unwrap());
    let input = read_file("input.txt");
    part_2(input.unwrap());
}

fn read_file(name: &str) -> std::io::Result<Vec<String>> {
    let mut list: Vec<String> = Vec::new();
    let file = File::open(name).expect("File not found");
    let buf = BufReader::new(file);

    for line in buf.lines() {
        list.push(line?);
    }
    Ok(list)
}

fn part_1(input: Vec<String>) -> u32 {
    use std::time::Instant;
    let now = Instant::now();
    let mut sum: u32 = 0;

    for (row, line) in input.iter().enumerate() {
        let bytes = line.as_bytes();
        for (ix, &c_char) in bytes.iter().enumerate() {
            if c_char == b'.' {
                continue;
            }

            let mut val = 0;
            for v_offset in -1isize..=1 {
                let idx = row as isize + v_offset;
                if idx < 0 || idx >= input.len() as isize {
                    continue;
                }
                let cur_line = input[idx as usize].as_bytes();
                for h_offset in -1..=1 {
                    let c_idx = ix as isize + h_offset;
                    if c_idx < 0 || c_idx >= cur_line.len() as isize {
                        continue;
                    }
                    if cur_line[c_idx as usize] == b'@' {
                        val += 1;
                    }
                }
            }
            if val <= 4 {
                sum += 1
            }
        }
    }
    let elapsed = now.elapsed();
    println!("{:.2?}", elapsed);
    println!("Total sum: {}", sum);
    sum
}

fn part_2(input: Vec<String>) -> u32 {
    use std::time::Instant;
    let now = Instant::now();
    let mut sum: u32 = 0;
    let rows = input.len();
    let cols = input[0].len();
    let mut check_coords = coordrange((0, rows), (0, cols));
    //Convert input into Vec<Vec<char>>
    let mut chars: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();
    loop {
        let mut removed = 0;
        let mut to_remove: Vec<(usize, usize)> = vec![];
        let mut temp: Vec<(usize, usize)> = vec![];
        //Only check coords in check_coords to avoid unnecessary computations
        for coord in &check_coords {
            let c_char = chars[coord.0][coord.1];
            if c_char == '.' {
                continue;
            }
            let mut val = 0;
            for v_offset in -1isize..=1 {
                let idx = coord.0 as isize + v_offset;
                if idx < 0 || idx >= input.len() as isize {
                    continue;
                }
                let cur_line = &chars[idx as usize];
                for h_offset in -1..=1 {
                    let c_idx = coord.1 as isize + h_offset;
                    if c_idx < 0 || c_idx >= cur_line.len() as isize {
                        continue;
                    }
                    if cur_line[c_idx as usize] == '@' {
                        val += 1;
                    }
                }
            }
            if val <= 4 {
                sum += 1;
                removed += 1;
                to_remove.push((coord.0, coord.1));
                let tmp = coordrange(
                    (
                        clamp(coord.0 as isize - 1, rows, 0),
                        clamp(coord.0 as isize + 1, rows, 0),
                    ),
                    (
                        clamp(coord.1 as isize - 1, cols, 0),
                        clamp(coord.1 as isize + 1, cols, 0),
                    ),
                );
                temp.extend(tmp);
            }
        }

        for coord in to_remove {
            chars[coord.0][coord.1] = '.';
        }
        if removed == 0 {
            break;
        }
        check_coords = temp;
    }
    let elapsed = now.elapsed();
    println!("{:.2?}", elapsed);
    println!("Total sum: {}", sum);
    sum
}

fn coordrange(x_range: (usize, usize), y_range: (usize, usize)) -> Vec<(usize, usize)> {
    (x_range.0..x_range.1)
        .flat_map(|i| (y_range.0..y_range.1).map(move |j| (i as usize, j as usize)))
        .collect()
}

fn clamp(x: isize, mx: usize, mn: usize) -> usize {
    min(mx, max(mn as isize, x) as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = read_file("test.txt");
        let val = part_1(input.unwrap());
        assert_eq!(val, 13);
    }

    #[test]
    fn test_part_2() {
        let input = read_file("test.txt");
        let val = part_2(input.unwrap());
        assert_eq!(val, 43);
    }
}
