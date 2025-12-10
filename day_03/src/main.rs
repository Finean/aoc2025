use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() {
    let _ = part_1("input.txt".to_string());
    let _ = part_2("input.txt".to_string());
}

fn part_1(file: String) -> io::Result<u64> {
    use std::time::Instant;
    let now = Instant::now();
    let file = File::open(file)?;
    let reader = BufReader::new(file);

    let mut output = 0;

    for line_result in reader.lines() {
        let line = line_result?; //Result<String, io::Error>
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
    Ok(output.into())
}

fn part_2(file: String) -> io::Result<u64> {
    use std::time::Instant;
    let now = Instant::now();
    let file = File::open(file)?;
    let reader = BufReader::new(file);

    let mut output: u64 = 0;

    for line_result in reader.lines() {
        let line = line_result?; //Result<String, io::Error>
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
    Ok(output.into())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let val = part_1("test.txt".to_string()).unwrap_or(0);
        assert_eq!(val, 357);
    }

    #[test]
    fn test_part_2() {
        let val = part_2("test.txt".to_string()).unwrap_or(0);
        assert_eq!(val, 3121910778619);
    }
}
