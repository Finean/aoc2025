use std::collections::HashMap;
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

    //convert to Vec<[i32; 2]>
    let lines: Vec<Vec<&str>> = input
        .iter()
        .map(|line| line.split_whitespace().collect())
        .collect();

    let mut seen: Vec<u16> = vec![0u16; lines.len()];
    let mut keymap: HashMap<String, usize> = HashMap::new();
    let mut you_idx: usize = 0;

    //Compute hashmap
    for (ix, line) in lines.iter().enumerate() {
        if line[0] == "you:" {
            you_idx = ix;
        } else if line.last().unwrap() == &"out" {
            seen[ix] = 1;
        }
        let key = line[0].replace(":", "");
        keymap.insert(key, ix as usize);
    }

    let output = search(&you_idx, &keymap, &lines, &mut seen);

    let elapsed = now.elapsed();
    println!("{:.2?}", elapsed);
    println!("Number of paths: {}", output);
    output as u64
}

fn part_2(input: Vec<String>) -> u64 {
    use std::time::Instant;
    let now = Instant::now();

    //convert to Vec<[i32; 2]>
    let lines: Vec<Vec<&str>> = input
        .iter()
        .map(|line| line.split_whitespace().collect())
        .collect();

    let mut keymap: HashMap<String, usize> = HashMap::new();

    //Compute hashmap
    for (ix, line) in lines.iter().enumerate() {
        let key = line[0].replace(":", "");
        keymap.insert(key, ix as usize);
    }
    let dac = count_paths("svr", "dac", &lines, &keymap);
    let fft = count_paths("svr", "fft", &lines, &keymap);
    let df = count_paths("dac", "fft", &lines, &keymap);
    let fd = count_paths("fft", "dac", &lines, &keymap);
    let fout = count_paths("fft", "out", &lines, &keymap);
    let dout = count_paths("dac", "out", &lines, &keymap);
    let output = dac * df * fout + fft * fd * dout;

    let elapsed = now.elapsed();
    println!("{:.2?}", elapsed);
    println!("Number of paths: {}", output);
    output as u64
}

fn count_paths(
    start: &str,
    end: &str,
    lines: &Vec<Vec<&str>>,
    keymap: &HashMap<String, usize>,
) -> i64 {
    let mut seen: Vec<i64> = vec![-1; lines.len()];
    return search_trg(start, end, lines, keymap, &mut seen);
}

fn search_trg(
    start: &str,
    end: &str,
    lines: &Vec<Vec<&str>>,
    keymap: &HashMap<String, usize>,
    seen: &mut Vec<i64>,
) -> i64 {
    let idx = *keymap.get(start).unwrap() as usize;
    if seen[idx] >= 0 {
        return seen[idx];
    }
    let mut sum: i64 = 0;
    for (ax, dest) in lines[idx].iter().enumerate() {
        if ax == 0 {
            continue;
        }
        if dest == &"out" && end != "out" {
            seen[idx] = 0;
            return 0;
        }
        if dest == &end {
            sum += 1;
        } else {
            sum += search_trg(&dest, end, lines, keymap, seen);
        }
    }
    seen[idx] = sum;
    return sum;
}

fn search(
    idx: &usize,
    keymap: &HashMap<String, usize>,
    lines: &Vec<Vec<&str>>,
    seen: &mut Vec<u16>,
) -> u16 {
    if seen[*idx] != 0 {
        return seen[*idx];
    }
    let mut sum: u16 = 0;
    for (ax, dest) in lines[*idx].iter().enumerate() {
        if ax == 0 {
            continue;
        }
        if let Some(kix) = keymap.get(*dest) {
            sum += search(kix, keymap, lines, seen);
        } else {
            println!("Key error, key: {}", dest);
        }
    }
    seen[*idx] = sum;
    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = read_file("test.txt");
        let val = part_1(input.unwrap());
        assert_eq!(val, 5);
    }
    /*
    #[test]
    fn test_part_2() {
        let input = read_file("test2.txt");
        let val = part_2(input.unwrap());
        assert_eq!(val, 2);
    }
    */
}
