use itertools::Itertools;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u32> {
    let mut stones = input.trim().split_whitespace().map(|n| n.parse::<u64>().unwrap()).collect_vec();

    for blink in 0..6 {
        let copy = stones.iter().copied().collect_vec();
        for (i, stone) in stones.iter_mut().enumerate() {
            if *stone == 0 {
                stones[i] = 1;
            }
        }
    }

    Some(0)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
