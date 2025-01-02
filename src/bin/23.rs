use std::collections::{HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(23);

pub fn part_one(input: &str) -> Option<u32> {

    let graph = get_graph(input);

    // println!("{:?}", graph);

    let mut result: HashSet<(String, String, String)> = HashSet::new();

    for (node, neighbours) in &graph {
        for n in neighbours {
            let intersection = graph.get(n).unwrap().intersection(neighbours).collect_vec();
            for i in intersection {
                if node.starts_with('t') || n.starts_with('t') || i.starts_with('t') {
                    let new = vec![node, n, i].iter().map(|&node| node.to_string()).sorted().collect_tuple().unwrap();
                    result.insert(new);
                }
            }
        }
    }

    // println!("{:?}", result);

    Some(result.len() as u32)
}

fn get_graph(input: &str) -> HashMap<String, HashSet<String>> {
    let connections = input.lines()
        .map(|line| line.split("-").collect_tuple().unwrap())
        .collect_vec();

    let mut graph = HashMap::new();

    for (a, b) in connections {
        graph.entry(a.to_string()).or_insert_with(HashSet::new).insert(b.to_string());
        graph.entry(b.to_string()).or_insert_with(HashSet::new).insert(a.to_string());
    }

    graph
}

// fn dfs(graph: &HashMap<&str, Vec<&str>>, start: &str, visited: &mut Vec<&str>, max_depth: usize) -> HashSet<(String, String, String)> {
//     visited.push(start);

//     if visited.len() == max_depth {
//         let new = visited.iter().map(|&node| node.to_string()).sorted().collect_tuple().unwrap();
//         let result = HashSet::new();
//         result.insert(new);
//         return result;
//     }

//     let mut result = HashSet::new();

//     for &node in graph.get(start).unwrap() {
//         if !visited.contains(&node) {
//             result.extend(dfs(graph, node, visited, max_depth));
//         }
//     }

//     result
// }



pub fn part_two(input: &str) -> Option<String> {
    let graph = get_graph(input);

    let vertices: HashSet<String> = graph.keys().map(|s| s.to_string()).collect();

    let result = bronkerbosch(&graph, &HashSet::new(), &vertices, &HashSet::new());

    println!("{:?}", result);

    Some(result.iter().sorted().join(","))
}

fn bronkerbosch(
    graph: &HashMap<String, HashSet<String>>,
    r: &HashSet<String>,
    p: &HashSet<String>,
    x: &HashSet<String>,
) -> HashSet<String> {
    let mut p_fp = p.clone();
    let mut x_fp = x.clone();

    if p.is_empty() && x.is_empty() {
        return r.clone();
    }

    let mut result = HashSet::new();

    for v in p.iter().cloned().collect_vec() {
        let neighbours = graph.get(&v).unwrap().clone();

        let mut r_union_v = r.clone();
        r_union_v.insert(v.clone());
        let p = p_fp.intersection(&neighbours).cloned().collect();
        let x = x_fp.intersection(&neighbours).cloned().collect();

        let b = bronkerbosch(graph, &r_union_v, &p, &x);
        if b.len() > result.len() {
            result = b;
        }

        p_fp.remove(&v);
        x_fp.insert(v);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("co,de,ka,ta".to_string()));
    }
}
