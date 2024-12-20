use std::collections::HashSet;

use advent_of_code::utils::{find_position_in_matrix, get_char_matrix};

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let grid = get_char_matrix(input);
    let start = find_position_in_matrix(&grid, '^');
    let path = calculate_path(&grid, start);
    Some(path.len() as u32)
}

fn turn(dir: (i32, i32)) -> (i32, i32) {
    let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    dirs[(dirs.iter().position(|d| d == &dir).unwrap() + 1) % dirs.len()]
}

fn out_of_grid(i: i32, j: i32, grid: &Vec<Vec<char>>) -> bool {
    i < 0 || j < 0 || i >= grid.len() as i32 || j >= grid[0].len() as i32
}

fn calculate_path(grid: &Vec<Vec<char>>, start: (usize, usize)) -> HashSet<(usize, usize)> {
    let (mut i, mut j) = start;
    let mut dir = (-1, 0);
    let mut result: HashSet<(usize, usize)> = HashSet::default();

    loop {
        result.insert((i, j));

        let (in_grid, new_i, new_j, dir0, dir1) = step(i, j, dir, grid);
        if in_grid {
            (i, j) = (new_i, new_j);
            dir = (dir0, dir1);
        } else {
            return result
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = get_char_matrix(input);
    let start = find_position_in_matrix(&grid, '^');
    let result = count_cycles(&grid, start);
    Some(result)
}

fn step(i: usize, j: usize, dir: (i32, i32), grid: &Vec<Vec<char>>) -> (bool, usize, usize, i32, i32) {
    let (di, dj) = (i as i32 + dir.0, j as i32 + dir.1);

    if out_of_grid(di, dj, grid) {
        return (false, 0, 0, 0, 0)
    }

    let mut new_dir = dir;
    let mut new_i = i;
    let mut new_j = j;

    if grid[di as usize][dj as usize] == '#' {
        new_dir = turn(dir);
    } else {
        new_i = di as usize;
        new_j = dj as usize;
    }

    return (true, new_i, new_j, new_dir.0, new_dir.1);
}

fn count_cycles(grid: &Vec<Vec<char>>, start: (usize, usize)) -> u32 {
    fn is_loop(clone: &Vec<Vec<char>>, start: (usize, usize)) -> bool {
        let (mut i, mut j) = start;
        let mut dir = (-1, 0);
        let mut visited: HashSet<(usize, usize, i32, i32)> = HashSet::default();

        loop {
            visited.insert((i, j, dir.0, dir.1));

            let (in_grid, new_i, new_j, dir0, dir1) = step(i, j, dir, clone);
            if in_grid {
                (i, j) = (new_i, new_j);
                dir = (dir0, dir1);

                if visited.contains(&(i, j, dir.0, dir.1)) {
                    return true;
                }
            } else {
                return false;
            }
        }
    }

    let mut clone = grid.clone();
    calculate_path(&grid, start)
        .into_iter()
        .filter(|(i, j)| {
            clone[*i][*j] = '#';
            let l = is_loop(&clone, start);
            clone[*i][*j] = '.';
            l
        })
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
