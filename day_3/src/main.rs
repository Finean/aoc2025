use std::fs::File;
use std::io::{self, BufRead, BufReader};



fn main() {
    part_1();
    part_2();
}


fn part_1() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut output = 0;
    
    for line_result in reader.lines() {
        let line = line_result?; //Result<String, io::Error>
        let bank_size = line.len();
        
        println!("{}", line);
        let mut max_digit: u32 = 0;
        let mut second_digit: u32 = 0;

        for (ix, cur_char) in line.chars().enumerate() {
            let digit = u32::from(cur_char) - u32::from('0');
            if ix < (bank_size - 1) && digit > max_digit {
                max_digit = digit;
                second_digit = 0;
            }
            else if digit > second_digit || (ix == bank_size - 1 && second_digit == 0) {
                second_digit = digit;
            }

        }

        let tmp_val = max_digit * 10 + second_digit;
        println!("{}, {}, {}", max_digit, second_digit, tmp_val);
        output += tmp_val;

    }

    println!("Total output: {}", output);
    Ok(())

}


fn part_2() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut output: u64 = 0;


    for line_result in reader.lines() {
        let line = line_result?; //Result<String, io::Error>
        let bank_size = line.len();
        
        println!("{}", line);

        let mut digits: [u32; 12]  = [0; 12];
        let mut max_digit: u32 = 0;
        let mut second_digit: u32 = 0;

        for (ix, cur_char) in line.chars().enumerate() {
            let digit = u32::from(cur_char) - u32::from('0');
            for cell in 0..12 {
                //If valid for 12 activations and larger than current digit
                if ix <= (bank_size - 12 + cell) && digit > digits[cell] {
                    digits[cell] = digit;
                    digits[cell+1..].fill(0);
                    //Break so only most significant digit is changed
                    break;
                } else if ix == (bank_size - 12 + cell) && digits[cell] == 0 {
                    digits[cell] = digit;
                    digits[cell+1..].fill(0);
                    break;
                }
            }

        }

        
        let mut result: u64 = 0;
        for x in digits {
            result *= 10;
            result += x as u64;
        }
        println!("{:?}, {}", digits, result);
        output += result;

    }

    println!("Total output: {}", output);
    Ok(())

}