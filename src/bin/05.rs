use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, updates) = get_rules_and_updates(input);

    let (rules_pre, rules_post) = convert_rules(rules);

    let result = validate_updates(&rules_pre, &rules_post, &updates);

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, updates) = get_rules_and_updates(input);

    let (rules_pre, rules_post) = convert_rules(rules);

    let result = count_invalid(&rules_pre, &rules_post, &updates);

    Some(result)
}

fn convert_rules(rules: Vec<(u32, u32)>) -> (HashMap<u32, HashSet<u32>>, HashMap<u32, HashSet<u32>>) {
    let mut rules_pre: HashMap<u32, HashSet<u32>> = HashMap::new();
    let mut rules_post: HashMap<u32, HashSet<u32>> = HashMap::new();
    for (n1, n2) in rules.into_iter() {
        rules_pre.entry(n1).or_insert(HashSet::new()).insert(n2);
        rules_post.entry(n2).or_insert(HashSet::new()).insert(n1);
    }
    (rules_pre, rules_post)
}

fn get_rules_and_updates(input: &str) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let p = input.lines()
        .position(|line| line.is_empty())
        .unwrap();

    let rules = input.lines()
        .take(p)
        .map(|line| line.split("|")
            .map(|n| n.parse::<u32>().unwrap())
            .collect_tuple()
            .unwrap()
        )
        .collect_vec();

    let updates = input.lines()
        .skip(p + 1)
        .map(|line| line.split(",")
            .map(|n| n.parse::<u32>().unwrap())
            .collect_vec()
        )
        .collect_vec();

    return (rules, updates);
}

fn validate_updates(rules_pre: &HashMap<u32, HashSet<u32>>, rules_post: &HashMap<u32, HashSet<u32>>, updates: &Vec<Vec<u32>>) -> u32 {
    updates
        .iter()
        .filter(|update| is_valid_update(update, rules_pre, rules_post))
        .map(|update| update[update.len() / 2])
        .sum()
}

fn count_invalid(rules_pre: &HashMap<u32, HashSet<u32>>, rules_post: &HashMap<u32, HashSet<u32>>, updates: &Vec<Vec<u32>>) -> u32 {
    updates
        .iter()
        .filter(|update| !is_valid_update(update, rules_pre, rules_post))
        .map(|update| {
            let mut copy = update.into_iter().clone().collect_vec();
            copy.sort_by(|a, b| compare(rules_pre, rules_post, a, b));
            copy[copy.len() / 2]
        }
        )
        .sum()
}

fn compare(rules_pre: &HashMap<u32, HashSet<u32>>, rules_post: &HashMap<u32, HashSet<u32>>, a: &u32, b: &u32) -> std::cmp::Ordering {
    let empty: HashSet<u32> = HashSet::new();
    if rules_pre.get(a).unwrap_or(&empty).contains(b) {
        return std::cmp::Ordering::Less;
    } else if rules_post.get(a).unwrap_or(&empty).contains(b) {
        return std::cmp::Ordering::Greater;
    } else {
        return std::cmp::Ordering::Equal;
    }
}

fn is_valid_update(update: &Vec<u32>, rules_pre: &HashMap<u32, HashSet<u32>>, rules_post: &HashMap<u32, HashSet<u32>>) -> bool {
    let mut valid = true;
    for (i, n) in update.iter().enumerate() {
        let left: &HashSet<u32> = &update[0..i].into_iter().copied().collect();
        let right: &HashSet<u32> = &update[i+1..].into_iter().copied().collect();

        let empty = HashSet::new();
        let pre = rules_pre.get(n).unwrap_or(&empty);
        let post = rules_post.get(n).unwrap_or(&empty);
        let left_post: HashSet<u32> = left.intersection(pre).into_iter().copied().collect();
        let right_pre: HashSet<u32> = right.intersection(post).into_iter().copied().collect();

        if left_post.len() != 0 || right_pre.len() != 0 {
            valid = false;
            break;
        }
    }
    valid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
