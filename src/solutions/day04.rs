use anyhow::Result;
use itertools::Itertools;
use std::{
    fs,
    ops::{Add, Mul},
};

pub fn solve() -> Result<()> {
    let input = fs::read_to_string("inputs/day04.txt")?;

    // Part 1
    let result1 = solve_part1(&input);
    println!("Part 1: {}", result1);

    // Part 2
    let result2 = solve_part2(&input);
    println!("Part 2: {}", result2);

    Ok(())
}

struct Point {
    x: isize,
    y: isize,
}

struct Vector {
    dx: isize,
    dy: isize,
}

impl Point {
    fn new((x, y): (usize, usize)) -> Self {
        Self {
            x: x as isize,
            y: y as isize,
        }
    }
}

impl Add<Vector> for &Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        Point {
            x: self.x + rhs.dx,
            y: self.y + rhs.dy,
        }
    }
}

impl Mul<usize> for &Vector {
    type Output = Vector;

    fn mul(self, rhs: usize) -> Self::Output {
        Vector {
            dx: self.dx * rhs as isize,
            dy: self.dy * rhs as isize,
        }
    }
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

const XMAS: &str = "XMAS";

fn is_in_bounds(matrix: &[Vec<char>], point: &Point) -> bool {
    point.x >= 0
        && point.x < matrix[0].len() as isize
        && point.y >= 0
        && point.y < matrix.len() as isize
}

fn get_char(matrix: &[Vec<char>], point: &Point) -> char {
    matrix[point.y as usize][point.x as usize]
}

fn test_char(matrix: &[Vec<char>], point: &Point, expected: char) -> bool {
    is_in_bounds(matrix, point) && matrix[point.y as usize][point.x as usize] == expected
}

fn find_xmas_in_direction(matrix: &[Vec<char>], point: &Point, direction: &Vector) -> bool {
    (0..XMAS.len()).all(|i| {
        let current_point = point + direction * i;
        test_char(matrix, &current_point, XMAS.chars().nth(i).unwrap())
    })
}

const DIRECTIONS: [Vector; 8] = [
    Vector { dx: 0, dy: -1 },  // North
    Vector { dx: 1, dy: -1 },  // Northeast
    Vector { dx: 1, dy: 0 },   // East
    Vector { dx: 1, dy: 1 },   // Southeast
    Vector { dx: 0, dy: 1 },   // South
    Vector { dx: -1, dy: 1 },  // Southwest
    Vector { dx: -1, dy: 0 },  // West
    Vector { dx: -1, dy: -1 }, // Northwest
];

fn find_xmas_in_all_directions(matrix: &[Vec<char>], point: &Point) -> usize {
    DIRECTIONS
        .iter()
        .filter(|direction| find_xmas_in_direction(matrix, point, direction))
        .count()
}

fn test_mas(matrix: &[Vec<char>], points: &[Point; 2]) -> bool {
    let s = points
        .iter()
        .map(|point| get_char(matrix, point))
        .collect::<String>();
    s == "MS" || s == "SM"
}

fn is_x_mas(matrix: &[Vec<char>], point: &Point) -> bool {
    let top_left = point + Vector { dx: -1, dy: -1 };
    let top_right = point + Vector { dx: 1, dy: -1 };
    let bottom_left = point + Vector { dx: -1, dy: 1 };
    let bottom_right = point + Vector { dx: 1, dy: 1 };

    if ![&top_left, &top_right, &bottom_left, &bottom_right]
        .iter()
        .all(|point| is_in_bounds(matrix, point))
    {
        return false;
    }

    test_mas(matrix, &[top_left, bottom_right]) && test_mas(matrix, &[top_right, bottom_left])
}

fn solve_part1(input: &str) -> usize {
    let matrix = parse_input(input);
    let find = |point: Point| find_xmas_in_all_directions(&matrix, &point);

    (0..matrix.len())
        .cartesian_product(0..matrix[0].len())
        .map(Point::new)
        .map(find)
        .sum()
}

fn solve_part2(input: &str) -> usize {
    let matrix = parse_input(input);

    (0..matrix.len())
        .cartesian_product(0..matrix[0].len())
        .map(Point::new)
        .filter(|point| {
            let value = matrix[point.y as usize][point.x as usize];
            value == 'A'
        })
        .filter(|point| is_x_mas(&matrix, point))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

    #[test]
    fn test_find_xmas_in_direction() {
        let matrix = parse_input(TEST_INPUT);

        assert!(!find_xmas_in_direction(
            &matrix,
            &Point { x: 0, y: 0 },
            &Vector { dx: 1, dy: 0 }
        ));

        assert!(find_xmas_in_direction(
            &matrix,
            &Point { x: 5, y: 0 },
            &Vector { dx: 1, dy: 0 }
        ));

        assert!(!find_xmas_in_direction(
            &matrix,
            &Point { x: 5, y: 0 },
            &Vector { dx: 1, dy: -1 }
        ));

        assert!(find_xmas_in_direction(
            &matrix,
            &Point { x: 4, y: 0 },
            &Vector { dx: 1, dy: 1 }
        ));
    }

    #[test]
    fn test_find_xmas_in_all_directions() {
        let matrix = parse_input(TEST_INPUT);

        assert_eq!(
            find_xmas_in_all_directions(&matrix, &Point { x: 6, y: 4 }),
            2
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(TEST_INPUT), 18);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT), 9);
    }
}
