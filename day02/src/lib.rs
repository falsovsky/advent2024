use std::cmp::Ordering;
use std::fs;
use std::path::PathBuf;

pub fn read_input(path: PathBuf) -> Vec<Vec<u32>> {
    let buffer = fs::read_to_string(path).expect("Path not found");
    buffer
        .lines()
        .filter_map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<u32>())
                .collect::<Result<Vec<_>, _>>()
                .ok()
        })
        .collect()
}

pub fn solve_part1(program: &[Vec<u32>]) -> u32 {
    program.iter().filter(|values| check_levels(values)).count() as u32
}

pub fn solve_part2(program: &[Vec<u32>]) -> u32 {
    program
        .iter()
        .map(|values| {
            if check_levels(values) {
                1
            } else {
                values
                    .iter()
                    .enumerate()
                    .find_map(|(i, _)| {
                        let mut new_values = values.clone();
                        new_values.remove(i);
                        if check_levels(&new_values) {
                            Some(1)
                        } else {
                            None
                        }
                    })
                    .unwrap_or(0)
            }
        })
        .sum()
}

fn check_levels(values: &[u32]) -> bool {
    let mut increasing = true;
    let mut decreasing = true;

    for window in values.windows(2) {
        match window[0].cmp(&window[1]) {
            Ordering::Greater => {
                if window[0] - window[1] > 3 {
                    return false;
                }
                increasing = false;
            }
            Ordering::Less => {
                if window[1] - window[0] > 3 {
                    return false;
                }
                decreasing = false;
            }
            Ordering::Equal => return false,
        }
    }

    (increasing || decreasing) && !(increasing && decreasing)
}
