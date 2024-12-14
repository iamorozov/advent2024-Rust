use std::fs::File;
use std::io::prelude::*;

use itertools::Itertools;

advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<u32> {
    let height = 103;
    let width = 101;

    let quadrants = input.lines()
        .into_iter()
        .map(|line| sscanf::scanf!(line.trim(), "p={},{} v={},{}", i32, i32, i32, i32).unwrap())
        .map(|(px, py, vx, vy)| make_step(height, width, px, py, vx, vy, 100))
        .map(|(cx, cy)| get_quadrant(height, width, cx, cy))
        .collect_vec();

    let res = quadrants
        .into_iter()
        .flatten()
        .counts()
        .values()
        .map(|&v| v as u32)
        .product();

    Some(res)
}

fn make_step(h: i32, w: i32, px: i32, py: i32, vx: i32, vy: i32, step: i32) -> (i32, i32) {
    let x = (px + vx * step) % w;
    let y = (py + vy * step) % h;

    let cx = if x >= 0 { x } else { w + x };
    let cy = if y >= 0 { y } else { h + y };

    (cx, cy)
}

fn get_quadrant(h: i32, w: i32, cx: i32, cy: i32) -> Option<(i32, i32)> {
    let qx = if cx < w / 2 { -1 } else if cx == w / 2 { 0 } else { 1 };
    let qy = if cy < h / 2 { -1 } else if cy == h / 2 { 0 } else { 1 };

    match (qx, qy) {
        (0, _) | (_, 0) => None,
        _ => Some((qx, qy))
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let height = 103;
    let width = 101;

    let robots = input.lines()
        .into_iter()
        .map(|line| sscanf::scanf!(line.trim(), "p={},{} v={},{}", i32, i32, i32, i32).unwrap())
        .collect_vec();

    let mut file = File::create("steps.txt").expect("Unable to create file");

    (0..10000)
        .map(|step| {
            file.write_all(format!("Step: {step}\n").as_bytes()).expect("Unable to write data");
            let positions = robots
                .iter()
                .map(|(px, py, vx, vy)| make_step(height, width, *px, *py, *vx, *vy, step))
                .counts();

            (0..height)
                .map(|y| {
                    (0..width)
                        .map(|x| {
                            if positions.contains_key(&(x, y)) {
                                positions[&(x, y)].to_string().chars().nth(0).unwrap()
                            } else {
                                ' '
                            }
                        })
                        .collect::<String>()
                })
                .for_each(|line| {
                    file.write_all(line.as_bytes()).expect("Unable to write data");
                    file.write_all(b"\n").expect("Unable to write data");
                });
            file.write_all(b"XOXOXOXOXOXOXOXOXOXOXOXOXOXOXOXOXOXOXOXOXOXOXOXOXOXOXOXOXOXOXOXOXOXOXOXOXOXOXOXOXOXOXOXOXOXOXOXOXOXOX\n").expect("Unable to write data");
        })
        .collect_vec();

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
