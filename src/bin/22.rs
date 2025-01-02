use std::{cmp::max, collections::HashMap, hash::Hash};

use itertools::{Itertools};

advent_of_code::solution!(22);

pub fn part_one(input: &str) -> Option<u64> {
    let result = input.lines()
        .map(|line| line.parse::<u64>().unwrap())
        .map(|secret| generate(secret, 2000))
        .collect::<Vec<u64>>();

    // println!("{:?}", result);

    result.iter()
        .sum::<u64>()
        .into()
}

fn generate(secret: u64, n: u64) -> u64 {
    let mut cur = secret;

    for _ in 0..n {
        cur = next(cur);
    }

    cur
}

fn next(cur: u64) -> u64 {
    let mut next = cur;

    next = (next ^ (next << 6)) & 16777215;

    next = (next ^ (next >> 5)) & 16777215;

    (next ^ (next << 11)) & 16777215
}

pub fn part_two(input: &str) -> Option<u64> {
    let prices = input.lines()
        .map(|line| line.parse::<u64>().unwrap())
        .map(|secret| generate_seq(secret, 2000))
        .collect_vec();

    let mut acc = HashMap::new();

    for price in prices {
        let mut acc_buyer = HashMap::new();

        for (p1, p2, p3, p4) in price.into_iter().tuple_windows() {
            acc_buyer.entry((p1.1, p2.1, p3.1, p4.1)).or_insert(p4.0);
        }

        // println!("{:?}", acc_buyer.entry((-2, 1, -1, 3)));

        for (k, v) in acc_buyer {
            *acc.entry(k).or_insert(0) += v;
        }
    }

    // return key with max value
    acc.into_iter()
        .max_by_key(|(_, v)| *v)
        .map(|(k, v)| { println!("{:?}", k); v })
}

fn generate_seq(secret: u64, n: u64) -> Vec<(u64, i64)> {
    let mut prev = secret;
    let mut prev_price = prev % 10;
    let mut result = Vec::new();

    for _ in 0..n {
        let cur = next(prev);
        let cur_price = cur % 10;

        result.push((cur_price, cur_price as i64 - prev_price as i64));

        prev = cur;
        prev_price = cur_price;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(23));
    }
}
