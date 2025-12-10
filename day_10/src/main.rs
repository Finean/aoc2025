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
        .map(|line| {
            line.split_whitespace()
                .map(|t| {
                    t.trim_matches(|c| {
                        c == '[' || c == ']' || c == '(' || c == ')' || c == '{' || c == '}'
                    })
                })
                .collect()
        })
        .collect();
    let mut presses: u32 = 0;

    //Compute for each line
    for line in lines {
        let mut trg: u16 = 0;
        let mut ops: Vec<u16> = vec![];
        let ubound = line.len() as u16 - 2;

        //Parse line
        for (ix, elt) in line.iter().enumerate() {
            if ix == 0 {
                //Light at index i -> 2^i digit
                let n = elt
                    .chars()
                    .rev()
                    .map(|c| match c {
                        '.' => '0',
                        '#' => '1',
                        _ => '0',
                    })
                    .collect::<String>();
                trg = u16::from_str_radix(&n, 2).unwrap();
            } else if ix == line.len() - 1 {
                continue;
            } else {
                let digits: u16 = elt
                    .split(',')
                    .map(|c| c.trim().parse::<u16>().unwrap())
                    .map(|x| 2u16 << x)
                    .sum::<u16>()
                    >> 1;
                ops.push(digits);
            }
        }
        presses += search_ops(&ops, trg, ubound) as u32;
    }

    let elapsed = now.elapsed();
    println!("{:.2?}", elapsed);
    println!("Minimum number of presses: {}", presses);
    presses as u64
}

fn part_2(input: Vec<String>) -> u64 {
    use std::time::Instant;
    let now = Instant::now();

    //convert to Vec<[i32; 2]>
    let lines: Vec<Vec<&str>> = input
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|t| {
                    t.trim_matches(|c| {
                        c == '[' || c == ']' || c == '(' || c == ')' || c == '{' || c == '}'
                    })
                })
                .collect()
        })
        .collect();
    let mut presses: u32 = 0;

    //Compute for each line
    for line in lines {
        let dim = line[0].len();
        let mut trg: Vec<u16> = Vec::with_capacity(dim);
        let mut ops: Vec<Vec<u16>> = vec![];

        //Parse line
        for (ix, elt) in line.iter().enumerate() {
            if ix == 0 {
                continue;
            } else if ix == line.len() - 1 {
                trg = elt
                    .split(',')
                    .map(|x| x.parse::<u16>())
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap();
            } else {
                let indices: Vec<u16> = elt.split(',').map(|x| x.parse::<u16>().unwrap()).collect();
                let mut v: Vec<u16> = vec![0u16; dim];
                for i in indices {
                    v[i as usize] = 1;
                }
                ops.push(v);
            }
        }
        let ox = find_min_ilp(&ops, trg, dim);
        if let Some(x) = ox {
            presses += x as u32;
        } else {
            println!("Error in find_min_ilp");
        }
    }

    let elapsed = now.elapsed();
    println!("{:.2?}", elapsed);
    println!("Minimum number of presses: {}", presses);
    presses as u64
}

fn find_min(ops: &Vec<Vec<bool>>, trg: Vec<u16>, dim: usize) -> u16 {
    let scor_mat: Vec<u16> = ops
        .iter()
        .map(|row| row.iter().filter(|&&b| b).count() as u16)
        .collect();
    let trg_score = trg.iter().sum::<u16>();
    let bounds = smallest_values(ops, &trg);
    let mut sum: u16 = 0;

    println!("{:?}", scor_mat);

    //Code goes here
    sum
}

use good_lp::{
    Expression, ProblemVariables, Solution, SolverModel, default_solver, variable, variables,
};

pub fn find_min_ilp(ops: &Vec<Vec<u16>>, trg: Vec<u16>, dim: usize) -> Option<u16> {
    assert!(
        ops.iter().all(|row| row.len() == dim),
        "ops rows must have length dim"
    );
    assert!(trg.len() == dim, "trg length must equal dim");

    // Variables: one integer variable per operator, x_i >= 0
    let mut vars = variables!();
    let x: Vec<_> = (0..ops.len())
        .map(|_| vars.add(variable().integer().min(0)))
        .collect();

    // Objective: minimize sum_i x_i
    let objective: Expression = x.iter().copied().sum();
    let mut pb = vars.minimise(objective).using(default_solver);

    // Constraints: for each coordinate j, sum_i x_i * ops[i][j] == trg[j]
    for j in 0..dim {
        let lhs: Expression = x
            .iter()
            .enumerate()
            .map(|(i, &xi)| (ops[i][j] as f64) * xi)
            .sum();
        pb = pb.with(lhs.eq(trg[j] as f64));
    }

    match pb.solve() {
        Ok(sol) => {
            // Total number of vectors used is sum_i x_i
            let total = x.iter().map(|&xi| sol.value(xi)).sum::<f64>();
            Some(total.round() as u16)
        }
        Err(_) => None,
    }
}

fn smallest_values(ops: &Vec<Vec<bool>>, trg: &Vec<u16>) -> Vec<u16> {
    ops.iter()
        .map(|x| {
            trg.iter()
                .enumerate()
                .filter_map(|(i, &val)| if x[i] { Some(val) } else { None })
                .min()
                .unwrap()
        })
        .collect()
}

fn search_ops(ops: &[u16], trg: u16, ubound: u16) -> u16 {
    let mut presses: u16 = 0;
    let mut cur_ops: u16 = 1;

    // visited states: 65536 booleans
    let mut visited = vec![false; 1 << 16];
    visited[0] = true;

    let mut cur_stats = vec![0u16];
    'outer: loop {
        let mut next_stats = Vec::new();

        for &op in &cur_stats {
            for &trial in ops {
                let new_state = op ^ trial;
                if new_state == trg {
                    presses += cur_ops;
                    break 'outer;
                }
                if !visited[new_state as usize] {
                    visited[new_state as usize] = true;
                    next_stats.push(new_state);
                }
            }
        }

        cur_ops += 1;
        if cur_ops >= ubound {
            presses += ubound;
            break;
        }

        if next_stats.is_empty() {
            presses += ubound;
            break;
        }

        cur_stats = next_stats;
    }
    presses
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = read_file("test.txt");
        let val = part_1(input.unwrap());
        assert_eq!(val, 7);
    }

    #[test]
    fn test_part_2() {
        let input = read_file("test.txt");
        let val = part_2(input.unwrap());
        assert_eq!(val, 33);
    }
}
