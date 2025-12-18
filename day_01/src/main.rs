use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn main() {
    let input = read_file("input.txt").unwrap();
    part_1(&input);
    part_2(&input);
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

fn part_1(input: &Vec<String>) -> i32 {
    let now = Instant::now();

    let mut ctr = 0;
    let mut pos = 50;

    for line in input {
        let first_char = line.chars().next().unwrap();
        let number_str = &line[1..];
        let number: i32 = number_str.parse().unwrap();

        if first_char == 'L' {
            pos -= number;
        } else {
            pos += number;
        }

        pos %= 100;
        //pos = pos.rem_euclid(100);

        if pos == 0 {
            ctr += 1;
        }
    }
    let elapsed = now.elapsed();
    println!("{:.2?}", elapsed);
    println!("Part 1: {}", ctr);
    ctr
}

fn part_2(input: &Vec<String>) -> i32 {
    let now = Instant::now();

    let mut ctr = 0;
    let mut pos = 50;

    for line in input {
        let first_char = line.chars().next().unwrap();
        let number_str = &line[1..];
        let number: i32 = number_str.parse().unwrap();

        // Number of full rotations
        ctr += number / 100;
        let rem = number % 100;

        let prev_pos = pos;

        if first_char == 'L' {
            pos -= rem;
        } else {
            pos += rem;
        }

        if ((pos <= 0 || pos >= 100) && prev_pos != 0) && rem != 0 {
            ctr += 1;
        }

        pos += 100;
        pos %= 100;
    }
    let elapsed = now.elapsed();
    println!("{:.2?}", elapsed);
    println!("Part 2: {}", ctr);
    ctr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_with_test_file() {
        let input = read_file("test.txt").unwrap();
        let result = part_1(&input);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_part_2_with_test_file() {
        let input = read_file("test.txt").unwrap();
        let result = part_2(&input);
        assert_eq!(result, 6);
    }
}
