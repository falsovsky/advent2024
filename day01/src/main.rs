use ::day01::*;
use std::env;

fn main() {
    let mut root = env::current_dir().unwrap();
    root.push("../input/day01.txt");
    let input = read_input(root);
    let pt1 = solve_part1(&input);
    println!("Part 1: {}", pt1);
    let pt2 = solve_part2(&input);
    println!("Part 2: {}", pt2);
}

#[cfg(test)]
mod day01 {
    #[test]
    fn part1() {
        use crate::*;
        let mut root = env::current_dir().unwrap();
        root.push("../input/sample01.txt");
        let input = read_input(root);
        assert_eq!(solve_part1(&input), 11);
    }

    #[test]
    fn part2() {
        use crate::*;
        let mut root = env::current_dir().unwrap();
        root.push("../input/sample01.txt");
        let input = read_input(root);
        assert_eq!(solve_part2(&input), 31);
    }
}
