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

    for row in 0..input.len() {
        for (ix, c_char) in input[row].chars().enumerate() {
            if c_char == '.' {
                continue;
            }
            let mut iter: Vec<Option<char>> = vec![];
            for v_offset in [-1isize, 0, 1] {
                let idx = row as isize + v_offset;
                if idx < 0 {
                    continue;
                }
                if let Some(line) = input.get(idx as usize) {
                    for offset in [-1, 0, 1] {
                        if let Some(&b) = line.as_bytes().get((ix as isize + offset) as usize) {
                            iter.push(Some(b as char));
                        } else {
                            iter.push(None); // out of bounds
                        }
                    }
                }
            }
            let mut val = 0;
            for i in iter {
                if i == Some('@') {
                    val += 1;
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

fn part_2(mut input: Vec<String>) -> u32 {
    use std::time::Instant;
    let now = Instant::now();
    let mut sum: u32 = 0;
    loop {
        let mut removed = 0;
        let mut to_remove: Vec<(usize, usize)> = vec![];
        for row in 0..input.len() {
            for (ix, c_char) in input[row].chars().enumerate() {
                if c_char == '.' {
                    continue;
                }
                let mut iter: Vec<Option<char>> = vec![];
                for v_offset in [-1isize, 0, 1] {
                    let idx = row as isize + v_offset;
                    if idx < 0 {
                        continue;
                    }
                    if let Some(line) = input.get(idx as usize) {
                        for offset in [-1, 0, 1] {
                            if let Some(&b) = line.as_bytes().get((ix as isize + offset) as usize) {
                                iter.push(Some(b as char));
                            } else {
                                iter.push(None); // out of bounds
                            }
                        }
                    }
                }
                let mut val = 0;
                for i in iter {
                    if i == Some('@') {
                        val += 1;
                    }
                }
                if val <= 4 {
                    sum += 1;
                    removed += 1;
                    to_remove.push((row, ix));
                }
            }
        }
        for coord in to_remove {
            input[coord.0].replace_range(coord.1..=coord.1, ".");
        }
        if removed == 0 {
            break;
        }
    }
    let elapsed = now.elapsed();
    println!("{:.2?}", elapsed);
    println!("Total sum: {}", sum);
    sum
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
