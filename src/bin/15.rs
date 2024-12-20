use std::cmp::min;

use itertools::Itertools;
use advent_of_code::utils::*;

advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u64> {
    let (mut map, moves) = get_map_and_moves(input);

    let (mut rx, mut ry) = find_position_in_matrix(&map, '@');

    for (dx, dy) in moves {
        let x = rx as i32 + dx;
        let y = ry as i32 + dy;

        // if wall continue
        if map[x as usize][y as usize] == '#' {
            continue;
        }

        // if empty space move
        if map[x as usize][y as usize] == '.' {
            map[rx][ry] = '.';
            rx = x as usize;
            ry = y as usize;
            map[rx][ry] = '@';
            continue;
        }

        // if box move all boxes in that direction if there is empty space
        if map[x as usize][y as usize] == 'O' {
            let mut bx = x;
            let mut by = y;

            while map[bx as usize][by as usize] == 'O' {
                bx += dx;
                by += dy;
            }

            if map[bx as usize][by as usize] == '.' {
                map[rx][ry] = '.';
                rx = x as usize;
                ry = y as usize;
                map[rx][ry] = '@';
                map[bx as usize][by as usize] = 'O';
            }
        }
    }

    // print_map(&map);

    // calculate box coordinates
    let result = map.iter()
        .enumerate()
        .map(|(i, row)| row.iter()
            .enumerate()
            .filter(|(_, ch)| **ch == 'O')
            .map(|(j, _)| i * 100 + j)
            .sum::<usize>())
        .sum::<usize>();

    Some(result as u64)
}



fn print_map(map: &Vec<Vec<char>>) {
    for row in map {
        for ch in row {
            print!("{}", ch);
        }
        println!();
    }
}

fn get_map_and_moves(input: &str) -> (Vec<Vec<char>>, Vec<(i32, i32)>) {
    let p = input.lines()
        .position(|line| line.is_empty())
        .unwrap();

    let map = input.lines()
        .take(p)
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let moves = get_moves(input, p);

    return (map, moves);
}

fn get_moves(input: &str, p: usize) -> Vec<(i32, i32)> {
    let moves = input.lines()
        .skip(p + 1)
        .map(|line| line.trim().chars().collect_vec())
        .flatten()
        .map(|ch| match ch {
            '<' => (0, -1),
            '>' => (0, 1),
            'v' => (1, 0),
            '^' => (-1, 0),
            _ => panic!("Invalid move")

        })
        .collect_vec();
    moves
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut map, moves) = get_wide_map_and_moves(input);

    print_map(&map);

    let (mut rx, mut ry) = find_position_in_matrix(&map, '@');

    // println!("rx: {}, ry: {}", rx, ry);

    for (dx, dy) in moves {
        // print_map(&map);
        // println!("dx: {}, dy: {}", dx, dy);

        let x = rx as i32 + dx;
        let y = ry as i32 + dy;

        // if wall continue
        if map[x as usize][y as usize] == '#' {
            continue;
        }

        // if empty space move
        if map[x as usize][y as usize] == '.' {
            map[rx][ry] = '.';
            rx = x as usize;
            ry = y as usize;
            map[rx][ry] = '@';
            continue;
        }

        // if box move all boxes in that direction if there is empty space
        if map[x as usize][y as usize] == '[' || map[x as usize][y as usize] == ']' {
            // if move is horizontal move all boxes in that direction
            if dx == 0 {
                let mut by = y;

                while map[x as usize][by as usize] == '[' || map[x as usize][by as usize] == ']' {
                    by += dy;
                }

                if map[x as usize][by as usize] == '.' {
                    // move all boxes to the empty space
                    while by != y as i32 {
                        map[x as usize][by as usize] = map[x as usize][(by - dy) as usize];
                        by -= dy;
                    }

                    // move robot to that direction
                    map[rx][ry] = '.';
                    rx = x as usize;
                    ry = y as usize;
                    map[rx][ry] = '@';
                }
            } else {
                // get box coordinates
                let (bx, by1, by2) = if map[x as usize][y as usize] == '[' {
                    (x, y, y + 1)
                } else {
                    (x, y - 1, y)
                };
                // move all boxes
                if can_move_boxes(&map, bx, by1, by2, dx) {
                    // println!("start moving boxes!!!");

                    move_boxes(&mut map, bx, by1, by2, dx);
                    map[rx][ry] = '.';
                    rx = x as usize;
                    ry = y as usize;
                    map[rx][ry] = '@';

                    // println!("end moving boxes!!!");
                }
            }
        }
    }

    // calculate box coordinates
    // let result = map.iter()
    //     .enumerate()
    //     .map(|(i, row)| row.iter()
    //         .enumerate()
    //         .filter(|(_, ch)| **ch != '#')
    //         .map(|(j, ch)| (min(i, map.len() - i - 1) * 100 + min(j, row.len() - j - 2)) * if *ch == '[' { 1 } else { 0 })
    //         .sum::<usize>())
    //     .sum::<usize>();

    print_map(&map);

    let result = map.iter()
        .enumerate()
        .map(|(i, row)| row.iter()
            .enumerate()
            .filter(|(_, ch)| **ch == '[')
            .map(|(j, _)| i * 100 + j)
            .sum::<usize>())
        .sum::<usize>();

    Some(result as u64)
}

fn can_move_boxes(map: &Vec<Vec<char>>, bx: i32, by1: i32, by2: i32, dx: i32) -> bool {
    match (map[(bx + dx) as usize][by1 as usize], map[(bx + dx) as usize][by2 as usize]) {
        ('.', '.') => true,
        ('#', _) | (_, '#') => false,
        ('[', ']') => can_move_boxes(map, bx + dx, by1, by2, dx),
        ('.', '[') => can_move_boxes(map, bx + dx, by2, by2 + 1, dx),
        (']', '.') => can_move_boxes(map, bx + dx, by1 - 1, by1, dx),
        (']', '[') => can_move_boxes(map, bx + dx, by1 - 1, by1, dx) && can_move_boxes(map, bx + dx, by2, by2 + 1, dx),
        _ => false
    }
}

fn move_boxes(map: &mut Vec<Vec<char>>, bx: i32, by1: i32, by2: i32, dx: i32) {
    fn move_box(map: &mut Vec<Vec<char>>, bx: i32, by1: i32, by2: i32, dx: i32) {
        map[bx as usize][by1 as usize] = '.';
        map[bx as usize][by2 as usize] = '.';
        map[(bx + dx) as usize][by1 as usize] = '[';
        map[(bx + dx) as usize][by2 as usize] = ']';

        // print_map(map);
    }

    match (map[(bx + dx) as usize][by1 as usize], map[(bx + dx) as usize][by2 as usize]) {
        ('.', '.') => move_box(map, bx, by1, by2, dx),
        ('[', ']') => {
            move_boxes(map, bx + dx, by1, by2, dx);
            move_box(map, bx, by1, by2, dx)
        },
        ('.', '[') => {
            move_boxes(map, bx + dx, by2, by2 + 1, dx);
            move_box(map, bx, by1, by2, dx)
        },
        (']', '.') => {
            move_boxes(map, bx + dx, by1 - 1, by1, dx);
            move_box(map, bx, by1, by2, dx)
        },
        (']', '[') => {
            move_boxes(map, bx + dx, by1 - 1, by1, dx);
            move_boxes(map, bx + dx, by2, by2 + 1, dx);
            move_box(map, bx, by1, by2, dx)
        },
        _ => panic!("Cannot move boxes {}, {}, {}, {}", bx, by1, by2, dx),
    }
}

fn get_wide_map_and_moves(input: &str) -> (Vec<Vec<char>>, Vec<(i32, i32)>) {
    let p = input.lines()
        .position(|line| line.is_empty())
        .unwrap();

    let map = input.lines()
        .take(p)
        .map(|line| line.chars()
            .map(|ch| match ch {
                '#' => ['#', '#'],
                'O' => ['[', ']'],
                '.' => ['.', '.'],
                '@' => ['@', '.'],
                _ => panic!("Invalid character")
            })
            .flatten()
            .collect_vec()
        )
        .collect_vec();

    let moves = get_moves(input, p);

    return (map, moves);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result: Option<u64> = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
