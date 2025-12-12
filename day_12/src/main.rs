use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input = read_file("input.txt");
    part_1(input.unwrap());
    let input = read_file("input.txt");
    //part_2(input.unwrap());
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

    let mut output = 0;
    let shape_sizes: [u32; 6] = [7, 5, 7, 6, 7, 7];

    for (ix, line) in lines.iter().enumerate() {
        if ix < 30 {
            continue;
        }
        let mut area: u32 = 0;
        let mut req: u32 = 0;
        for (aix, arg) in line.iter().enumerate() {
            if aix == 0 {
                let m = arg.trim_end_matches(':');
                let dim: Vec<u32> = m.split('x').map(|n| n.parse::<u32>().unwrap()).collect();
                area += (dim[0] / 3) * (dim[1] / 3);
            } else {
                req += arg.parse::<u32>().unwrap();
            }
        }
        if req <= area {
            output += 1;
        } else {
        }
    }

    let elapsed = now.elapsed();
    println!("{:.2?}", elapsed);
    println!("Number of sufficient regions: {}", output);
    output as u64
}
