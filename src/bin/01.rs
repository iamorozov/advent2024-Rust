use itertools::Itertools;
use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (list1, list2) = get_lists(input);

    let list1: Vec<u32> = list1.into_iter().sorted().collect();
    let list2: Vec<u32> = list2.into_iter().sorted().collect();

    let sum = list1.into_iter().zip(list2)
        .map(|(n1, n2)| (n1 as i32 - n2 as i32).abs() as u32)
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (list1, list2) = get_lists(input);

    let list2_map: HashMap<u32, u32> = list2.into_iter()
        .fold(HashMap::new(), |mut map, num| {
            *map.entry(num).or_insert(0) += 1;
            map
        });

    let sum: u32 = list1.into_iter()
        .map(|d| d * list2_map.get(&d).unwrap_or(&0))
        .sum();

    Some(sum)
}

fn get_lists(input: &str) -> (Vec<u32>, Vec<u32>) {
    return input.lines()
        .map ( |line| line.split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect_tuple()
            .unwrap()
        )
        .unzip();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
