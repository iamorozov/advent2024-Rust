use std::result;

use itertools::Itertools;

advent_of_code::solution!(17);

pub fn part_one(input: &str) -> Option<String> {

    let lines = input.lines().collect::<Vec<&str>>();
    let mut A = sscanf::sscanf!(&lines[0], "Register A: {}", i64).unwrap();
    let mut B = sscanf::sscanf!(&lines[1], "Register B: {}", i64).unwrap();
    let mut C = sscanf::sscanf!(&lines[2], "Register C: {}", i64).unwrap();

    let program = sscanf::sscanf!(&lines[4], "Program: {}", String).unwrap()
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect_vec();

    println!("A: {}, B: {}, C: {}, Program: {:?}", A, B, C, program);

    let output = eval(&program, A, B, C);

    Some(output.into_iter().join(","))
}

fn eval(program: &Vec<i64>, mut A: i64, mut B: i64, mut C: i64) -> Vec<i64> {
    let mut ptr = 0;

    let mut output = vec![];

    while ptr < program.len() {
        let opcode = program[ptr];
        let operand = program[ptr + 1];

        match opcode {
            // adv
            0 => A = A / ((2 as i64).pow(combo(operand, A, B, C) as u32)),
            // bxl
            1 => B = B ^ operand,
            // bst
            2 => B = combo(operand, A, B, C) % 8,
            // jnz
            3 => if A != 0 {
                    ptr = operand as usize;
                    continue;
                },
            // bxc
            4 => B = B ^ C,
            // out
            5 => output.push(combo(operand, A, B, C) % 8),
            // bdv
            6 => B = A / ((2 as i64).pow(combo(operand, A, B, C) as u32)),
            // cdv
            7 => C = A / ((2 as i64).pow(combo(operand, A, B, C) as u32)),
            _ => panic!("incorrect opcode")
        }

        ptr += 2;
    }
    output
}

fn combo(operand: i64, A: i64, B: i64, C: i64) -> i64 {
    match operand {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => A,
        5 => B,
        6 => C,
        _ => panic!("incorrect combo operand")
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = input.lines().collect::<Vec<&str>>();

    let program = sscanf::sscanf!(&lines[4], "Program: {}", String).unwrap()
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect_vec();

    // let program = eval(program, A, 0, 0);
    // println!("Program: {:?}", program);

    // let mut result: u64 = 0;

    // for p in program.clone().into_iter().rev() {
    //     let mut A: u64 = 0;

    //     loop {
    //         let output = eval(&program, A as i64, 0, 0);

    //         if output[0] == p {
    //             result = (result * 8) + A;
    //             println!("A: {}, P: {}, result: {}", A, p, result);
    //             break;
    //         }

    //         A += 1;
    //     }
    // }

    // Some(result)

    let mut factors = vec![0i64; program.len()];

    loop {
        let A: i64 = factors.iter().enumerate().map(|(index, v)| v * 2i64.pow(3 * index as u32)).sum();

        let output: Vec<i64> = eval(&program, A, 0, 0).into_iter().collect();
        if output == program {
            return Some(A as u64);
        }

        for i in (0..program.len()).rev() {
            if output.len() < i {
                factors[i] += 1;
                break;
            }
            if output[i] != program[i] {
                factors[i] += 1;
                break;
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
        assert_eq!(result, Some(String::from("4,6,3,5,6,3,5,2,1,0")));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(117440));
    }

    #[test]
    fn test() {
        print!("{}", 6323144925690 as u64 % 8)
    }
}
