use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let matrix = get_map(input);
    let trailheads = find_traiheads(&matrix);
    let result = find_trailheads_score(&matrix, &trailheads);

    return Some(result);
}

fn get_map(input: &str) -> Vec<Vec<u32>> {
    return input.lines()
        .map(|line| line.chars()
            .map(|ch| ch.to_digit(10).unwrap())
            .collect_vec()
        )
        .collect_vec();
}

fn find_traiheads(matrix: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let mut trailheads = Vec::new();
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == 0 {
                trailheads.push((i, j));
            }
        }
    }

    return trailheads;
}

fn find_trailheads_score(matrix: &Vec<Vec<u32>>, trailheads: &Vec<(usize, usize)>) -> u32 {
    trailheads.iter()
        .map(|(x, y)| find_trailhead_score(&matrix, *x, *y))
        .sum()
}

fn find_trailhead_score(matrix: &Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    let mut result: HashSet<(usize, usize)> = HashSet::new();

    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut level = VecDeque::new();
    level.push_back((x, y));


    while !level.is_empty() {
        let (cur_x, cur_y) = level.pop_front().unwrap();

        if matrix[cur_x][cur_y] == 9 {
            result.insert((cur_x, cur_y));
            continue;
        }

        for (dx, dy) in &directions {
            let new_x = cur_x as i32 + dx;
            let new_y = cur_y as i32 + dy;
            if new_x >= 0 && new_x < matrix.len() as i32 && new_y >= 0 && new_y < matrix[0].len() as i32
                          && matrix[new_x as usize][new_y as usize] as i32 - matrix[cur_x][cur_y] as i32 == 1 {
                level.push_back((new_x as usize, new_y as usize));
            }
        }
    }

    return result.len() as u32;
}

pub fn part_two(input: &str) -> Option<u32> {
    let matrix = get_map(input);
    let trailheads = find_traiheads(&matrix);
    let result = find_trailheads_rating(&matrix, &trailheads);

    return Some(result);
}

fn find_trailheads_rating(matrix: &Vec<Vec<u32>>, trailheads: &Vec<(usize, usize)>) -> u32 {
    trailheads.iter()
        .map(|(x, y)| find_rating(&matrix, *x, *y))
        .sum()
}

fn find_rating(matrix: &Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    let mut result = 0;

    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut level = VecDeque::new();
    level.push_back((x, y));

    while !level.is_empty() {
        let (cur_x, cur_y) = level.pop_front().unwrap();

        if matrix[cur_x][cur_y] == 9 {
            result += 1;
            continue;
        }

        for (dx, dy) in &directions {
            let new_x = cur_x as i32 + dx;
            let new_y = cur_y as i32 + dy;
            if new_x >= 0 && new_x < matrix.len() as i32 && new_y >= 0 && new_y < matrix[0].len() as i32
                          && matrix[new_x as usize][new_y as usize] as i32 - matrix[cur_x][cur_y] as i32 == 1 {
                level.push_back((new_x as usize, new_y as usize));
            }
        }
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
