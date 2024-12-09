use anyhow::Result;
use itertools::{repeat_n, Itertools};
use std::fs;

pub fn solve() -> Result<()> {
    let input = fs::read_to_string("inputs/day07.txt")?;

    // Part 1
    let result1 = solve_part1(&input);
    println!("Part 1: {}", result1);

    // Part 2
    let result2 = solve_part2(&input);
    println!("Part 2: {}", result2);

    Ok(())
}

fn parse_input(input: &str) -> Vec<(u64, Vec<u64>)> {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(':').unwrap();
            (
                left.parse().unwrap(),
                right
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect(),
            )
        })
        .collect()
}

const POSSIBLE_OPERATORS: &[char] = &['+', '*'];
const POSSIBLE_OPERATORS_2: &[char] = &['+', '*', '|'];

fn test_combination(expected: u64, numbers: &[u64], operators: &[&char]) -> bool {
    let mut operands = numbers.iter().rev().collect::<Vec<_>>();
    let total = operators
        .iter()
        .fold(operands.pop().unwrap().to_owned(), |acc, op| match op {
            '+' => acc + operands.pop().unwrap(),
            '*' => acc * operands.pop().unwrap(),
            '|' => (acc.to_string() + operands.pop().unwrap().to_string().as_str())
                .parse()
                .unwrap(),
            _ => acc,
        });

    total == expected
}

fn find_combination(total: u64, numbers: &[u64], operators: &[char]) -> bool {
    let operators = repeat_n(operators, numbers.len() - 1)
        .multi_cartesian_product()
        .collect::<Vec<_>>();
    operators
        .iter()
        .any(|ops| test_combination(total, numbers, ops))
}

fn solve_part1(input: &str) -> u64 {
    let data = parse_input(input);
    data.iter()
        .filter_map(|(total, numbers)| {
            find_combination(*total, &numbers, POSSIBLE_OPERATORS).then_some(total)
        })
        .sum()
}

fn solve_part2(input: &str) -> u64 {
    let data = parse_input(input);
    data.iter()
        .filter_map(|(total, numbers)| {
            find_combination(*total, &numbers, POSSIBLE_OPERATORS_2).then_some(total)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(TEST_INPUT), 3749);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT), 11387);
    }
}
