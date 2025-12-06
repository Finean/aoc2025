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
    let mut sum: u64 = 0;
    let mut lines: Vec<Vec<u64>> = vec![];
    let mut ops: Vec<&str> = vec![];

    //Format input into arrays
    for (ix, line) in input.iter().enumerate() {
        if ix < input.len() - 1 {
            let split: Vec<u64> = line
                .split_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            lines.push(split);
        } else {
            ops = line.split_whitespace().collect::<Vec<&str>>();
        }
    }

    for (ix, op) in ops.into_iter().enumerate() {
        if op == "*" {
            let mut x = lines[0][ix] as u64;
            for it in 1..lines.len() {
                x *= lines[it][ix] as u64;
            }
            sum += x;
        } else {
            let mut x: u64 = 0;
            for it in 0..lines.len() {
                x += lines[it][ix] as u64;
            }
            sum += x;
        }
    }

    println!("Total sum part 1: {}", sum);
    sum
}

fn part_2(input: Vec<String>) -> u64 {
    let mut sum: u64 = 0;
    let mut char_lines: Vec<Vec<char>> = vec![];

    //Format input into arrays
    for (ix, line) in input.iter().enumerate() {
        if ix < input.len() - 1 {
            let split: Vec<char> = line.chars().collect();
            char_lines.push(split);
        }
    }

    for (ix, op) in input
        .last()
        .expect("Reading operations line")
        .chars()
        .enumerate()
    {
        if op == ' ' {
            continue;
        }
        //Gather digits
        let mut val = 0 as u64;
        let mut offset = 0 as usize;
        loop {
            let mut digits: Vec<char> = vec![];
            let mut whitespace = true;
            for lin in 0..char_lines.len() {
                if let Some(ch) = char_lines[lin].get(ix + offset) {
                    digits.push(*ch);
                    if *ch != ' ' {
                        whitespace = false;
                    }
                } else {
                    whitespace = true;
                    break;
                }
            }
            if whitespace {
                break;
            }
            let num = digits
                .into_iter()
                .filter(|&c| c != ' ')
                .collect::<String>()
                .parse::<u64>()
                .unwrap();
            if op == '*' {
                if offset == 0 {
                    val = num;
                } else {
                    val *= num;
                }
            } else {
                val += num;
            }
            offset += 1;
        }
        sum += val;
    }

    println!("Total sum part 2: {}", sum);
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = read_file("test.txt");
        let val = part_1(input.unwrap());
        assert_eq!(val, 4277556);
    }

    #[test]
    fn test_part_2() {
        let input = read_file("test.txt");
        let val = part_2(input.unwrap());
        assert_eq!(val, 3263827);
    }
}
