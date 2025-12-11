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
    let output = count_paths("you", "out", &lines, &keymap);

    let elapsed = now.elapsed();
    println!("{:.2?}", elapsed);
    println!("Number of paths: {}", output);
    output as u64
}

fn part_2(input: Vec<String>) -> u64 {
    use std::time::Instant;
    let now = Instant::now();

    let lines: Vec<Vec<&str>> = input
        .iter()
        .map(|line| line.split_whitespace().collect())
        .collect();

    let mut keymap: HashMap<String, usize> = HashMap::new();

    //Compute hashmap
    for (ix, line) in lines.iter().enumerate() {
        let key = line[0].strip_suffix(":").unwrap();
        keymap.insert(key.to_string(), ix as usize);
    }
    let mut dac = count_paths("dac", "fft", &lines, &keymap);
    if dac != 0 {
        dac *= count_paths("svr", "dac", &lines, &keymap);
        dac *= count_paths("fft", "out", &lines, &keymap);
    }
    let mut fft = count_paths("fft", "dac", &lines, &keymap);
    if fft != 0 {
        fft *= count_paths("svr", "fft", &lines, &keymap);
        fft *= count_paths("dac", "out", &lines, &keymap);
    }

    let output = fft + dac;

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
            sum += search_trg(dest, end, lines, keymap, seen);
        }
    }
    seen[idx] = sum;
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

    #[test]
    fn test_part_2() {
        let input = read_file("test2.txt");
        let val = part_2(input.unwrap());
        assert_eq!(val, 2);
    }
}
