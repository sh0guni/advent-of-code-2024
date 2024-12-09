use anyhow::Result;
use std::collections::HashSet;
use std::fs;

use crate::types::{Point, Vector};

pub fn solve() -> Result<()> {
    let input = fs::read_to_string("inputs/day06.txt")?;

    // Part 1
    let result1 = solve_part1(&input);
    println!("Part 1: {}", result1);

    // Part 2
    let result2 = solve_part2(&input);
    println!("Part 2: {}", result2);

    Ok(())
}

fn parse_input(input: &str) -> Map {
    input
        .lines()
        .map(|line| line.chars().map(parse_tile).collect())
        .collect()
}

type Map = Vec<Vec<Tile>>;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tile {
    Open,
    Obstacle,
    Guard(Vector),
}

fn parse_tile(c: char) -> Tile {
    match c {
        '.' => Tile::Open,
        '#' => Tile::Obstacle,
        '^' => Tile::Guard(Vector { dx: 0, dy: -1 }),
        'v' => Tile::Guard(Vector { dx: 0, dy: 1 }),
        '<' => Tile::Guard(Vector { dx: -1, dy: 0 }),
        '>' => Tile::Guard(Vector { dx: 1, dy: 0 }),
        _ => unreachable!(),
    }
}

fn get_tile(map: &Map, pos: &Point) -> Tile {
    map[pos.y as usize][pos.x as usize].clone()
}

fn turn_right(dir: &Vector) -> Vector {
    match dir {
        Vector { dx: 0, dy: -1 } => Vector { dx: 1, dy: 0 },
        Vector { dx: 1, dy: 0 } => Vector { dx: 0, dy: 1 },
        Vector { dx: 0, dy: 1 } => Vector { dx: -1, dy: 0 },
        Vector { dx: -1, dy: 0 } => Vector { dx: 0, dy: -1 },
        _ => unreachable!(),
    }
}

fn is_in_bounds(map: &Map, pos: &Point) -> bool {
    pos.x >= 0 && pos.y >= 0 && pos.x < map[0].len() as isize && pos.y < map.len() as isize
}

type GuardPosition = (Point, Vector);

fn step(
    map: &Map,
    pos: &Point,
    dir: &Vector,
    visited: &mut HashSet<Point>,
    positions: &mut HashSet<GuardPosition>,
) -> Option<usize> {
    if !positions.insert((pos.clone(), dir.clone())) {
        return None;
    }

    let next = pos + dir;
    if !is_in_bounds(map, &next) {
        return Some(0);
    }

    match get_tile(map, &next) {
        Tile::Obstacle => step(map, pos, &turn_right(dir), visited, positions),
        _ => {
            let ds = if visited.insert(next.clone()) { 1 } else { 0 };
            step(map, &next, dir, visited, positions).map(|ds2| ds + ds2)
        }
    }
}

fn go(map: &Map, pos: &Point, dir: &Vector) -> Option<(usize, HashSet<Point>)> {
    let mut visited = HashSet::from([pos.clone()]);
    let mut positions = HashSet::new();
    step(map, pos, dir, &mut visited, &mut positions).map(|ds| (ds + 1, visited))
}

fn find_start(map: &Map) -> (Point, Vector) {
    let initial_guard_dir = Vector { dx: 0, dy: -1 };
    let initial_guard_tile = Tile::Guard(initial_guard_dir.clone());
    let initial_pos = map
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .position(|c| c == &initial_guard_tile)
                .map(|x| Point::new((x, y)))
        })
        .unwrap();
    (initial_pos, initial_guard_dir)
}

fn start_simulation(map: &Map) -> usize {
    let (start, dir) = find_start(map);
    go(&map, &start, &dir).unwrap().0
}

fn add_obstacle(map: &Map, pos: &Point) -> Map {
    let mut new_map = map.clone();
    new_map[pos.y as usize][pos.x as usize] = Tile::Obstacle;
    new_map
}

fn solve_part1(input: &str) -> usize {
    let map = parse_input(input);
    start_simulation(&map)
}

fn solve_part2(input: &str) -> usize {
    let map = parse_input(input);
    let (start, dir) = find_start(&map);
    let (_, visited) = go(&map, &start, &dir).unwrap();
    visited
        .iter()
        .filter(|p| {
            let map = add_obstacle(&map, p);
            go(&map, &start, &dir).is_none()
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

    #[test]
    fn test_go() {
        let map: Map = vec![vec![Tile::Guard(Vector { dx: 0, dy: -1 })]];
        assert_eq!(
            go(&map, &Point::new((0, 0)), &Vector { dx: 0, dy: -1 })
                .unwrap()
                .0,
            1
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(TEST_INPUT), 41);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT), 6);
    }
}
