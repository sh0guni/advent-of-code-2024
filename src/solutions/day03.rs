use anyhow::Result;
use std::fs;

use regex::{Match, Regex};

pub fn solve() -> Result<()> {
    let input = fs::read_to_string("inputs/day03.txt")?;

    // Part 1
    let result1 = solve_part1(&input);
    println!("Part 1: {}", result1);

    // Part 2
    let result2 = solve_part2(&input);
    println!("Part 2: {}", result2);

    Ok(())
}

struct State {
    result: u32,
    on: bool,
}

fn cap_to_u32(cap: Option<Match>) -> u32 {
    cap.unwrap().as_str().parse::<u32>().unwrap()
}

fn solve_part1(input: &str) -> u32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re.captures_iter(input)
        .map(|cap| cap_to_u32(cap.get(1)) * cap_to_u32(cap.get(2)))
        .sum()
}

fn solve_part2(input: &str) -> u32 {
    let mut regexes = [r"mul\((\d{1,3}),(\d{1,3})\)", r"do\(\)", r"don't\(\)"]
        .into_iter()
        .map(Regex::new)
        .map(Result::unwrap)
        .enumerate()
        .map(|(i, re)| {
            re.captures_iter(input)
                .map(|cap| (i, cap))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>();

    regexes.sort_by_key(|(_, cap)| cap.get(0).unwrap().start());

    regexes
        .into_iter()
        .fold(
            State {
                result: 0,
                on: true,
            },
            |state, (i, cap)| match i {
                0 => State {
                    result: if state.on {
                        state.result + cap_to_u32(cap.get(1)) * cap_to_u32(cap.get(2))
                    } else {
                        state.result
                    },
                    on: state.on,
                },
                1 => State {
                    result: state.result,
                    on: true,
                },
                2 => State {
                    result: state.result,
                    on: false,
                },
                _ => unreachable!(),
            },
        )
        .result
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    const TEST_INPUT_2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(TEST_INPUT), 161);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT_2), 48);
    }
}
