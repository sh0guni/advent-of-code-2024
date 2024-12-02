use anyhow::Result;
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
fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let lists = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    let lists: Vec<Vec<i32>> = (0..2)
        .map(|i| lists.iter().map(|list| list[i]).collect::<Vec<i32>>())
        .collect::<Vec<Vec<i32>>>();
    (lists[0].clone(), lists[1].clone())
}

fn solve_part1((mut list1, mut list2): (Vec<i32>, Vec<i32>)) -> i32 {
    list1.sort();
    list2.sort();
    list1.iter().zip(list2.iter()).map(|(a, b)| (a - b).abs()).sum()
}

fn solve_part2((list1, list2): (Vec<i32>, Vec<i32>)) -> i32 {
    list1.iter().map(|a| list2.iter().filter(|b| a == *b).sum::<i32>()).sum()
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
