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

pub fn part_two(input: &str) -> Option<u64> {
    let (files, _) = extract_files_and_space_2(input);

    let mut filesystem: Vec<Option<u64>> = input
        .trim()
        .chars()
        .map(|ch| ch.to_digit(10).unwrap() as u64)
        .enumerate()
        .map(|(i, len)| if i % 2 == 0 { vec![Some((i / 2) as u64); len as usize] } else { vec![None; len as usize] })
        .flatten()
        .collect();

    let compacted = compact2(&mut filesystem, files);

    Some(checksum(compacted))
}

fn extract_files_and_space_2(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let (files, space): (Vec<_>, Vec<_>) = input
        .trim()
        .chars()
        .map(|ch| ch.to_digit(10).unwrap() as u64)
        .enumerate()
        .partition_map(|(i, d)| {
            if i % 2 == 0 {
                Either::Left(((i / 2) as u64, d))
            } else {
                Either::Right(d)
            }
        });

    (files, space)
}

fn compact2(filesystem: &mut Vec<Option<u64>>, files: Vec<(u64, u64)>) -> Vec<u64> {
    // let mut result: Vec<Option<u64>> = vec![None; filesystem.len()];

    // println!("filesystem: {:?}", filesystem);
    // println!("files: {:?}", files);

    for (file_id, file_size) in files.iter().rev() {
        // find a file to copy from the end of the filesystem which is not None and has the same id
        let mut file_to_copy_i = 0;
        for i in (0..filesystem.len()).rev() {
            if filesystem[i] == Some(*file_id) {
                file_to_copy_i = (i as i64 - *file_size as i64 + 1) as usize;
                break;
            }
        }

        // find a space to copy the file to from the begginning of the filesystem
        let mut space_to_copy_i = None;
        for i in 0..filesystem.len() {
            // if a window starting from i is big enough to fit the file
            if i < file_to_copy_i && filesystem[i..].iter().take(*file_size as usize).all(|x| *x == None) {
                space_to_copy_i = Some(i);
                break;
            }
        }

        // if space_to_copy is not none, copy the file to the result and delete the file from the initial position, else copy the file to the result to the initial position
        match space_to_copy_i {
            Some(space_i) => {
                for i in 0..*file_size {
                    filesystem[space_i + i as usize] = Some(*file_id);
                    filesystem[file_to_copy_i + i as usize] = None;
                }
            }
            None => {
                for i in 0..*file_size {
                    filesystem[file_to_copy_i + i as usize] = Some(*file_id);
                }
            }
        }
    }

    // println!("result: {:?}", filesystem);

    let r = filesystem.into_iter().map(|m| m.unwrap_or(0)).collect();

    r
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
