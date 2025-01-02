use std::vec;

use itertools::Itertools;

advent_of_code::solution!(25);

pub fn part_one(input: &str) -> Option<u32> {
    let mut locks = Vec::new();
    let mut keys = Vec::new();

    input.lines().chunks(8).into_iter()
        .for_each(|chunk| {
            let grid = chunk.take(7).map(|line| line.chars().collect_vec()).collect_vec();
            let mut counts = vec![0; 5];

            for j in 0..5 {
                for i in 0..7 {
                    if grid[i][j] == '#' {
                        counts[j] += 1;
                    }
                }
            }

            if grid[0][0] == '#' {
                locks.push(counts.clone());
            } else {
                keys.push(counts.clone());
            }
        });

    // println!("{:?}", locks);
    // println!("{:?}", keys);

    let result = locks.iter().cartesian_product(keys).filter(|(lock, key)| {
        lock.iter().zip(key.iter()).all(|(l, k)| l + k < 8)
    }).count();

    Some(result as u32)

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
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
