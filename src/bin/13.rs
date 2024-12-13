use std::cmp::min;
use memoize::memoize;

use itertools::Itertools;
advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<u64> {
    let tokens = extract_machines(input, 0)
        .into_iter()
        .map(|(ax, ay, bx, by, prize_x, prize_y)| calculate_tokens(ax, ay, bx, by, prize_x, prize_y))
        .flatten()
        .sum();

    Some(tokens)
}

fn extract_machines(input: &str, correction: i64) -> Vec<(i64, i64, i64, i64, i64, i64)> {
    input.lines()
        .chunks(4)
        .into_iter()
        .map(|chunk| {
            let machine = chunk.collect::<Vec<&str>>();
            let (ax, ay) = sscanf::scanf!(machine[0], "Button A: X+{}, Y+{}", i64, i64).unwrap();
            let (bx, by) = sscanf::scanf!(machine[1], "Button B: X+{}, Y+{}", i64, i64).unwrap();
            let (prize_x, prize_y) = sscanf::scanf!(machine[2], "Prize: X={}, Y={}", i64, i64).unwrap();

            // println!("A ({}, {}) -> B ({}, {}) -> Prize ({}, {})", ax, ay, bx, by, prize_x, prize_y);

            (ax, ay, bx, by, prize_x + correction, prize_y + correction)
        })
        .collect_vec()
}

#[memoize]
fn calculate_tokens(ax: i64, ay: i64, bx: i64, by: i64, prize_x: i64, prize_y: i64) -> Option<u64> {
    // println!("A ({}, {}) -> B ({}, {}) -> Prize ({}, {})", ax, ay, bx, by, prize_x, prize_y);

    if prize_x == 0 && prize_y == 0 {
        return Some(0);
    }
    if prize_x < 0 || prize_y < 0 {
        return None;
    }

    let press_a = calculate_tokens(ax, ay, bx, by, prize_x - ax, prize_y - ay);
    let press_b = calculate_tokens(ax, ay, bx, by, prize_x - bx, prize_y - by);

    match (press_a, press_b) {
        (Some(a), Some(b)) => Some(min(3 + a, 1 + b)),
        (Some(a), None) => Some(3 + a),
        (None, Some(b)) => Some(1 + b),
        (None, None) => None,
    }
}

fn solve_machine(ax: i64, ay: i64, bx: i64, by: i64, prize_x: i64, prize_y: i64) -> i64 {
    let prize = (prize_x, prize_y);
    let det = ax * by - ay * bx;
    let a = (prize.0 * by - prize.1 * bx) / det;
    let b = (ax * prize.1 - ay * prize.0) / det;
    if (ax * a + bx * b, ay * a + by * b) == (prize.0, prize.1) {
        a * 3 + b
    } else {
        0
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let tokens = extract_machines(input, 10000000000000)
        .into_iter()
        .map(|(ax, ay, bx, by, prize_x, prize_y)| solve_machine(ax, ay, bx, by, prize_x, prize_y))
        .sum::<i64>() as u64;

    Some(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
