use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

use advent_of_code::utils::get_char_matrix;

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u32> {
    let farm = get_char_matrix(input);
    let mut result = 0;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    for i in 0..farm.len() {
        for j in 0..farm[0].len() {
            if visited.contains(&(i, j)) {
                continue;
            }

            let (perimeter, area, region) = find_region(&farm, i, j);

            result += area * perimeter;
            visited.extend(region);
        }
    }

    Some(result)
}

fn find_region(farm: &Vec<Vec<char>>, i: usize, j: usize) -> (u32, u32, HashSet<(usize, usize)>) {
    let mut perimeter = 0;
    let mut area = 0;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    queue.push_back((i, j));
    let dir = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    while !queue.is_empty() {
        let (x, y ) = queue.pop_front().unwrap();
        visited.insert((x, y));
        area += 1;

        for (dx, dy) in dir {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx < 0 || ny < 0 || nx >= farm.len() as i32 || ny >= farm[0].len() as i32 || farm[nx as usize][ny as usize] != farm[x][y] {
                perimeter += 1;
                continue;
            } else {
                if !visited.contains(&(nx as usize, ny as usize)) && !queue.contains(&(nx as usize, ny as usize)) {
                    queue.push_back((nx as usize, ny as usize));
                }
            }
        }
    }

    return (perimeter, area, visited);
}

pub fn part_two(input: &str) -> Option<u32> {
    let farm = get_char_matrix(input);
    let mut result = 0;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    for i in 0..farm.len() {
        for j in 0..farm[0].len() {
            if visited.contains(&(i, j)) {
                continue;
            }

            let (sides, area, region) = find_region_with_sides(&farm, i, j);

            result += area * sides;
            visited.extend(region);
        }
    }

    Some(result)
}

fn find_region_with_sides(farm: &Vec<Vec<char>>, i: usize, j: usize) -> (u32, u32, HashSet<(usize, usize)>) {
    let mut sides_right = HashMap::new();
    let mut sides_left = HashMap::new();
    let mut sides_top = HashMap::new();
    let mut sides_bottom = HashMap::new();

    let mut area = 0;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    queue.push_back((i, j));
    let dir = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    while !queue.is_empty() {
        let (x, y ) = queue.pop_front().unwrap();
        visited.insert((x, y));
        area += 1;

        for (dx, dy) in dir {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx < 0 || ny < 0 || nx >= farm.len() as i32 || ny >= farm[0].len() as i32 || farm[nx as usize][ny as usize] != farm[x][y] {
                if dx == 0 && dy == 1 {
                    sides_right.entry(y).or_insert(vec![]).push(x);
                } else if dx == 0 && dy == -1 {
                    sides_left.entry(y).or_insert(vec![]).push(x);
                } else if dx == 1 && dy == 0 {
                    sides_bottom.entry(x).or_insert(vec![]).push(y);
                } else if dx == -1 && dy == 0 {
                    sides_top.entry(x).or_insert(vec![]).push(y);
                }
            } else {
                if !visited.contains(&(nx as usize, ny as usize)) && !queue.contains(&(nx as usize, ny as usize)) {
                    queue.push_back((nx as usize, ny as usize));
                }
            }
        }
    }

    let sides = unique_sides_count(&sides_right) + unique_sides_count(&sides_left) + unique_sides_count(&sides_top) + unique_sides_count(&sides_bottom);

    return (sides, area, visited);
}

fn unique_sides_count(sides: &HashMap<usize, Vec<usize>>) -> u32 {
    let mut result = 0;
    for (_, v) in sides {
        for i in 0..v.len() {
            let sorted = v.iter().sorted().collect_vec();

            if i == 0 {
                result += 1;
            } else {
                if sorted[i] - sorted[i - 1] > 1 {
                    result += 1;
                }
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
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
