use std::collections::HashMap;

use itertools::Itertools;
use memoize::memoize;

advent_of_code::solution!(21);

fn numeric() -> HashMap<char, (i32, i32)> {
    HashMap::from([
        ('0', (0, 1)),
        ('A', (0, 2)),
        ('1', (1, 0)),
        ('2', (1, 1)),
        ('3', (1, 2)),
        ('4', (2, 0)),
        ('5', (2, 1)),
        ('6', (2, 2)),
        ('7', (3, 0)),
        ('8', (3, 1)),
        ('9', (3, 2)),
    ])
}

fn directional() -> HashMap<char, (i32, i32)> {
    HashMap::from([
        ('<', (0, 0)),
        ('v', (0, 1)),
        ('>', (0, 2)),
        ('^', (1, 1)),
        ('A', (1, 2)),
    ])
}

pub fn part_one(input: &str) -> Option<usize> {
    let codes = input.lines().collect_vec();

    let mut result = 0;

    for code in codes {
        let num_pad = type_sequence_num(code, 'A');
        let dir_pad = type_sequence_dir(num_pad, 'A');
        let dir_pad_2 = type_sequence_dir(dir_pad, 'A');

        let final_len = dir_pad_2.len();
        let code_num = code[0..code.len()-1].parse::<usize>().unwrap();

        let complexity = final_len * code_num;
        result += complexity;

        // println!("code: {} -> {} -> {} -> {}", code, num_pad, dir_pad, dir_pad_2);
        // println!("code: {} -> len {} * num {} = {}", code, final_len, code_num, complexity);
    }

    Some(result)
}

// +---+---+---+
// | 7 | 8 | 9 |
// +---+---+---+
// | 4 | 5 | 6 |
// +---+---+---+
// | 1 | 2 | 3 |
// +---+---+---+
//     | 0 | A |
//     +---+---+

//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+

// <v<A >>^A vA ^A <vA  <A A   >>^A  A vA <^A >A A vA ^A <vA >^A A <A >A <v<A >A >^A A A vA <^A >A
// <    A    >  A  v    <  <   A
// v<<A ^>>A vA ^A v<<A ^>>AAv<A<A^>>AA<Av>AA^Av<A^>AA<A>Av<A<A^>>AAA<Av>A^A
// <AAA>Av<<AA^>>Av<AA>AA^Av<A^>A
// ^^^A<<Avv>>AvA
// 973A

fn type_sequence_num(code: &str, start: char) -> String {
    let pad = numeric();
    let (mut cur_x, mut cur_y) = pad[&start];
    let mut sequence = String::new();

    for c in code.chars() {
        let (x, y) = pad[&c];

        let dx = cur_x - x;
        let dy = cur_y - y;

        if cur_x == 0 && y == 0 {
            sequence.push_str(&"^".repeat(dx.abs() as usize));
            sequence.push_str(&"<".repeat(dy as usize));
        } else if cur_y == 0 && x == 0 {
            sequence.push_str(&">".repeat(dy.abs() as usize));
            sequence.push_str(&"v".repeat(dx as usize));
        } else {
            if dy > 0 {
                sequence.push_str(&"<".repeat(dy as usize));
            }

            if dx > 0 {
                sequence.push_str(&"v".repeat(dx as usize));
            }

            if dy < 0 {
                sequence.push_str(&">".repeat(dy.abs() as usize));
            }

            if dx < 0 {
                sequence.push_str(&"^".repeat(dx.abs() as usize));
            }
        }

        sequence.push('A');

        (cur_x, cur_y) = (x, y);
    }

    sequence
}

#[memoize]
fn type_sequence_dir(code: String, start: char) -> String {
    let pad = directional();
    let (mut cur_x, mut cur_y) = pad[&start];
    let mut sequence = String::new();

    for c in code.chars() {
        let (x, y) = pad[&c];

        let dx = cur_x - x;
        let dy = cur_y - y;

        if cur_x == 1 && y == 0 {
            sequence.push_str(&"v".repeat(dx as usize));
            sequence.push_str(&"<".repeat(dy as usize));
        } else if cur_y == 0 && x == 1 {
            sequence.push_str(&">".repeat(dy.abs() as usize));
            sequence.push_str(&"^".repeat(dx.abs() as usize));
        } else {
            if dy > 0 {
                sequence.push_str(&"<".repeat(dy as usize));
            }

            if dx > 0 {
                sequence.push_str(&"v".repeat(dx as usize));
            }

            if dy < 0 {
                sequence.push_str(&">".repeat(dy.abs() as usize));
            }

            if dx < 0 {
                sequence.push_str(&"^".repeat(dx.abs() as usize));
            }
        }

        sequence.push('A');

        (cur_x, cur_y) = (x, y);
    }

    sequence
}

fn moves() -> HashMap<char, HashMap<char, String>> {
    HashMap::from([
        ('A', HashMap::from([
            ('A', "A".to_string()),
            ('^', "<A".to_string()),
            ('<', "v<<A".to_string()),
            ('>', "vA".to_string()),
            ('v', "<vA".to_string()),
        ])),
        ('^', HashMap::from([
            ('A', ">A".to_string()),
            ('^', "A".to_string()),
            ('<', "v<A".to_string()),
            ('>', "v>A".to_string()),
            ('v', "vA".to_string()),
        ])),
        ('<', HashMap::from([
            ('A', ">>^A".to_string()),
            ('^', ">^A".to_string()),
            ('<', "A".to_string()),
            ('>', ">>A".to_string()),
            ('v', ">A".to_string()),
        ])),
        ('v', HashMap::from([
            ('A', "^>A".to_string()),
            ('^', "^A".to_string()),
            ('<', "<A".to_string()),
            ('>', ">A".to_string()),
            ('v', "A".to_string()),
        ])),
        ('>', HashMap::from([
            ('A', "^A".to_string()),
            ('^', "<^A".to_string()),
            ('<', "<<A".to_string()),
            ('>', "A".to_string()),
            ('v', "<A".to_string()),
        ])),
    ])
}

#[memoize]
fn type_sequence_dir_rec(from: char, to: char, robot: u32) -> u64 {
    let map = moves();

    if robot == 25 {
        return map[&from][&to].len() as u64;
    }

    let mut result = 0;
    let mut prev = 'A';

    for next in map[&from][&to].chars() {
        result += type_sequence_dir_rec(prev, next, robot + 1);
        prev = next;
    }

    result
}

pub fn part_two(input: &str) -> Option<u64> {
    let codes = input.lines().collect_vec();

    let mut result: u64 = 0;

    for code in codes {
        let num_pad = type_sequence_num(code, 'A');

        let mut final_len = 0;
        let mut prev = 'A';

        num_pad.chars().for_each(|c| {
            final_len += type_sequence_dir_rec(prev, c, 1);
            prev = c;
        });

        let code_num = code[0..code.len()-1].parse::<usize>().unwrap() as u64;

        let complexity = final_len * code_num;
        result += complexity as u64;

        println!("code: {} -> {}", code, final_len);
        // println!("code: {} -> len {} * num {} = {}", code, final_len, code_num, complexity);
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }


    #[test]
    fn test_type_num() {
        let result = type_sequence_num("7", 'A');
        assert_eq!(result, "^^^<<A");

        let result = type_sequence_num("4", 'A');
        assert_eq!(result, "^^<<A");

        let result = type_sequence_num("1", 'A');
        assert_eq!(result, "^<<A");

        let result = type_sequence_num("8", 'A');
        assert_eq!(result, "<^^^A");

        let result = type_sequence_num("7", '3');
        assert_eq!(result, "<<^^A");

        let result = type_sequence_num("A", '7');
        assert_eq!(result, ">>vvvA");
    }

    #[test]
    fn test_type_dir() {
        let result = type_sequence_dir(String::from("<"), 'A');
        assert_eq!(result, "v<<A");

        let result = type_sequence_dir(String::from("^"), '<');
        assert_eq!(result, ">^A");

        let result = type_sequence_dir(String::from("A"), '<');
        assert_eq!(result, ">>^A");

        let result = type_sequence_dir(String::from("^"), '>');
        assert_eq!(result, "<^A");
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
