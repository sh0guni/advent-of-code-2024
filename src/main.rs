use aoc_2024::solutions;
use std::env;

fn main() -> anyhow::Result<()> {
    let day: u8 = env::args()
        .nth(1)
        .expect("Please provide the day number")
        .parse()?;

    match day {
        1 => solutions::day01::solve()?,
        2 => solutions::day02::solve()?,
        3 => solutions::day03::solve()?,
        4 => solutions::day04::solve()?,
        5 => solutions::day05::solve()?,
        6 => solutions::day06::solve()?,
        7 => solutions::day07::solve()?,
        _ => println!("Day {} not implemented yet", day),
    }

    Ok(())
}
