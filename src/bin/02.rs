use advent_of_code::utils::get_int_matrix;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let reports = get_int_matrix(input);
    Some(count_safe_reports(reports))
}

pub fn part_two(input: &str) -> Option<u32> {
    let reports = get_int_matrix(input);
    Some(count_safe_reports_with_dampener(reports))
}

fn count_safe_reports(reports: Vec<Vec<u32>>) -> u32 {
    reports.iter()
        .filter(|report| is_safe(report))
        .count() as u32
}

fn count_safe_reports_with_dampener(reports: Vec<Vec<u32>>) -> u32 {
    reports.iter()
        .filter(|report| is_safe_with_dampener(report))
        .count() as u32
}

fn is_safe(report: &Vec<u32>) -> bool {
    is_safe_increase(report) || is_safe_decrease(report)
}

fn is_safe_with_dampener(report: &Vec<u32>) -> bool {
    if is_safe_increase(report) || is_safe_decrease(report) {
        return true;
    }

    (0..report.len()).any(|i| {
        let mut removed = report.clone();
        removed.remove(i);
        is_safe_increase(&removed) || is_safe_decrease(&removed)
    })
}

fn is_safe_increase(nums: &Vec<u32>) -> bool {
    nums.windows(2).all(|w| w[1] <= w[0] + 3 && w[1] > w[0])
}

fn is_safe_decrease(nums: &Vec<u32>) -> bool {
    nums.windows(2).all(|w| w[0] <= w[1] + 3 && w[0] > w[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
