use std::cmp::Ordering;
use std::fs;
use std::path::PathBuf;

pub fn read_input(path: PathBuf) -> (Vec<u32>, Vec<u32>) {
    let buffer = fs::read_to_string(path).unwrap();
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();
    for line in buffer.lines() {
        let values: Vec<_> = line.split_ascii_whitespace().collect();
        left.push(values[0].parse().expect("Not a number"));
        right.push(values[1].parse().expect("Not a number"));
    }
    (left, right)
}

pub fn solve_part1(program: &(Vec<u32>, Vec<u32>)) -> u32 {
    let mut ls = program.0.clone();
    let mut rs = program.1.clone();
    ls.sort_unstable();
    rs.sort_unstable();
    let mut result: u32 = 0;
    for (l, r) in ls.into_iter().zip(rs.into_iter()) {
        result += match l.cmp(&r) {
            Ordering::Greater => l - r,
            Ordering::Less => r - l,
            Ordering::Equal => 0,
        }
    }
    result
}

pub fn solve_part2(program: &(Vec<u32>, Vec<u32>)) -> u32 {
    let mut result: u32 = 0;
    for &l in &program.0 {
        let count = program.1.iter().filter(|&&val| val == l).count() as u32;
        result += l * count;
    }
    result
}
