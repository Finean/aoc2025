use std::cmp;
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
    let mut inputsplit: usize = 0;
    let mut fresh_ranges: Vec<Vec<u64>> = vec![];

    for (ix, line) in input.iter().enumerate() {
        if line.is_empty() {
            inputsplit = ix + 1 as usize;
            break;
        }
        let range: Vec<u64> = line.split('-').map(|x| x.parse::<u64>().unwrap()).collect();
        let ext_range: Vec<u64> = vec![range[0] - 1, range[1] + 1];
        match binrangesearch(&fresh_ranges, &ext_range) {
            Ok(mut pos) => {
                fresh_ranges[pos] = unionrange(&fresh_ranges[pos], &range);
                // merge forward
                while pos + 1 < fresh_ranges.len()
                    && ranges_overlap(&fresh_ranges[pos], &fresh_ranges[pos + 1])
                {
                    let merged = unionrange(&fresh_ranges[pos], &fresh_ranges[pos + 1]);
                    fresh_ranges[pos] = merged;
                    fresh_ranges.remove(pos + 1);
                }
                // merge backward
                while pos > 0 && ranges_overlap(&fresh_ranges[pos - 1], &fresh_ranges[pos]) {
                    let merged = unionrange(&fresh_ranges[pos - 1], &fresh_ranges[pos]);
                    fresh_ranges[pos - 1] = merged;
                    fresh_ranges.remove(pos);
                    pos -= 1;
                }
            }
            Err(pos) => {
                fresh_ranges.insert(pos, range);
                // merge forward
                if pos + 1 < fresh_ranges.len()
                    && ranges_overlap(&fresh_ranges[pos], &fresh_ranges[pos + 1])
                {
                    let merged = unionrange(&fresh_ranges[pos], &fresh_ranges[pos + 1]);
                    fresh_ranges[pos] = merged;
                    fresh_ranges.remove(pos + 1);
                }
                // merge backward
                if pos > 0 && ranges_overlap(&fresh_ranges[pos - 1], &fresh_ranges[pos]) {
                    let merged = unionrange(&fresh_ranges[pos - 1], &fresh_ranges[pos]);
                    fresh_ranges[pos - 1] = merged;
                    fresh_ranges.remove(pos);
                }
            }
        }
    }
    println!("{:?} ranges to check", fresh_ranges.len());

    for line in input[inputsplit..].iter() {
        let id = line.parse::<u64>().unwrap();
        match binrangesearch(&fresh_ranges, &vec![id, id]) {
            Ok(_pos) => sum += 1,
            Err(_pos) => {}
        }
    }
    let elapsed = now.elapsed();
    println!("{:.2?}", elapsed);
    println!("Total fresh: {}", sum);
    sum
}

fn part_2(input: Vec<String>) -> u64 {
    use std::time::Instant;
    let now = Instant::now();
    let mut sum: u64 = 0;
    let mut fresh_ranges: Vec<Vec<u64>> = vec![];

    for (_, line) in input.iter().enumerate() {
        if line.is_empty() {
            break;
        }
        let range: Vec<u64> = line.split('-').map(|x| x.parse::<u64>().unwrap()).collect();
        let ext_range: Vec<u64> = vec![range[0] - 1, range[1] + 1];
        match binrangesearch(&fresh_ranges, &ext_range) {
            Ok(mut pos) => {
                fresh_ranges[pos] = unionrange(&fresh_ranges[pos], &range);
                // merge forward
                while pos + 1 < fresh_ranges.len()
                    && ranges_overlap(&fresh_ranges[pos], &fresh_ranges[pos + 1])
                {
                    let merged = unionrange(&fresh_ranges[pos], &fresh_ranges[pos + 1]);
                    fresh_ranges[pos] = merged;
                    fresh_ranges.remove(pos + 1);
                }
                // merge backward
                while pos > 0 && ranges_overlap(&fresh_ranges[pos - 1], &fresh_ranges[pos]) {
                    let merged = unionrange(&fresh_ranges[pos - 1], &fresh_ranges[pos]);
                    fresh_ranges[pos - 1] = merged;
                    fresh_ranges.remove(pos);
                    pos -= 1;
                }
            }
            Err(pos) => {
                fresh_ranges.insert(pos, range);
                // merge forward
                if pos + 1 < fresh_ranges.len()
                    && ranges_overlap(&fresh_ranges[pos], &fresh_ranges[pos + 1])
                {
                    let merged = unionrange(&fresh_ranges[pos], &fresh_ranges[pos + 1]);
                    fresh_ranges[pos] = merged;
                    fresh_ranges.remove(pos + 1);
                }
                // merge backward
                if pos > 0 && ranges_overlap(&fresh_ranges[pos - 1], &fresh_ranges[pos]) {
                    let merged = unionrange(&fresh_ranges[pos - 1], &fresh_ranges[pos]);
                    fresh_ranges[pos - 1] = merged;
                    fresh_ranges.remove(pos);
                }
            }
        }
    }
    println!("{:?} ranges to check", fresh_ranges.len());

    for range in fresh_ranges {
        sum += 1 + range[1] - range[0];
    }
    let elapsed = now.elapsed();
    println!("{:.2?}", elapsed);
    println!("Total fresh IDs: {}", sum);
    sum
}

fn unionrange(arr_1: &Vec<u64>, arr_2: &Vec<u64>) -> Vec<u64> {
    return vec![cmp::min(arr_1[0], arr_2[0]), cmp::max(arr_1[1], arr_2[1])];
}

fn ranges_overlap(a: &Vec<u64>, b: &Vec<u64>) -> bool {
    a[0] <= b[1] && b[0] <= a[1]
}

fn binrangesearch(input: &Vec<Vec<u64>>, range: &Vec<u64>) -> Result<usize, usize> {
    if input.is_empty() {
        return Err(0);
    }
    let mut a: usize = 0;
    let mut b: usize = input.len();
    while a < b {
        let pt = (a + b) / 2;
        let r = &input[pt];
        if ranges_overlap(r, range) {
            return Ok(pt);
        } else if range[1] < r[0] {
            b = pt;
        } else {
            a = pt + 1;
        }
    }
    Err(a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = read_file("test.txt");
        let val = part_1(input.unwrap());
        assert_eq!(val, 3);
    }

    #[test]
    fn test_part_2() {
        let input = read_file("test.txt");
        let val = part_2(input.unwrap());
        assert_eq!(val, 14);
    }
}
