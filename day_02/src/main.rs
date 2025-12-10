use rustc_hash::FxHashSet;
use std::fs;
use std::time::Instant;

fn part_1(input: &str) -> u64 {
    let now = Instant::now();
    let ranges: Vec<&str> = input.trim().split(',').collect();
    let mut sum = 0;

    for r in ranges {
        let parts: Vec<&str> = r.split('-').collect();
        if parts.len() == 2 {
            let start: u64 = parts[0].parse().unwrap();
            let end: u64 = parts[1].parse().unwrap();
            let start_len = (parts[0].len() - 1) as u32;
            let end_len = (parts[1].len() - 1) as u32;

            sum += p1_invalid_sum(start, end, start_len, end_len);
        }
    }

    let elapsed = now.elapsed();
    println!("{:.2?}", elapsed);
    println!("Part 1 Sum: {}", sum);
    sum
}

fn part_2(input: &str) -> u64 {
    let now = Instant::now();
    let ranges: Vec<&str> = input.trim().split(',').collect();
    let mut sum = 0;

    for r in ranges {
        let parts: Vec<&str> = r.split('-').collect();
        if parts.len() == 2 {
            let start: u64 = parts[0].parse().unwrap();
            let end: u64 = parts[1].parse().unwrap();
            let start_len = (parts[0].len() - 1) as u32;
            let end_len = (parts[1].len() - 1) as u32;

            sum += p2_invalid_sum(start, end, start_len, end_len);
        }
    }
    let elapsed = now.elapsed();
    println!("{:.2?}", elapsed);
    println!("Part 2 Sum: {}", sum);
    sum
}

fn main() -> std::io::Result<()> {
    let content = fs::read_to_string("input.txt")?;
    let _sum_1 = part_1(&content);
    let _sum_2 = part_2(&content);
    Ok(())
}

fn p1_invalid_sum(min: u64, max: u64, base_min: u32, base_max: u32) -> u64 {
    let mut sum: u64 = 0;

    if base_min == base_max {
        if base_min % 2 == 0 {
            return 0;
        }
        let i_min = min / 10u64.pow(1 + (base_min / 2));
        let i_max = max / 10u64.pow(1 + (base_max / 2));

        if i_min == i_max {
            let val = i_min * 10u64.pow(1 + base_min / 2) + i_min;
            if min <= val && val <= max {
                return val;
            } else {
                return 0;
            }
        } else {
            sum += sum_inv(i_min, i_max, base_min);
        }

        let val_min = i_min * 10u64.pow(1 + base_min / 2) + i_min;
        let val_max = i_max * 10u64.pow(1 + base_max / 2) + i_max;

        if val_min < min {
            sum -= val_min;
        }
        if val_max > max {
            sum -= val_max;
        }
        return sum;
    }

    for base in base_min..=base_max {
        if base % 2 == 0 {
            continue;
        }
        if base == base_min {
            let i_min = min / 10u64.pow(1 + base_min / 2);
            let i_max = 10u64.pow(1 + base / 2) - 1;
            sum += sum_inv(i_min, i_max, base);
        } else if base == base_max {
            let i_min = 10u64.pow(base / 2);
            let i_max = max / 10u64.pow(1 + base_max / 2);
            sum += sum_inv(i_min, i_max, base);
        } else {
            let i_min = 10u64.pow(base / 2);
            let i_max = i_min * 10 - 1;
            sum += sum_inv(i_min, i_max, base);
        }
    }
    sum
}

fn sum_inv(min: u64, max: u64, base: u32) -> u64 {
    let mut sum: u64 = 0;
    for i in min..=max {
        let val = i * 10u64.pow(base / 2 + 1) + i;
        sum += val;
    }
    sum
}

fn p2_invalid_sum(min: u64, max: u64, base_min: u32, base_max: u32) -> u64 {
    let mut sum: u64 = 0;
    let mut seen = FxHashSet::default();

    for base in base_min..=base_max {
        //Divisors to check
        let divs = chunk_count(&base);
        for divisor in divs {
            let str_len = (base + 1) / divisor;
            let min_val = 10u64.pow(str_len - 1);
            let max_val = 10u64.pow(str_len) - 1;

            for val in min_val..=max_val {
                let int: u64 = reconstruct(val, divisor);
                if int > max {
                    break;
                }
                if int < min {
                    continue;
                }
                if seen.insert(int) {
                    sum += int
                }
            }
        }
    }
    sum
}

fn reconstruct(val: u64, divi: u32) -> u64 {
    let mut sum: u64 = 0;
    let base = val.to_string().len() as u32;
    for _ in 0..divi {
        sum *= 10u64.pow(base) as u64;
        sum += val;
    }
    sum
}

fn chunk_count(base: &u32) -> Vec<u32> {
    //Returns possible lengths of sequences to check
    let mut divs = Vec::new();
    //Number of digits in val
    let length = (base + 1) as u32;

    for i in 2..=length {
        //If i divides length
        if (base + 1) % i == 0 {
            divs.push(i);
        }
    }
    divs
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part_1_with_test_file() {
        let content = fs::read_to_string("test.txt").expect("Failed to read test.txt");
        let result = part_1(&content);
        assert_eq!(result, 1227775554);
    }

    #[test]
    fn test_part_2_with_test_file() {
        let content = fs::read_to_string("test.txt").expect("Failed to read test.txt");
        let result = part_2(&content);
        assert_eq!(result, 4174379265);
    }
}
