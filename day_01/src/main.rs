use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::time::Instant;

fn part_1(input: &str) -> i32 {
    let now = Instant::now();
    let file = File::open(input).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut ctr = 0;
    let mut pos = 50;

    for line_result in reader.lines() {
        let line = line_result.expect("Failed to read line");
        let first_char = line.chars().next().unwrap();
        let number_str = &line[1..];
        let number: i32 = number_str.parse().unwrap();

        if first_char == 'L' {
            pos -= number % 100;
        } else {
            pos += number % 100;
        }

        pos %= 100;
        if pos < 0 {
            pos += 100;
        }

        if pos == 0 {
            ctr += 1;
        }
    }
    let elapsed = now.elapsed();
    println!("{:.2?}", elapsed);
    println!("Part 1: {}", ctr);
    ctr
}

fn part_2(input: &str) -> i32 {
    let now = Instant::now();
    let file = File::open(input).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut ctr_2 = 0;
    let mut pos = 50;

    for line_result in reader.lines() {
        let line = line_result.expect("Failed to read line");
        let first_char = line.chars().next().unwrap();
        let number_str = &line[1..];
        let number: i32 = number_str.parse().unwrap();

        // Number of full rotations
        ctr_2 += number / 100;

        let pos_1 = pos;

        if first_char == 'L' {
            pos -= number % 100;
        } else {
            pos += number % 100;
        }

        if ((pos <= 0 || pos >= 100) && pos_1 != 0) && number % 100 != 0 {
            ctr_2 += 1;
        }

        pos %= 100;
        if pos < 0 {
            pos += 100;
        }
    }
    let elapsed = now.elapsed();
    println!("{:.2?}", elapsed);
    println!("Part 2: {}", ctr_2);
    ctr_2
}

fn main() -> io::Result<()> {
    let content = "input.txt";

    let _sum_1 = part_1(content);
    let _sum_2 = part_2(content);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_with_test_file() {
        let result = part_1("test.txt");
        assert_eq!(result, 3);
    }

    #[test]
    fn test_part_2_with_test_file() {
        let result = part_2("test.txt");
        assert_eq!(result, 6);
    }
}
