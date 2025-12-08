use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    use std::time::Instant;
    let now = Instant::now();
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut ctr = 0;
    let mut ctr_2 = 0;
    let mut pos = 50;

    for line_result in reader.lines() {
        let line = line_result?; //Result<String, io::Error>
        let first_char = line.chars().next().unwrap();
        let number_str = &line[1..];
        let number: i32 = number_str.parse().unwrap();

        //Number of full rotations
        ctr_2 += number / 100;

        let mut pos_1 = pos;

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

        if pos == 0 {
            ctr += 1;
        }
    }

    let elapsed = now.elapsed();
    println!("{:.2?}", elapsed);
    println!("Part 1: {}", ctr);
    println!("Part 2: {}", ctr_2);
    Ok(())
}
