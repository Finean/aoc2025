use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input = read_file("input.txt");
    part_1(input.unwrap());
    //0.110ms
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
    let lines: Vec<[i32; 2]> = input
        .iter()
        .map(|line| {
            let mut nums = line.split(',').map(|num| num.parse::<i32>().unwrap());
            [nums.next().unwrap(), nums.next().unwrap()]
        })
        .collect();

    let mut bound: u32 = 0;
    let mut max_area: u64 = 0;

    for (ix, point) in lines.iter().enumerate() {
        for tp in ix + 1..lines.len() {
            let trial = lines[tp];
            let dx = 1 + (trial[0] - point[0]).abs() as u32;
            let dy = 1 + (trial[1] - point[1]).abs() as u32;
            if dx <= bound && dy <= bound {
                continue;
            }
            let area = dx as u64 * dy as u64;
            if area > max_area {
                max_area = area;
                bound = max_area.isqrt() as u32;
            }
        }
    }
    let elapsed = now.elapsed();
    println!("{:.2?}", elapsed);
    println!("Area of largest rectangle: {}", max_area);
    max_area
}

use std::collections::HashSet;

fn part_2(input: Vec<String>) -> u64 {
    use std::time::Instant;
    let now = Instant::now();
    //Max dimension of array
    const MAX_COORD: u32 = 100000;

    //convert to Vec<[i32; 2]>
    let lines: Vec<[i32; 2]> = input
        .iter()
        .map(|line| {
            let mut nums = line.split(',').map(|num| num.parse::<i32>().unwrap());
            [nums.next().unwrap(), nums.next().unwrap()]
        })
        .collect();

    // Create HashSet for O(1) lookups
    let lines_set: HashSet<[i32; 2]> = lines.iter().copied().collect();

    let n_points = lines.len();
    let mut edges: Vec<u64> = vec![];
    let mut horiz: Vec<u64> = vec![];
    for (ix, point) in lines.iter().enumerate() {
        let next_point: [i32; 2];
        if ix == n_points - 1 {
            next_point = lines[0];
        } else {
            next_point = lines[ix + 1];
        }
        if point[0] == next_point[0] {
            let y_start = point[1].min(next_point[1]);
            let y_end = point[1].max(next_point[1]);
            for y in y_start..=y_end {
                edges.push(lincoord(point[0], y, MAX_COORD));
            }
        } else {
            let x_start = point[0].min(next_point[0]);
            let x_end = point[0].max(next_point[0]);
            for x in x_start..=x_end {
                horiz.push(lincoord(point[1], x, MAX_COORD));
            }
        }
    }

    edges.sort();
    horiz.sort();
    //Sort by key (a, _)
    let mut bound: u32 = 4;
    let mut max_area: u64 = 0;

    for (ix, point) in lines.iter().enumerate() {
        for tp in ix + 1..n_points {
            let trial = lines[tp];
            let dx = 1 + (trial[0] - point[0]).abs() as u32;
            let dy = 1 + (trial[1] - point[1]).abs() as u32;
            if dx <= bound && dy <= bound {
                continue;
            }
            let area = dx as u64 * dy as u64;
            if area > max_area {
                //And check other corners!
                //points are point[0]trial[1], point[1]trial[0]
                if valid_rect(point, &trial, &edges, &horiz, &lines_set) {
                    max_area = area;
                    bound = max_area.isqrt() as u32;
                }
            }
        }
    }
    let elapsed = now.elapsed();
    println!("{:.2?}", elapsed);
    println!("Area of largest rectangle: {}", max_area);
    max_area
}

#[inline]
fn lincoord(x: i32, y: i32, max_coord: u32) -> u64 {
    (y as u64) * (max_coord as u64) + (x as u64)
}

#[inline]
fn fromlin(x: u64, max_coord: u32) -> [i32; 2] {
    let dx = x % max_coord as u64;
    let dy = x / max_coord as u64;
    [dx as i32, dy as i32]
}

#[inline]
fn valid_rect(
    px: &[i32; 2],
    py: &[i32; 2],
    edges: &[u64],
    horiz: &[u64],
    lines: &HashSet<[i32; 2]>,
) -> bool {
    //px and py assumed
    //horiz has reversed indices (y, x)
    let x_start = 1 + px[0].min(py[0]);
    let x_end = px[0].max(py[0]);
    let y_start = 1 + px[1].min(py[1]);
    let y_end = px[1].max(py[1]);
    if (valid_horiz(x_start, x_end, y_start, false, edges, lines)
        && valid_horiz(x_start, x_end, y_end, true, edges, lines))
        && (valid_vert(y_start, y_end, x_start, true, horiz, lines)
            && valid_vert(y_start, y_end, x_end, false, horiz, lines))
    {
        return true;
    }
    false
}

fn valid_horiz(
    x_start: i32,
    x_end: i32,
    y: i32,
    top: bool,
    edges: &[u64],
    lines: &HashSet<[i32; 2]>,
) -> bool {
    const MAX_COORD: u32 = 100000;
    let linc = lincoord(x_start, y, MAX_COORD);
    let mut eix = edges.binary_search(&linc).unwrap_or_else(|pos| pos);
    //Check along non-corner part of horizontal edge of rectangle
    let mut interior = true;
    let offset = if top { -1 } else { 1 };
    while eix < edges.len() {
        let [cur_x, cur_y] = fromlin(edges[eix], MAX_COORD);
        if cur_y != y || cur_x >= x_end {
            break;
        }
        if cur_x >= x_start {
            if lines.contains(&[cur_x, y]) {
                let new_linc = lincoord(cur_x, y + offset, MAX_COORD);
                if edges.binary_search(&new_linc).is_ok() {
                    interior = !interior;
                }
            } else {
                interior = !interior;
            }

            if !interior {
                return false;
            }
        }
        eix += 1;
    }
    true
}

fn valid_vert(
    y_start: i32,
    y_end: i32,
    x: i32,
    left: bool,
    horiz: &[u64],
    lines: &HashSet<[i32; 2]>,
) -> bool {
    const MAX_COORD: u32 = 100000;
    let invc = lincoord(y_start, x, MAX_COORD);
    let mut hix = horiz.binary_search(&invc).unwrap_or_else(|pos| pos);
    //Check along non-corner part of vertical edge of rectangle
    let mut interior = true;
    let offset = if left { 1 } else { -1 };
    while hix < horiz.len() {
        let [cur_y, cur_x] = fromlin(horiz[hix], MAX_COORD);

        // Early exit conditions
        if cur_x != x || cur_y >= y_end {
            break;
        }

        if cur_y >= y_start {
            if lines.contains(&[x, cur_y]) {
                let new_linc = lincoord(cur_y, x + offset, MAX_COORD);
                if horiz.binary_search(&new_linc).is_ok() {
                    interior = !interior;
                }
            } else {
                interior = !interior;
            }

            if !interior {
                return false;
            }
        }

        hix += 1;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = read_file("test.txt");
        let val = part_1(input.unwrap());
        assert_eq!(val, 50);
    }

    #[test]
    fn test_part_2() {
        let input = read_file("test.txt");
        let val = part_2(input.unwrap());
        assert_eq!(val, 24);
    }
}
