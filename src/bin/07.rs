use itertools::Itertools;
use std::cmp::max;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let equasions = get_equasions(input);

    equasions
        .into_iter()
        .map(|(res, ops)| Some(make_equasion(res, ops) as u64))
        .sum()
}

fn make_equasion(res: i64, operands: Vec<i64>) -> i64 {
    fn rec(cur: i64, rem: &[i64], goal: i64) -> i64 {
        if rem.len() == 0 && cur == goal{
            goal
        } else if rem.len() == 0 {
            return 0
        } else {
            max(
                rec(cur + rem[0], &rem[1..], goal),
                rec(cur * rem[0], &rem[1..], goal)
            )
        }
    }

    rec(operands[0], &operands[1..], res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let equasions = get_equasions(input);

    equasions
        .into_iter()
        .map(|(res, ops)| Some(make_equasion_with_concat(res, ops) as u64))
        .sum()
}

fn get_equasions(input: &str) -> Vec<(i64, Vec<i64>)> {
    let equasions: Vec<(i64, Vec<i64>)> = input.lines()
        .map ( |line| line.split(":").collect_vec())
        .map ( |line| {
            let res = line[0].parse::<i64>().unwrap();
            let operands = line[1].trim().split_whitespace()
                .map(|n| n.parse::<i64>().unwrap()).collect_vec();

            (res, operands)
        })
        .collect_vec();
    equasions
}

fn make_equasion_with_concat(res: i64, operands: Vec<i64>) -> i64 {
    fn concat(a: i64, b: i64) -> i64 {
        let b_digits = (b as f64).log10().floor() as u32 + 1;
        let scale = 10_u32.pow(b_digits) as i64;
        a * scale + b
    }

    fn rec(cur: i64, rem: &[i64], goal: i64) -> i64 {
        if rem.len() == 0 && cur == goal {
            goal
        } else if rem.len() == 0 {
            return 0
        } else {
            max(
                rec(cur + rem[0], &rem[1..], goal),
                max(
                    rec(cur * rem[0], &rem[1..], goal),
                    rec(concat(cur, rem[0]), &rem[1..], goal)
                )
            )
        }
    }

    rec(operands[0], &operands[1..], res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
