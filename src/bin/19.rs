use std::{cmp::min, collections::HashSet, hash::Hash, mem, result};

use itertools::Itertools;
use memoize::memoize;

advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<u64> {

    let towels = input.lines().collect_vec()[0].split(", ").map(|s| s.to_string()).collect_vec();

    let towels_set: HashSet<String> = HashSet::from_iter(towels.iter().cloned());

    let designs = input.lines()
        .skip(2)
        .map(|s| s.to_string())
        .collect_vec();

    let result = designs.iter()
        .cloned()
        .filter(|design| can_build(design.clone(), &towels_set) > 0)
        .count();

    Some(result as u64)
}

#[memoize(Ignore: towels)]
fn can_build(design: String, towels: &HashSet<String>) -> u64 {
    if design.is_empty() {
        return 1;
    }

    let mut result = 0;

    for prefix in 1..min(design.len() + 1, 9) {
        // println!("Checking {} {} {}", design, &design[..prefix], &design[prefix..].to_string());
        // println!("Contains: {:?}", towels.contains(&design[..prefix]));

        if towels.contains(&design[..prefix]) {
            result += can_build(design[prefix..].to_string(), towels);
        }
    }

    result
}

pub fn part_two(input: &str) -> Option<u64> {
    let towels = input.lines().collect_vec()[0].split(", ").map(|s| s.to_string()).collect_vec();

    let towels_set: HashSet<String> = HashSet::from_iter(towels.iter().cloned());

    let designs = input.lines()
        .skip(2)
        .map(|s| s.to_string())
        .collect_vec();

    let result: u64 = designs.iter()
        .cloned()
        .map(|design| can_build(design.clone(), &towels_set))
        .sum();

    Some(result as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
