
use std::{cmp::Ordering, collections::{BinaryHeap, HashMap, HashSet}};

use itertools::Itertools;

advent_of_code::solution!(18);

pub fn part_one(input: &str) -> Option<usize> {
    let bytes = read_bytes(input);
    let mut grid: Vec<Vec<char>> = vec![vec!['.'; 71]; 71];

    for (x, y) in &bytes[..1024] {
        grid[*x][*y] = '#';
    }

    let path = shortest_path(&mut grid, (0, 0), (70, 70)).unwrap();

    Some(path)
}

fn read_bytes(input: &str) -> Vec<(usize, usize)> {
    input.lines()
        .map(|line| sscanf::scanf!(line, "{},{}", usize, usize).unwrap())
        .collect_vec()
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    score: usize,
    direction: Direction,
    position: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.score.cmp(&self.score)
            .then_with(|| self.position.1.cmp(&other.position.1).then_with(|| self.position.0.cmp(&other.position.0)))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn shortest_path(maze: &Vec<Vec<char>>, start: (usize, usize), goal: (usize, usize)) -> Option<usize> {
    let mut dist: HashMap<(usize, usize), usize> = HashMap::new();
    let mut heap = BinaryHeap::new();

    dist.insert(start, 0);
    heap.push(State { score: 0, direction: Direction::Right, position: start });

    while let Some(State { score, direction, position }) = heap.pop() {

        if position == goal { return Some(score); }

        if score > *dist.get(&position).unwrap_or(&usize::MAX) { continue; }

        for next in get_next_tiles(maze, position.0, position.1, direction, score) {
            if next.score < *dist.get(&next.position).unwrap_or(&usize::MAX) {
                heap.push(next);
                dist.insert(next.position, next.score);
            }
        }
    }

    None
}

fn get_next_tiles(maze: &Vec<Vec<char>>, x: usize, y: usize, direction: Direction, score: usize) -> Vec<State> {
    let mut next_tiles = Vec::new();
    let directions = vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right];

    for d in directions {
        let (dx, dy): (i32, i32) = match d {
            Direction::Up => (x as i32 - 1, y as i32),
            Direction::Down => (x as i32 + 1, y as i32),
            Direction::Left => (x as i32, y as i32 - 1),
            Direction::Right => (x as i32, y as i32 + 1),
        };

        if dx >= 0 && dx < maze.len() as i32 && dy >= 0 && dy < maze[0].len() as i32 && maze[dx as usize][dy as usize] != '#' {
            next_tiles.push(State { score: score + 1, direction, position: (dx as usize, dy as usize) });
        }
    }

    next_tiles
}

pub fn part_two(input: &str) -> Option<String> {
    let bytes = input.lines()
        .map(|line| sscanf::scanf!(line, "{},{}", usize, usize).unwrap())
        .collect_vec();

    let mut grid = vec![vec!['.'; 71]; 71];

    for (x, y) in bytes {
        grid[x][y] = '#';

        let path = shortest_path(&mut grid, (0, 0), (70, 70));

        if path.is_none() {
            return Some(format!("{}, {}", x, y));
        }
    }

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
        assert_eq!(result, Some(String::from("6, 1")));
    }
}
