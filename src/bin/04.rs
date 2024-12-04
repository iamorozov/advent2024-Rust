use itertools::Itertools;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let matrix = get_matrix(input);
    let result = find_xmas(&matrix);

    return Some(result);
}

pub fn part_two(input: &str) -> Option<u32> {
    let matrix = get_matrix(input);
    let result = find_x_mas(&matrix);

    return Some(result);
}

fn get_matrix(input: &str) -> Vec<Vec<char>> {
    return input.lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
}

fn find_xmas(matrix: &Vec<Vec<char>>) -> u32 {
    let mut count = 0;
    let directions: [(i32, i32); 8] = [(0, 1), (0, -1), (1, 0), (-1, 0), (1, 1), (-1, -1), (1, -1), (-1, 1)];
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == 'X' {
                for &(dx, dy) in &directions {
                    let mut x = i;
                    let mut y = j;
                    let mut word = String::new();

                    while x < matrix.len() && y < matrix[x].len() {
                        word.push(matrix[x][y]);
                        if word.len() == 4 && word == "XMAS" {
                            count += 1;
                            break;
                        }

                        if dx == 1 {
                            x += 1;
                        } else if dx == -1 {
                            if x == 0 {
                                break;
                            }
                            x -= 1;
                        }

                        if dy == 1 {
                            y += 1;
                        } else if dy == -1 {
                            if y == 0 {
                                break;
                            }
                            y -= 1;
                        }
                    }
                }
            }
        }
    }
    count
}

fn find_x_mas(matrix: &Vec<Vec<char>>) -> u32 {
    let mut count = 0;
    let directions: [(i32, i32); 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
    for i in 1..matrix.len()-1 {
        for j in 1..matrix[i].len()-1 {
            if matrix[i][j] == 'A' {
                let mut word = String::new();
                let mut x = 0;
                let mut y = 0;

                for &(dx, dy) in &directions {
                    if dx == 1 {
                        x = i + 1;
                    } else if dx == -1 {
                        x = i - 1;
                    }

                    if dy == 1 {
                        y = j + 1;
                    } else if dy == -1 {
                        y = j - 1;
                    }
                    word.push(matrix[x][y]);
                }

                if ["MSMS", "SMSM", "MMSS", "SSMM"].contains(&word.as_str()) {
                    count += 1;
                }
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
