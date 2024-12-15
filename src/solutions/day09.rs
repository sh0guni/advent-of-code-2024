use anyhow::Result;
use itertools::Itertools;
use std::{
    fs,
    iter::{once, repeat_n},
};

pub fn solve() -> Result<()> {
    let input = fs::read_to_string("inputs/day09.txt")?;

    // Part 1
    let result1 = solve_part1(&input);
    println!("Part 1: {}", result1);

    // Part 2
    let result2 = solve_part2(&input);
    println!("Part 2: {}", result2);

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum FileBlock {
    Id(usize),
    Free,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FileBlob {
    id: usize,
    size: usize,
    free_space: usize,
}

fn parse_input(input: &str) -> Vec<FileBlock> {
    input
        .chars()
        .chain(once('0'))
        .chunks(2)
        .into_iter()
        .enumerate()
        .flat_map(|(id, chunk)| {
            let (size, space_after) = chunk.collect_tuple().unwrap();
            let size = size.to_digit(10).unwrap() as usize;
            let space_after = space_after.to_digit(10).unwrap() as usize;
            repeat_n(FileBlock::Id(id), size).chain(repeat_n(FileBlock::Free, space_after))
        })
        .collect()
}

fn parse_input_to_blobs(input: &str) -> Vec<FileBlob> {
    input
        .chars()
        .chain(once('0'))
        .chunks(2)
        .into_iter()
        .enumerate()
        .map(|(id, chunk)| {
            let (size, space_after) = chunk.collect_tuple().unwrap();
            let size = size.to_digit(10).unwrap() as usize;
            let space_after = space_after.to_digit(10).unwrap() as usize;
            FileBlob {
                id,
                size,
                free_space: space_after,
            }
        })
        .collect()
}

fn sort_file_map_by_block(file_map: &mut Vec<FileBlock>) {
    let mut i = 0;
    let mut last_i = file_map.len();
    while i < last_i {
        if file_map[i] == FileBlock::Free {
            let last = file_map[last_i - 1].clone();
            if last == FileBlock::Free {
                last_i -= 1;
                continue;
            }
            file_map[i] = last;
            file_map[last_i - 1] = FileBlock::Free;
            last_i -= 1;
        }
        i += 1;
    }
}

fn blob_file_map_to_block_file_map(file_map: &Vec<FileBlob>) -> Vec<FileBlock> {
    file_map
        .iter()
        .flat_map(|blob| {
            repeat_n(FileBlock::Id(blob.id), blob.size)
                .chain(repeat_n(FileBlock::Free, blob.free_space))
        })
        .collect()
}

fn sort_file_map_by_blob(file_map: &mut Vec<FileBlob>) {
    let mut i = file_map.len() - 1;
    while i > 0 {
        let file = file_map[i].clone();
        if let Some((loc, target_space)) = file_map
            .clone()
            .into_iter()
            .take(i)
            .find_position(|blob| blob.free_space >= file.size)
        {
            if loc != i - 1 {
                let file = file_map.remove(i);

                // Add blobs free space to previous blob
                file_map[i - 1].free_space += file.size + file.free_space;

                let free_space = target_space.free_space - file.size;

                file_map[loc].free_space = 0;

                file_map.insert(
                    loc + 1,
                    FileBlob {
                        id: file.id,
                        size: file.size,
                        free_space,
                    },
                );
            } else {
                file_map[i].free_space += file_map[loc].free_space;
                file_map[loc].free_space = 0;
                i -= 1;
            }
        } else {
            i -= 1;
        }
    }
}

fn calculate_checksum(file_map: &Vec<FileBlock>) -> usize {
    file_map
        .iter()
        .enumerate()
        .filter_map(|(i, c)| match c {
            FileBlock::Id(id) => Some(i * id),
            FileBlock::Free => None,
        })
        .sum()
}

fn solve_part1(input: &str) -> usize {
    let mut file_map = parse_input(input);
    sort_file_map_by_block(&mut file_map);
    calculate_checksum(&file_map)
}

fn solve_part2(input: &str) -> usize {
    let mut file_map = parse_input_to_blobs(input);
    sort_file_map_by_blob(&mut file_map);
    calculate_checksum(&blob_file_map_to_block_file_map(&file_map))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn blob_file_map_to_string(file_map: &Vec<FileBlob>) -> String {
        file_map
            .iter()
            .flat_map(|blob| {
                repeat_n(blob.id.to_string(), blob.size)
                    .chain(repeat_n(".".to_string(), blob.free_space))
            })
            .collect::<String>()
    }

    const TEST_INPUT: &str = "2333133121414131402";

    const TEST_INPUT_2: &str = "233313312141413140211";

    const TEST_INPUT_3: &str = "23331331214141314021103";

    const TEST_INPUT_4: &str = "386266167067808";

    #[test]
    fn test_parse_input() {
        let file_map: Vec<FileBlock> = parse_input("12345");
        assert_eq!(
            file_map,
            vec![
                FileBlock::Id(0),
                FileBlock::Free,
                FileBlock::Free,
                FileBlock::Id(1),
                FileBlock::Id(1),
                FileBlock::Id(1),
                FileBlock::Free,
                FileBlock::Free,
                FileBlock::Free,
                FileBlock::Free,
                FileBlock::Id(2),
                FileBlock::Id(2),
                FileBlock::Id(2),
                FileBlock::Id(2),
                FileBlock::Id(2)
            ]
        );
    }

    #[test]
    fn test_simple_part1() {
        assert_eq!(solve_part1("12345"), 60);
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(TEST_INPUT), 1928);
    }

    #[test]
    fn test_part2_sort_2() {
        let mut file_map = parse_input_to_blobs(TEST_INPUT_2);
        println!("{}", blob_file_map_to_string(&file_map));
        sort_file_map_by_blob(&mut file_map);
        println!("{}", blob_file_map_to_string(&file_map));
        assert_eq!(
            blob_file_map_to_string(&file_map),
            "001099111777244.333....5555.6666.....8888...."
        );
    }

    #[test]
    fn test_part2_sort_3() {
        let mut file_map = parse_input_to_blobs(TEST_INPUT_3);
        sort_file_map_by_blob(&mut file_map);
        assert_eq!(
            blob_file_map_to_string(&file_map),
            "0011111111110992777333.44.5555.6666.....8888......."
        );
    }

    #[test]
    fn test_part2_sort_4() {
        let mut file_map = parse_input_to_blobs(TEST_INPUT_4);
        sort_file_map_by_blob(&mut file_map);
        assert_eq!(
            blob_file_map_to_string(&file_map),
            "000777777771111113.222222555555.......4444444.............66666666........"
        );
    }

    const TEST_INPUT_5: &str = "2333133121414134404";

    #[test]
    fn test_part2_sort_5() {
        let mut file_map = parse_input_to_blobs(TEST_INPUT_5);
        sort_file_map_by_blob(&mut file_map);
        assert_eq!(
            blob_file_map_to_string(&file_map),
            "00777111442.333.......5555.6666....99998888...."
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT), 2858);
    }
}
