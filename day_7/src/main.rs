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

fn part_1(input: Vec<String>) -> u64 {
    use std::time::Instant;
    let now = Instant::now();
    let mut sum: u64 = 0;
    let lines: Vec<Vec<char>> = input.into_iter().map(|x| x.chars().collect()).collect();

    let mut mapped: Vec<Vec<i32>> = lines
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|c| match c {
                    '.' => 0,
                    '^' => -1,
                    'S' => 1,
                    _ => 0,
                })
                .collect()
        })
        .collect();

    for ix in 0..mapped.len() {
        let line = mapped[ix].clone();
        if ix == mapped.len() - 1 {
            break;
        }
        for (xc, val) in line.iter().enumerate() {
            if val == &0 || val == &-1 {
                continue;
            } else if mapped[ix + 1][xc] == -1 {
                sum += 1;
                mapped[ix + 1][xc + 1] = 1;
                mapped[ix + 1][xc - 1] = 1;
            } else {
                mapped[ix + 1][xc] = 1;
            }
        }
    }
    let elapsed = now.elapsed();
    println!("{:.2?}", elapsed);
    println!("Total splits part 1: {}", sum);
    sum
}

fn part_2(input: Vec<String>) -> u64 {
    use std::time::Instant;
    let now = Instant::now();
    let mut sum: u64 = 0;
    let lines: Vec<Vec<char>> = input.into_iter().map(|x| x.chars().collect()).collect();

    let mut mapped: Vec<Vec<i64>> = lines
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|c| match c {
                    '.' => 0,
                    '^' => -1,
                    'S' => 1,
                    _ => 0,
                })
                .collect()
        })
        .collect();

    for ix in 0..mapped.len() {
        let line = mapped[ix].clone();
        if ix == mapped.len() - 1 {
            for val in line {
                if val < 0 {
                    continue;
                }
                sum += val as u64;
            }
            break;
        }
        for (xc, val) in line.iter().enumerate() {
            if val == &0 || val == &-1 {
                continue;
            } else if mapped[ix + 1][xc] == -1 {
                mapped[ix + 1][xc + 1] += val;
                mapped[ix + 1][xc - 1] += val;
            } else {
                mapped[ix + 1][xc] += val;
            }
        }
    }
    let elapsed = now.elapsed();
    println!("{:.2?}", elapsed);
    println!("Total splits part 2: {}", sum);
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = read_file("test.txt");
        let val = part_1(input.unwrap());
        assert_eq!(val, 21);
    }

    #[test]
    fn test_part_2() {
        let input = read_file("test.txt");
        let val = part_2(input.unwrap());
        assert_eq!(val, 40);
    }
}
