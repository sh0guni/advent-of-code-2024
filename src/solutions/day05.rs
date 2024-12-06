use anyhow::Result;
use itertools::Itertools;
use std::{cmp::Ordering, collections::HashMap, fs};

pub fn solve() -> Result<()> {
    let input = fs::read_to_string("inputs/day05.txt")?;

    // Part 1
    let result1 = solve_part1(&input);
    println!("Part 1: {}", result1);

    // Part 2
    let result2 = solve_part2(&input);
    println!("Part 2: {}", result2);

    Ok(())
}

fn parse_input(input: &str) -> (HashMap<usize, Vec<usize>>, Vec<Vec<usize>>) {
    let (ordering, updates) = input.split_once("\n\n").unwrap();
    let ordering = ordering
        .lines()
        .map(|line| line.split_once("|").unwrap())
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .fold(HashMap::new(), |mut map, (a, b)| {
            map.entry(b).or_insert(vec![]).push(a);
            map
        });
    let pages = updates
        .lines()
        .map(|line| line.split(",").map(|s| s.parse().unwrap()).collect())
        .collect();
    (ordering, pages)
}

fn is_valid(ordering: &HashMap<usize, Vec<usize>>, update: &[usize]) -> bool {
    update.iter().enumerate().all(|(i, &n)| {
        ordering
            .get(&n)
            .is_none_or(|orders| update[i + 1..].iter().all(|m| !orders.contains(m)))
    })
}

fn sort_invalid_update(ordering: &HashMap<usize, Vec<usize>>, update: Vec<usize>) -> Vec<usize> {
    update
        .into_iter()
        .sorted_by(|a, b| {
            let orders = ordering.get(&a);
            match orders {
                None => Ordering::Equal,
                Some(orders) => {
                    if orders.contains(b) {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                }
            }
        })
        .collect()
}

fn solve_part1(input: &str) -> usize {
    let (ordering, updates) = parse_input(input);
    updates
        .into_iter()
        .filter(|update| is_valid(&ordering, &update))
        .map(|update| update[update.len() / 2])
        .sum()
}

fn solve_part2(input: &str) -> usize {
    let (ordering, updates) = parse_input(input);
    updates
        .into_iter()
        .filter(|update| !is_valid(&ordering, update))
        .map(|update| sort_invalid_update(&ordering, update))
        .map(|update| update[update.len() / 2])
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

    #[test]
    fn test_is_valid() {
        let (ordering, _) = parse_input(TEST_INPUT);
        assert!(is_valid(&ordering, &vec![75, 47, 61, 53, 29]));

        assert!(!is_valid(&ordering, &vec![75, 97, 47, 61, 53]));
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(TEST_INPUT), 143);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT), 123);
    }
}
