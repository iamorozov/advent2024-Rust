use std::{collections::HashMap, result};

use itertools::Itertools;
use sscanf::sscanf;

advent_of_code::solution!(24);

pub fn part_one(input: &str) -> Option<u64> {
    let (wires, mut gates) = get_wires_and_gates(input);

    // println!("wires: {:?}", wires);
    // println!("gates: {:?}", gates);

    let mut result = Vec::new();

    for (name, gate) in gates.clone().into_iter() {
        if name.starts_with('z') {
            result.push((name, eval(gate.clone(), &wires, &mut gates)));
        }
    }

    let bin = result.iter()
        .sorted_by_key(|(name, _)| name.clone())
        .map(|e| e.1)
        .rev()
        .collect_vec();

    Some(bin_vec_to_dec(bin))
}

fn get_wires_and_gates(input: &str) -> (HashMap<String, u32>, HashMap<String, Gate>) {
    let p = input.lines().find_position(|line| line.is_empty()).unwrap().0;

    let wires: HashMap<String, u32> = input.lines().take(p)
        .map(|line| sscanf!(line, "{}: {}", String, u32).unwrap())
        .collect();

    let mut gates: HashMap<String, Gate> = input.lines().skip(p + 1)
        .map(|line| {
            let (i1, op, i2, name) = sscanf!(line, "{} {} {} -> {}", String, String, String, String).unwrap();
            (name, Gate::Expr { op, i1, i2 })
        })
        .collect();

    (wires, gates)
}

fn eval(gate: Gate, wires: &HashMap<String, u32>, gates: &mut HashMap<String, Gate>) -> u32 {
    match gate {
        Gate::Expr { op, i1, i2 } => {
            let op1 = if wires.contains_key(&i1) {
                *wires.get(&i1).unwrap()
            } else {
                eval(gates[&i1].clone(), wires, gates)
            };

            gates.insert(i1, Gate::Result { val: op1 });

            let op2 = if wires.contains_key(&i2) {
                *wires.get(&i2).unwrap()
            } else {
                eval(gates[&i2].clone(), wires, gates)
            };
            gates.insert(i2, Gate::Result { val: op2 });

            match op.as_str() {
                "AND" => op1 & op2,
                "OR" => op1 | op2,
                "XOR" => op1 ^ op2,
                _ => panic!("Unknown operator: {}", op),
            }
        },
        Gate::Result { val } => val,
    }
}

fn bin_vec_to_dec(bin: Vec<u32>) -> u64 {
    bin.iter().rev().enumerate().map(|(i, &b)| (b as u64) << i).sum()
}

#[derive(Debug, Clone)]
enum Gate {
    Expr { op: String, i1: String, i2: String },
    Result { val: u32 },
}

pub fn part_two(input: &str) -> Option<u32> {
    let (wires, gates) = get_wires_and_gates(input);

    // let mut result = 0 as u64;

    // for i in 0..2363807160 as u64 {
    //     result += 1;
    // }

    // println!("result: {:?}", result);

    let mut result = Vec::new();

    for (name, gate) in gates.clone().into_iter() {
        match gate {
            Gate::Expr { op, i1, i2 } => {
                if name.starts_with('z') && op != "XOR" && name != "z45" {
                    println!("rule 1: Gate({} {} {} -> {})", i1, op, i2, name);
                    result.push(name.clone());
                }

                else if !name.starts_with('z') && !(i1.starts_with('x') && i2.starts_with('y') || i1.starts_with('y') && i2.starts_with('x')) && op == "XOR" {
                    println!("rule 2: Gate({} {} {} -> {})", i1, op, i2, name);
                    result.push(name.clone());
                }

                else if op == "XOR" && (i1.starts_with('x') && i2.starts_with('y') || i1.starts_with('y') && i2.starts_with('x')) && (i1 != "x00" && i2 != "y00") {

                    if find_xor_with_input(&gates, name.clone()).is_none() {
                        println!("rule 3: Gate({} {} {} -> {})", i1, op, i2, name);
                        result.push(name.clone());
                    }
                }

                else if op == "AND" && (i1 != "x00" && i2 != "y00") {

                    if find_or_with_input(&gates, name.clone()).is_none() {
                        println!("rule 4: Gate({} {} {} -> {})", i1, op, i2, name);
                        result.push(name.clone());
                    }
                }
            },
            _ => {},
        }
    }

    println!("result: {:?}", result.into_iter().sorted().collect_vec().join(","));

    None
}

fn find_xor_with_input(gates: &HashMap<String, Gate>, name: String) -> Option<String> {
    for (n, gate) in gates.clone().into_iter() {
        match gate {
            Gate::Expr { op, i1, i2 } => {
                if op == "XOR" && (i1 == name || i2 == name) {
                    return Some(n);
                }
            },
            _ => {},
        }
    }

    None
}

fn find_or_with_input(gates: &HashMap<String, Gate>, name: String) -> Option<String> {
    for (n, gate) in gates.clone().into_iter() {
        match gate {
            Gate::Expr { op, i1, i2 } => {
                if op == "OR" && (i1 == name || i2 == name) {
                    return Some(n);
                }
            },
            _ => {},
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
        assert_eq!(result, Some(2024));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
