use sscanf::sscanf;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re = regex::Regex::new(r"mul\(\d{1,3},\d{1,3}\)").ok()?;
    let sum = re.find_iter(input)
        .map(|m| m.as_str())
        .map(|e| sscanf!(e, "mul({},{})", u32, u32).unwrap())
        .map(|(a, b)| a * b)
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = regex::Regex::new(r"mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\)").ok()?;
    let exprs: Vec<&str> = re.find_iter(input)
        .map(|m| m.as_str())
        .collect();

    let mut enabled = true;
    let mut sum = 0;
    for e in exprs {
        match e {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ if enabled => {
                let (a, b) = sscanf!(e, "mul({},{})", u32, u32).unwrap();
                sum += a * b;
            },
            _ => (),
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
