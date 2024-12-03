use anyhow::Result;
use std::fs;

pub fn solve() -> Result<()> {
    let input = fs::read_to_string("inputs/day02.txt")?;

    let lines = parse_input(&input);

    // Part 1
    let result1 = solve_part1(lines.clone());
    println!("Part 1: {}", result1);

    // Part 2
    let result2 = solve_part2(lines.clone());
    println!("Part 2: {}", result2);

    Ok(())
}

// Parse the input into a list of lists of integers
fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

fn validate_distance(a: u32, b: u32, comparator: fn(u32, u32) -> bool) -> bool {
    comparator(a, b) && a.abs_diff(b) <= 3
}

fn validate_with_comparator(line: &[u32], comparator: fn(u32, u32) -> bool) -> bool {
    line.windows(2)
        .all(|window| validate_distance(window[0], window[1], comparator))
}

fn validate_safety(line: &Vec<u32>) -> bool {
    let validate = |comparator| validate_with_comparator(line, comparator);
    validate(|a, b| a < b) || validate(|a, b| a > b)
}

fn validate_safety_with_tolerance(line: &Vec<u32>) -> bool {
    (0..line.len()).any(|i| {
        let mut line = line.clone();
        line.remove(i);
        validate_safety(&line)
    })
}

fn solve_part1(lines: Vec<Vec<u32>>) -> usize {
    lines.into_iter().filter(validate_safety).count()
}

fn solve_part2(lines: Vec<Vec<u32>>) -> usize {
    lines
        .iter()
        .filter(|line| validate_safety_with_tolerance(line))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    #[test]
    fn test_validate_distance() {
        assert!(validate_distance(1, 2, |a, b| a < b));
        assert!(!validate_distance(1, 2, |a, b| a > b));
        assert!(validate_distance(2, 1, |a, b| a > b));
        assert!(validate_distance(1, 3, |a, b| a < b));
        assert!(!validate_distance(1, 3, |a, b| a > b));
        assert!(validate_distance(1, 4, |a, b| a < b));
        assert!(!validate_distance(1, 5, |a, b| a < b));
    }

    #[test]
    fn test_validate_with_comparator() {
        assert!(validate_with_comparator(&vec![7, 6, 4, 2, 1], |a, b| a > b));

        assert!(!validate_with_comparator(&vec![7, 6, 4, 2, 1], |a, b| a < b));

        assert!(!validate_with_comparator(&vec![1, 2, 7, 8, 9], |a, b| a < b));

        assert!(!validate_with_comparator(&vec![1, 3, 2, 4, 5], |a, b| a > b));

        assert!(!validate_with_comparator(&vec![1, 3, 2, 4, 5], |a, b| a < b));
    }

    #[test]
    fn test_validate_safety() {
        assert!(validate_safety(&vec![1, 2, 3, 4, 5]));
        assert!(!validate_safety(&vec![1, 2, 7, 8, 9]));
        assert!(!validate_safety(&vec![92, 94, 97, 98, 97]));
    }

    #[test]
    fn test_validate_safety_with_tolerance() {
        assert!(validate_safety_with_tolerance(&vec![1, 2, 3, 4, 5]));
        assert!(!validate_safety_with_tolerance(&vec![1, 2, 7, 8, 9]));
        assert!(validate_safety_with_tolerance(&vec![92, 94, 97, 98, 97]));
        assert!(validate_safety_with_tolerance(&vec![
            26, 27, 28, 31, 33, 34, 37, 37
        ]));
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(parse_input(TEST_INPUT)), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(parse_input(TEST_INPUT)), 4);
    }
}
