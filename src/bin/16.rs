
use std::{cmp::Ordering, collections::HashMap};
use std::collections::{BinaryHeap, HashSet};

use advent_of_code::utils::{find_position_in_matrix, get_char_matrix};
use itertools::Itertools;

advent_of_code::solution!(16);

pub fn part_one(input: &str) -> Option<usize> {
    let mut maze = get_char_matrix(input);
    let (sx, sy) = find_position_in_matrix(&maze, 'S');
    let (ex, ey) = find_position_in_matrix(&maze, 'E');

    shortest_path(&mut maze, (sx, sy), (ex, ey))
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    score: usize,
    direction: Direction,
    tile: (usize, usize),
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.score.cmp(&self.score)
            .then_with(|| self.tile.1.cmp(&other.tile.1).then_with(|| self.tile.0.cmp(&other.tile.0)))
    }
}

// `PartialOrd` needs to be implemented as well.
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
    heap.push(State { score: 0, direction: Direction::Right, tile: start });

    while let Some(State { score, direction, tile }) = heap.pop() {

        // print!("current tile: {:?}, score: {}, direction: {:?}\n", tile, score, direction);

        // Alternatively we could have continued to find all shortest paths
        if tile == goal { return Some(score); }

        if score > *dist.get(&tile).unwrap_or(&usize::MAX) { continue; }

        for next in get_next_tiles(maze, tile.0, tile.1, direction, score) {
            if next.score < *dist.get(&next.tile).unwrap_or(&usize::MAX) {
                heap.push(next);
                dist.insert(next.tile, next.score);
            }
        }
    }

    None
}

fn get_next_tiles(maze: &Vec<Vec<char>>, x: usize, y: usize, direction: Direction, score: usize) -> Vec<State> {
    let mut next_tiles = Vec::new();
    let mut directions = vec![direction];

    match direction {
        Direction::Up | Direction::Down => { directions.push(Direction::Left); directions.push(Direction::Right) },
        Direction::Left | Direction::Right => { directions.push(Direction::Up); directions.push(Direction::Down) },
    };

    for d in directions {
        let (dx, dy) = match d {
            Direction::Up => (x-1, y),
            Direction::Down => (x+1, y),
            Direction::Left => (x, y-1),
            Direction::Right => (x, y+1),
        };

        if maze[dx][dy] != '#' {
            if d == direction {
                next_tiles.push(State { score: score + 1, direction, tile: (dx, dy) });
            } else {
                next_tiles.push(State { score: score + 1001, direction: d, tile: (dx, dy) });
            }
        }
    }

    next_tiles
}

pub fn part_two(input: &str) -> Option<usize> {
    let maze = get_char_matrix(input);

    Some(shortest_paths_tiles(maze))
}

fn shortest_paths_tiles(mut map: Vec<Vec<char>>) -> usize {
    // find all shortest paths from start to goal. The shortest path is the one with the smallest number of tiles and the smallest number of turns
    // every time we can turn only on 90 degrees. The algorithm should be dfs

    let mut to_visit = vec![(find(&map, 'S'), Direction::Right)];
    let mut value_map: HashMap<(Position, Direction), (i64, Vec<(Position, Direction)>)> =
        Default::default();

    value_map.insert(to_visit[0], (0, vec![]));

    let mut visited: HashSet<(Position, Direction)> = Default::default();

    while let Some((visiting, current_direction)) = to_visit.pop() {
        if visited.contains(&(visiting, current_direction)) {
            continue;
        }
        visited.insert((visiting, current_direction));

        let visiting_value = value_map.get(&(visiting, current_direction)).unwrap().0;

        if let Some(follow_point) = try_move(&map, visiting, current_direction) {
            if map[follow_point.row][follow_point.column] != '#' {
                update_value_map_if_better(
                    &mut value_map,
                    (visiting, current_direction),
                    (follow_point, current_direction),
                    visiting_value + 1,
                );
                to_visit.push((follow_point, current_direction))
            }
        }

        {
            let rotated_right = current_direction.rotate_right();

            update_value_map_if_better(
                &mut value_map,
                (visiting, current_direction),
                (visiting, rotated_right),
                visiting_value + 1000,
            );
            to_visit.push((visiting, rotated_right))
        }

        {
            let rotated_left = current_direction.rotate_left();
            update_value_map_if_better(
                &mut value_map,
                (visiting, current_direction),
                (visiting, rotated_left),
                visiting_value + 1000,
            );
            to_visit.push((visiting, rotated_left))
        }

        to_visit = to_visit
            .into_iter()
            .unique()
            .sorted_by_key(|v| -value_map.get(v).unwrap().0)
            .collect();
    }

    let end = find(&map, 'E');

    let best_direction = [
        Direction::Up,
        Direction::Left,
        Direction::Down,
        Direction::Right,
    ]
    .iter()
    .filter_map(|dir| Some((*dir, value_map.get(&(end, *dir))?)))
    .min_by_key(|(dir, (v, _))| *v)
    .unwrap()
    .0;

    let mut result_tiles: HashSet<Position> = Default::default();
    result_tiles.insert(end);
    let mut to_visit = value_map.get(&(end, best_direction)).unwrap().1.clone();

    while let Some(visiting) = to_visit.pop() {
        let pre = value_map.get(&visiting).unwrap().1.clone();
        result_tiles.insert(visiting.0);
        to_visit.extend(pre);
    }

    result_tiles.len()

}


fn update_value_map_if_better(
    value_map: &mut HashMap<(Position, Direction), (i64, Vec<(Position, Direction)>)>,
    origin: (Position, Direction),
    to_update: (Position, Direction),
    value: i64,
) {
    let current_follow_value = value_map.entry(to_update).or_insert((i64::MAX, vec![]));

    if current_follow_value.0 > value {
        *current_follow_value = (value, vec![origin]);
    } else if current_follow_value.0 == value {
        current_follow_value.1.push(origin)
    }
}

fn find(map: &[Vec<char>], value: char) -> Position {
    for row in 0..map.len() {
        for column in 0..map[row].len() {
            if map[row][column] == value {
                return Position { row, column };
            }
        }
    }
    panic!("No starting position found")
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Position {
    row: usize,
    column: usize,
}

impl Direction {
    fn rotate_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn rotate_left(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }
}

fn try_move(map: &[Vec<char>], start: Position, direction: Direction) -> Option<Position> {
    match direction {
        Direction::Up => {
            if start.row == 0 {
                None
            } else {
                Some(Position {
                    row: start.row - 1,
                    column: start.column,
                })
            }
        }
        Direction::Right => {
            if start.column + 1 == map[start.row].len() {
                None
            } else {
                Some(Position {
                    row: start.row,
                    column: start.column + 1,
                })
            }
        }
        Direction::Down => {
            if start.row + 1 == map.len() {
                None
            } else {
                Some(Position {
                    row: start.row + 1,
                    column: start.column,
                })
            }
        }
        Direction::Left => {
            if start.column == 0 {
                None
            } else {
                Some(Position {
                    row: start.row,
                    column: start.column - 1,
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45));
    }
}
