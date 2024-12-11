use itertools::Itertools;
use memoize::memoize;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    let stones = input.trim().split_whitespace().map(|n| n.parse::<u64>().unwrap()).collect_vec();

    let result = stones.iter().map(|&stone| count_splits(stone, 25)).sum::<u64>();

    Some(result)
}

#[memoize]
fn count_splits(stone: u64, blinks: u64) -> u64 {
    if blinks == 0 {
        return 1;
    }

    if stone == 0 {
        return count_splits(1, blinks - 1);
    }

    if stone.to_string().len() % 2 == 0 {
        let (left, right) = split(stone);
        return count_splits(left, blinks - 1) + count_splits(right, blinks - 1);
    } else {
        return count_splits(stone * 2024, blinks - 1);
    }
}

fn split(stone: u64) -> (u64, u64) {
    let str = stone.to_string();
    let left = str[0..(str.len() / 2)].parse::<u64>().unwrap();
    let right = str[(str.len() / 2)..].parse::<u64>().unwrap();

    (left, right)
}

pub fn part_two(input: &str) -> Option<u64> {
    let stones = input.trim().split_whitespace().map(|n| n.parse::<u64>().unwrap()).collect_vec();

    let result = stones.iter().map(|&stone| count_splits(stone, 75)).sum::<u64>();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
