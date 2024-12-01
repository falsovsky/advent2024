use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub fn read_input(path: PathBuf) -> (Vec<u32>, Vec<u32>) {
    let buffer = fs::read_to_string(path).expect("Path not found");
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
    // Clone vectors to allow sorting
    let mut left_values = program.0.clone();
    let mut right_values = program.1.clone();

    // Use sort_unstable() for better performance
    left_values.sort_unstable();
    right_values.sort_unstable();

    // Iterate both vectors and calculate the result
    left_values
        .iter()
        .zip(right_values.iter())
        .fold(0, |result, (&left, &right)| {
            result
                + match left.cmp(&right) {
                    Ordering::Greater => left - right,
                    Ordering::Less => right - left,
                    Ordering::Equal => 0,
                }
        })
}

pub fn solve_part2(program: &(Vec<u32>, Vec<u32>)) -> u32 {
    // Create a similarity score HashMap to avoid multiple computations for
    // the same number
    let mut similarity = HashMap::new();
    for &value in &program.1 {
        *similarity.entry(value).or_insert(0) += 1;
    }

    // Iterate through the left values
    program.0.iter().fold(0, |result, &left| {
        // Check if the value exists in the similarity score
        if let Some(&right_count) = similarity.get(&left) {
            result + left * right_count
        } else {
            result
        }
    })
}
