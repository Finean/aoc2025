use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

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
    let now = Instant::now();

    let mut output = 0;

    for line in input {
        let bank_size = line.len();

        let mut max_digit: u32 = 0;
        let mut second_digit: u32 = 0;

        for (ix, cur_char) in line.chars().enumerate() {
            let digit = u32::from(cur_char) - u32::from('0');
            if ix < (bank_size - 1) && digit > max_digit {
                max_digit = digit;
                second_digit = 0;
            } else if digit > second_digit || (ix == bank_size - 1 && second_digit == 0) {
                second_digit = digit;
            }
        }

        let tmp_val = max_digit * 10 + second_digit;
        output += tmp_val;
    }
    let elapsed = now.elapsed();
    println!("{:.2?}", elapsed);
    println!("Total output: {}", output);
    output as u64
}

fn part_2(input: Vec<String>) -> u64 {
    let now = Instant::now();

    let mut output: u64 = 0;

    for line in input {
        let bank_size = line.len();
        let mut digits: [u32; 12] = [0; 12];

        for (ix, cur_char) in line.chars().enumerate() {
            let digit = u32::from(cur_char) - u32::from('0');
            for cell in 0..12 {
                //If valid for 12 activations and larger than current digit
                if ix <= (bank_size - 12 + cell) && digit > digits[cell] {
                    digits[cell] = digit;
                    digits[cell + 1..].fill(0);
                    //Break so only most significant digit is changed
                    break;
                } else if ix == (bank_size - 12 + cell) && digits[cell] == 0 {
                    digits[cell] = digit;
                    digits[cell + 1..].fill(0);
                    break;
                }
            }
        }

        let mut result: u64 = 0;
        for x in digits {
            result *= 10;
            result += x as u64;
        }
        output += result;
    }
    let elapsed = now.elapsed();
    println!("{:.2?}", elapsed);
    println!("Total output: {}", output);
    output
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = read_file("test.txt");
        let val = part_1(input.unwrap());
        assert_eq!(val, 357);
    }

    #[test]
    fn test_part_2() {
        let input = read_file("test.txt");
        let val = part_2(input.unwrap());
        assert_eq!(val, 3121910778619);
    }
}
