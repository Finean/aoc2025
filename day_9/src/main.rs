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
            let nums: Vec<i32> = line
                .split(',')
                .map(|num| num.parse::<i32>().unwrap())
                .collect();
            nums.try_into().unwrap() // convert Vec<i32> into [i32; 3]
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

fn part_2(input: Vec<String>) -> u64 {
    use std::time::Instant;
    let now = Instant::now();
    //Max dimension of array
    let range = 100000;

    //convert to Vec<[i32; 2]>
    let lines: Vec<[i32; 2]> = input
        .iter()
        .map(|line| {
            let nums: Vec<i32> = line
                .split(',')
                .map(|num| num.parse::<i32>().unwrap())
                .collect();
            nums.try_into().unwrap() // convert Vec<i32> into [i32; 3]
        })
        .collect();

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
                edges.push(lincoord(point[0], y, range));
            }
        } else {
            let x_start = point[0].min(next_point[0]);
            let x_end = point[0].max(next_point[0]);
            for x in x_start..=x_end {
                horiz.push(lincoord(point[1], x, range));
            }
        }
    }

    edges.sort();
    horiz.sort();
    //Sort by key (a, _)
    let mut bound: u32 = 2;
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
                if valid_rect(point, &trial, &edges, &horiz, &lines) {
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

fn lincoord(x: i32, y: i32, max_coord: u32) -> u64 {
    (y as u64) * (max_coord as u64) + (x as u64)
}

fn fromlin(x: u64, max_coord: u32) -> [i32; 2] {
    let dx = x % max_coord as u64;
    let dy = x / max_coord as u64;
    [dx as i32, dy as i32]
}

fn valid_rect(
    px: &[i32; 2],
    py: &[i32; 2],
    edges: &Vec<u64>,
    horiz: &Vec<u64>,
    lines: &Vec<[i32; 2]>,
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
    edges: &Vec<u64>,
    lines: &Vec<[i32; 2]>,
) -> bool {
    let max_coord = 100000;
    let linc = lincoord(x_start, y, max_coord);
    let lbix: usize;
    match edges.binary_search_by(|x| x.cmp(&linc)) {
        Ok(pos) => lbix = pos,
        Err(pos) => lbix = pos,
    };
    let mut interior = true;
    let mut eix = lbix;
    loop {
        if eix >= edges.len() {
            break;
        }
        let [cur_x, cur_y] = fromlin(edges[eix], max_coord);
        if cur_y != y || cur_x >= x_end {
            break;
        }
        if cur_x < x_start {
            eix += 1;
            continue;
        } else {
            if lines.contains(&[cur_x, y]) {
                let t = if top { -1 } else { 1 };
                let new_linc = lincoord(cur_x, y + t, max_coord);
                match edges.binary_search_by(|x| x.cmp(&new_linc)) {
                    Ok(_) => interior = !interior,
                    Err(_) => {}
                }
            } else {
                interior = !interior;
            }
        }
        if !interior {
            return false;
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
    horiz: &Vec<u64>,
    lines: &Vec<[i32; 2]>,
) -> bool {
    let max_coord = 100000;
    let invc = lincoord(y_start, x, max_coord);
    let lbix: usize;
    match horiz.binary_search_by(|x| x.cmp(&invc)) {
        Ok(pos) => lbix = pos,
        Err(pos) => lbix = pos,
    };
    //Check first point on line
    let mut interior = true;
    let mut hix = lbix;
    loop {
        if hix >= horiz.len() {
            break;
        }
        let [cur_y, cur_x] = fromlin(horiz[hix], max_coord);
        if cur_x != x || cur_y >= y_end {
            break;
        }
        if cur_y < y_start {
            hix += 1;
            continue;
        } else {
            if lines.contains(&[x, cur_y]) {
                let t = if left { 1 } else { -1 };
                let new_linc = lincoord(cur_y, x + t, max_coord);
                match horiz.binary_search_by(|x| x.cmp(&new_linc)) {
                    Ok(_) => interior = !interior,
                    Err(_) => {}
                }
            } else {
                interior = !interior;
            }
        }
        if !interior {
            return false;
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
