use std::collections::VecDeque;

use itertools::{Either, Itertools};

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let (files, space) = extract_files_and_space(input);

    // println!("files: {:?}", files);
    // println!("space: {:?}", space);

    let compacted = compact(&files, &space, input.len());

    // println!("compacted: {:?}", compacted);

    Some(checksum(compacted))
}

fn extract_files_and_space(input: &str) -> (Vec<(usize, u32)>, Vec<u32>) {
    let (files, space): (Vec<_>, Vec<_>) = input
        .trim()
        .chars()
        .map(|ch| ch.to_digit(10).unwrap())
        .enumerate()
        .partition_map(|(i, d)| {
            if i % 2 == 0 {
                Either::Left((i / 2, d))
            } else {
                Either::Right(d)
            }
        });
    (files, space)
}

fn compact(files: &Vec<(usize, u32)>, space: &Vec<u32>, n: usize) -> Vec<u64> {
    let mut stack = file_stack(files);

    let mut result: Vec<u64> = Vec::new();

    for step in 0..n {
        if stack.is_empty() {
            break;
        }

        if step / 2 >= space.len() {
            let mut stack_rev: Vec<u64> = stack.iter().copied().collect();
            result.append(&mut stack_rev);
            break;
        }

        if step % 2 == 0 {
            let cur_file = stack[0];
            while !stack.is_empty() && stack[0] == cur_file {
                result.push(stack.pop_front().unwrap());
            }
        } else {
            let cur_space = space[step / 2];
            for _ in 0..cur_space {
                result.push(stack.pop_back().unwrap());
            }
        }
    }

    result
}

fn file_stack(files: &Vec<(usize, u32)>) -> VecDeque<u64> {
    files.iter()
        .map(|(id, len)| vec![*id as u64; *len as usize])
        .flatten()
        .collect()
}

fn checksum(compacted: Vec<u64>) -> u64 {
    compacted.iter()
        .enumerate()
        .map(|(i, e)| i as u64 * e)
        .sum()
}

pub fn part_two(input: &str) -> Option<u32> {
    let (files, space) = extract_files_and_space_2(input);

    let len: u64 = input
        .trim()
        .chars()
        .map(|ch| ch.to_digit(10).unwrap() as u64)
        .sum();

    let compacted = compact2(&files, &space, len as usize);

    Some(0)
}

fn extract_files_and_space_2(input: &str) -> (Vec<(u64, u64, u64)>, Vec<(u64, u64)>) {
    let (files, space): (Vec<_>, Vec<_>) = input
        .trim()
        .chars()
        .map(|ch| ch.to_digit(10).unwrap() as u64)
        .enumerate()
        .map(|(i, l)| vec![i; l as usize])
        .flatten()
        .enumerate()
        .partition_map(|(i, d)| {
            if i % 2 == 0 {
                Either::Left((i as u64, (i / 2) as u64, d))
            } else {
                Either::Right((i as u64, d))
            }
        });

    (files, space)
}

fn compact2(files: &Vec<(u64, u64, u64)>, spaces: &Vec<(u64, u64)>, n: usize) -> Vec<u64> {
    let mut result: Vec<u64> = vec![0; n];
    let mut s_copy = spaces.iter().copied().collect();

    for (file_i, file_id, file_len) in files.iter().rev() {
        // find suitable space
        let space = spaces.iter().find(|(_, size)| size >= file_len);

        match space {
            Some((space_i, _)) => {
                // if found add file to the result from the space i
                result[*space_i as usize] = (*file_id, *file_len);
            }
            None => {
                // if not found add file to the result at file id
                result[*file_i as usize] = (*file_id, *file_len);
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
