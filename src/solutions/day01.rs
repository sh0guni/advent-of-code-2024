use anyhow::Result;
use itertools::Itertools;
use std::fs;

pub fn solve() -> Result<()> {
    let input = fs::read_to_string("inputs/day01.txt")?;

    let (list1, list2) = parse_input(&input);

    // Part 1
    let result1 = solve_part1((list1.clone(), list2.clone()));
    println!("Part 1: {}", result1);

    // Part 2
    let result2 = solve_part2((list1, list2));
    println!("Part 2: {}", result2);

    Ok(())
}

// Parse the input into two lists of integers
fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    let lists = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    let lists: Vec<Vec<u32>> = (0..2)
        .map(|i| lists.iter().map(|list| list[i]).collect::<Vec<u32>>())
        .collect::<Vec<Vec<u32>>>();
    (lists[0].clone(), lists[1].clone())
}

fn solve_part1((list1, list2): (Vec<u32>, Vec<u32>)) -> u32 {
    list1
        .into_iter()
        .sorted()
        .zip(list2.into_iter().sorted())
        .map(|(a, b)| a.abs_diff(b))
        .sum()
}

fn solve_part2((list1, list2): (Vec<u32>, Vec<u32>)) -> u32 {
    list1
        .iter()
        .map(|a| list2.iter().filter(|b| a == *b).sum::<u32>())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(parse_input(TEST_INPUT)), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(parse_input(TEST_INPUT)), 31);
    }
}
