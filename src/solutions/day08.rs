use anyhow::Result;
use itertools::Itertools;
use std::fs;

use crate::types::{Point, Vector};

pub fn solve() -> Result<()> {
    let input = fs::read_to_string("inputs/day08.txt")?;

    // Part 1
    let result1 = solve_part1(&input);
    println!("Part 1: {}", result1);

    // Part 2
    let result2 = solve_part2(&input);
    println!("Part 2: {}", result2);

    Ok(())
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn find_antennas(grid: &[Vec<char>]) -> Vec<(char, Point)> {
    grid.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().map(move |(x, &c)| {
                (
                    c,
                    Point {
                        x: x as isize,
                        y: y as isize,
                    },
                )
            })
        })
        .filter(|&(c, _)| c != '.')
        .collect()
}

fn find_antinode(p1: &Point, p2: &Point) -> Point {
    let diff = Vector {
        dx: (p1.x - p2.x),
        dy: (p1.y - p2.y),
    };
    p1 + diff
}

fn find_all_antinodes(grid: &[Vec<char>], p1: &Point, p2: &Point) -> Vec<Point> {
    let diff = Vector {
        dx: (p1.x - p2.x),
        dy: (p1.y - p2.y),
    };
    let pos = (1..)
        .map(|i| p1 + &diff * i)
        .take_while(|p| within_bounds(p, grid));
    let neg = (1..)
        .map(|i| p1 - &diff * i)
        .take_while(|p| within_bounds(p, grid));
    pos.chain(neg).collect()
}

fn within_bounds(p: &Point, grid: &[Vec<char>]) -> bool {
    p.x >= 0 && p.x < grid[0].len() as isize && p.y >= 0 && p.y < grid.len() as isize
}

fn solve_part1(input: &str) -> usize {
    let grid = parse_input(input);
    let antennas = find_antennas(&grid);
    antennas
        .iter()
        .flat_map(|(c, p1)| {
            antennas
                .iter()
                .filter(|(c2, p2)| c2 == c && p2 != p1)
                .map(|(_, p2)| find_antinode(p1, p2))
                .collect::<Vec<_>>()
        })
        .filter(|p| within_bounds(p, &grid))
        .unique()
        .count()
}

fn solve_part2(input: &str) -> usize {
    let grid = parse_input(input);
    let antennas = find_antennas(&grid);
    antennas
        .iter()
        .flat_map(|(c, p1)| {
            antennas
                .iter()
                .filter(|(c2, p2)| c2 == c && p2 != p1)
                .flat_map(|(_, p2)| find_all_antinodes(&grid, p1, p2))
                .collect::<Vec<_>>()
        })
        .unique()
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

    const TEST_INPUT_2: &str = "\
..........
..........
..........
....a.....
..........
.....a....
..........
..........
..........
..........
";

    const TEST_INPUT_3: &str = "\
T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
..........
";

    #[test]
    fn test_find_antinode() {
        assert_eq!(
            find_antinode(&Point { x: 4, y: 3 }, &Point { x: 5, y: 5 }),
            Point { x: 3, y: 1 }
        );
    }

    #[test]
    fn test_simple_input() {
        assert_eq!(solve_part1(TEST_INPUT_2), 2);
    }

    #[test]
    fn test_simple_input_2() {
        assert_eq!(solve_part2(TEST_INPUT_3), 9);
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(TEST_INPUT), 14);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT), 34);
    }
}
