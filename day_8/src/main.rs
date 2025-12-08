use rayon::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::mem;

use std::collections::{HashMap, HashSet};

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<u32>,
    count: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            rank: vec![0; n],
            count: n,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]); // path compression
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let mut xr = self.find(x);
        let mut yr = self.find(y);
        if xr == yr {
            return false;
        }
        if self.rank[xr] < self.rank[yr] {
            std::mem::swap(&mut xr, &mut yr);
        }
        self.parent[yr] = xr;
        if self.rank[xr] == self.rank[yr] {
            self.rank[xr] += 1;
        }
        self.count -= 1;
        true
    }
}

fn build_circuits_from_edges(minix: &[(i64, usize, usize)]) -> Vec<Vec<usize>> {
    // Collect all nodes that appear in any edge
    let mut nodes: Vec<usize> = {
        let mut set = HashSet::new();
        for &(_, a, b) in minix {
            set.insert(a);
            set.insert(b);
        }
        set.into_iter().collect()
    };

    // Sort and compact indices so UnionFind is dense: 0..nodes.len()
    nodes.sort_unstable();
    let index_of: HashMap<usize, usize> = nodes
        .iter()
        .enumerate()
        .map(|(i, &orig)| (orig, i))
        .collect();

    // UnionFind sized to number of distinct nodes in minix
    let mut uf = UnionFind::new(nodes.len());

    // Union edges
    for &(_, a, b) in minix {
        let ia = index_of[&a];
        let ib = index_of[&b];
        uf.union(ia, ib);
    }

    // Group compact indices, then map back to original node ids
    let mut groups_map: HashMap<usize, Vec<usize>> = HashMap::new();
    for (i, &orig) in nodes.iter().enumerate() {
        let root = uf.find(i);
        groups_map.entry(root).or_default().push(orig);
    }

    groups_map.into_values().collect()
}

fn last_merge_x_coords(lines: &Vec<[i32; 3]>, minix: &[(i64, usize, usize)]) -> Option<(i32, i32)> {
    let mut uf = UnionFind::new(lines.len());

    for &(_, a, b) in minix {
        if uf.union(a, b) {
            if uf.count == 1 {
                let x1 = lines[a][0];
                let x2 = lines[b][0];
                return Some((x1, x2));
            }
        }
    }
    None
}

fn main() {
    let input = read_file("input.txt");
    part_1(input.unwrap(), 1000);
    //22ms
    let input = read_file("input.txt");
    part_2(input.unwrap());
    //39ms
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

fn part_1(input: Vec<String>, n_connections: usize) -> u64 {
    use std::time::Instant;
    let now = Instant::now();

    //convert to Vec<Vec<u32>>
    let lines: Vec<[i32; 3]> = input
        .iter()
        .map(|line| {
            let nums: Vec<i32> = line
                .split(',')
                .map(|num| num.parse::<i32>().unwrap())
                .collect();
            nums.try_into().unwrap() // convert Vec<i32> into [i32; 3]
        })
        .collect();

    //Precalculate distances - O(n^2)
    let mut distances: Vec<(i64, usize, usize)> =
        Vec::with_capacity(lines.len() * (lines.len() - 1) / 2); //Length (n-1) * (n-1)
    for ix in 0..lines.len() - 1 {
        let [px, py, pz]: [i32; 3] = lines[ix];
        for apx in ix + 1..lines.len() {
            let [ax, ay, az]: [i32; 3] = lines[apx];
            let dx: i64 = (px - ax) as i64;
            let dy: i64 = (py - ay) as i64;
            let dz: i64 = (pz - az) as i64;
            let dist = dx * dx + dy * dy + dz * dz;
            distances.push((dist, ix, apx));
        }
    }
    //Calculate minimum n_connections distances
    let mut minix: Vec<(i64, usize, usize)> = Vec::with_capacity(n_connections);

    for ix in 0..distances.len() {
        let cur_dist = distances[ix];
        if cur_dist.0 == 0 {
            continue;
        }
        if minix.len() < n_connections {
            let idx = minix
                .binary_search_by_key(&cur_dist.0, |&(a, _, _)| a)
                .unwrap_or_else(|i| i);
            minix.insert(idx, cur_dist);
        } else if cur_dist.0 < minix.last().unwrap().0 {
            let idx = minix
                .binary_search_by_key(&cur_dist.0, |&(a, _, _)| a)
                .unwrap_or_else(|i| i);
            minix.insert(idx, cur_dist);
            minix.pop();
        }
    }

    let circuits = build_circuits_from_edges(&minix);
    //println!("{:?}", circuits);
    let mut longest: Vec<&Vec<usize>> = circuits.par_iter().collect();
    longest.sort_by_key(|v| -(v.len() as isize));
    // Take top 3
    let sum: u64 = longest
        .into_par_iter()
        .take(3)
        .map(|v| v.len() as u64)
        .product();

    let elapsed = now.elapsed();
    println!("{:.2?}", elapsed);
    println!("Product of 3 largest circuits: {}", sum);
    sum
}

fn part_2(input: Vec<String>) -> i64 {
    use std::time::Instant;
    let now = Instant::now();
    let mut sum: i64 = 0;

    //convert to Vec<Vec<u32>>

    let lines: Vec<[i32; 3]> = input
        .iter()
        .map(|line| {
            let nums: Vec<i32> = line
                .split(',')
                .map(|num| num.parse::<i32>().unwrap())
                .collect();
            nums.try_into().unwrap() // convert Vec<i32> into [i32; 3]
        })
        .collect();

    //Precalculate distances - O(n^2)
    let mut distances: Vec<(i64, usize, usize)> =
        Vec::with_capacity(lines.len() * (lines.len() - 1) / 2); //Length (n-1) * (n-1)
    for ix in 0..lines.len() - 1 {
        let [px, py, pz]: [i32; 3] = lines[ix];
        for apx in ix + 1..lines.len() {
            let [ax, ay, az]: [i32; 3] = lines[apx];
            let dx: i64 = (px - ax) as i64;
            let dy: i64 = (py - ay) as i64;
            let dz: i64 = (pz - az) as i64;
            let dist = dx * dx + dy * dy + dz * dz;
            distances.push((dist, ix, apx));
        }
    }

    distances.par_sort_by_key(|&(x, _, _)| x);

    if let Some(x) = last_merge_x_coords(&lines, &distances) {
        sum = (x.0 * x.1) as i64;
    }

    let elapsed = now.elapsed();
    println!("{:.2?}", elapsed);
    println!("Product of 3 largest circuits: {}", sum);
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = read_file("test.txt");
        let val = part_1(input.unwrap(), 10);
        assert_eq!(val, 40);
    }

    #[test]
    fn test_part_2() {
        let input = read_file("test.txt");
        let val = part_2(input.unwrap());
        assert_eq!(val, 25272);
    }
}
