use itertools::Itertools;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines().collect_vec();
    let n = lines.len();
    let m = lines[0].len();

    let antennas = get_antennas(lines);

    let result = antennas
        .values()
        .map(|idx| get_antinodes(idx, m, n))
        .flatten()
        .unique()
        .count();

    // println!("{:?}", antennas);

    Some(result as u32)
}

fn get_antennas(lines: Vec<&str>) -> std::collections::HashMap<char, Vec<(usize, usize)>> {
    let antennas = lines
        .iter()
        .enumerate()
        .map(|(i, line)| line.char_indices()
            .filter_map(move |(j, ch)| if ch != '.' { Some((ch, (i, j))) } else { None })
            .collect_vec()
        )
        .flatten()
        .into_group_map();
    antennas
}

fn get_antinodes(antennas: &Vec<(usize, usize)>, n: usize, m: usize) -> Vec<(i32, i32)> {
    antennas
        .into_iter()
        .tuple_combinations()
        .map(|((x1, y1), (x2, y2))| {
            let (a1, a2) = get_pair_antinodes(*x1, *y1, *x2, *y2);
            [a1, a2]
        })
        .flatten()
        .filter(|(x, y)| *x >= 0 && *x < n as i32 && *y >= 0 && *y < m as i32)
        .collect_vec()
}

fn get_pair_antinodes(x1: usize, y1: usize, x2: usize, y2: usize) -> ((i32, i32), (i32, i32)) {

    let dx = (x1 as i32 - x2 as i32).abs();
    let a1_x = if x1 < x2 { x1 as i32 - dx } else { x1 as i32 + dx };
    let a2_x = if x1 < x2 { x2 as i32 + dx } else { x2 as i32 - dx };

    let dy = (y1 as i32 - y2 as i32).abs();
    let a1_y = if y1 < y2 { y1 as i32 - dy } else { y1 as i32 + dy };
    let a2_y = if y1 < y2 { y2 as i32 + dy } else { y2 as i32 - dy };

    ((a1_x, a1_y), (a2_x, a2_y))
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines().collect_vec();
    let n = lines.len();
    let m = lines[0].len();

    let antennas = get_antennas(lines);

    let result = antennas
        .values()
        .map(|idx| get_antinodes_2(idx, m, n))
        .flatten()
        .unique()
        .count();

    // println!("{:?}", antennas);

    Some(result as u32)
}

fn get_antinodes_2(antennas: &Vec<(usize, usize)>, n: usize, m: usize) -> Vec<(i32, i32)> {
    antennas
        .into_iter()
        .tuple_combinations()
        .map(|((x1, y1), (x2, y2))| get_all_antinodes(*x1, *y1, *x2, *y2))
        .flatten()
        .filter(|(x, y)| *x >= 0 && *x < n as i32 && *y >= 0 && *y < m as i32)
        .collect_vec()
}

fn get_all_antinodes(x1: usize, y1: usize, x2: usize, y2: usize) -> Vec<(i32, i32)> {
    let ddx = (x1 as i32 - x2 as i32).abs();
    let ddy = (y1 as i32 - y2 as i32).abs();

    let mut result: Vec<(i32, i32)> = vec![];

    for i in 0..50 {
        let dx = ddx * i;
        let dy = ddy * i;
        let a1_x = if x1 < x2 { x1 as i32 - dx } else { x1 as i32 + dx };
        let a2_x = if x1 < x2 { x2 as i32 + dx } else { x2 as i32 - dx };

        let a1_y = if y1 < y2 { y1 as i32 - dy } else { y1 as i32 + dy };
        let a2_y = if y1 < y2 { y2 as i32 + dy } else { y2 as i32 - dy };

        result.push((a1_x, a1_y));
        result.push((a2_x, a2_y));
    }


    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
