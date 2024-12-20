advent_of_code::solution!(20);

use std::collections::HashMap;
use std::collections::{HashSet, VecDeque};

use advent_of_code::utils::{find_position_in_matrix, get_char_matrix};
use itertools::Itertools;

pub fn part_one(input: &str) -> Option<usize> {
    let mut maze = get_char_matrix(input);
    let start = find_position_in_matrix(&maze, 'S');
    let end = find_position_in_matrix(&maze, 'E');

    let mut positions = HashMap::new();

    positions.insert(start, 0);
    let mut cur = start;
    let mut len = 0;

    while cur != end {
        for (x, y) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let (dx, dy) = ((cur.0 as i32 + x) as usize, (cur.1 as i32 + y) as usize);

            if maze[dx][dy] == '#' || positions.contains_key(&(dx, dy)) {
                continue;
            }

            len += 1;
            positions.insert((dx, dy), len);
            cur = (dx, dy);
            break;
        }
    }

    let mut cheats = HashMap::new();
    for (x, y) in positions.keys() {
        let candidates = find_in_radius(&maze, *x, *y, 2);
        // println!("x,y = {},{}, Candidates {:?}", x, y, candidates);

        for (px, py, d) in candidates {
            let save = positions[&(px, py)] as i32 - positions[&(*x, *y)] as i32 - d as i32;
            if save > 0 {
                // println!("Save {}, x,y = {},{} px,py = {},{}", save, *x, *y, px, py);
                let ch = cheats.entry(save).or_insert(0);
                *ch += 1;
            }
        }
    }

    // println!("Cheats {:?}", cheats.iter().sorted_by(|a, b| b.1.cmp(a.1)).collect::<Vec<_>>());

    let result = cheats.into_iter()
        .filter_map(|(save, count)| if save >= 100 { Some(count) } else { None })
        .sum::<usize>();

    Some(result)
}

fn find_in_radius(maze: &Vec<Vec<char>>, x: usize, y: usize, radius: usize) -> Vec<(usize, usize, usize)> {
    let mut positions = Vec::new();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back((x, y, 0));

    while let Some((x, y, d)) = queue.pop_front() {
        if d > radius { break; }

        if visited.contains(&(x, y)) { continue; }

        visited.insert((x, y));

        if maze[x][y] != '#' {
            positions.push((x, y, d));
        }

        if x > 0 { queue.push_back((x - 1, y, d + 1)); }
        if x < maze.len() - 1 { queue.push_back((x + 1, y, d + 1)); }
        if y > 0 { queue.push_back((x, y - 1, d + 1)); }
        if y < maze[0].len() - 1 { queue.push_back((x, y + 1, d + 1)); }
    }

    positions
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut maze = get_char_matrix(input);
    let start = find_position_in_matrix(&maze, 'S');
    let end = find_position_in_matrix(&maze, 'E');

    let mut positions = HashMap::new();

    positions.insert(start, 0);
    let mut cur = start;
    let mut len = 0;

    while cur != end {
        for (x, y) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let (dx, dy) = ((cur.0 as i32 + x) as usize, (cur.1 as i32 + y) as usize);

            if maze[dx][dy] == '#' || positions.contains_key(&(dx, dy)) {
                continue;
            }

            len += 1;
            positions.insert((dx, dy), len);
            cur = (dx, dy);
            break;
        }
    }

    let mut cheats = HashMap::new();
    for (x, y) in positions.keys() {
        let candidates = find_in_radius(&maze, *x, *y, 20);
        // println!("x,y = {},{}, Candidates {:?}", x, y, candidates);

        for (px, py, d) in candidates {
            let save = positions[&(px, py)] as i32 - positions[&(*x, *y)] as i32 - d as i32;
            if save > 0 {
                // println!("Save {}, x,y = {},{} px,py = {},{}", save, *x, *y, px, py);
                let ch = cheats.entry(save).or_insert(0);
                *ch += 1;
            }
        }
    }

    // println!("Cheats {:?}", cheats.iter().sorted_by(|a, b| b.1.cmp(a.1)).collect::<Vec<_>>());

    let result = cheats.into_iter()
        .filter_map(|(save, count)| if save >= 100 { Some(count) } else { None })
        .sum::<usize>();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }
}
