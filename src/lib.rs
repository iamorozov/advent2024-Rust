pub mod template;

pub mod utils {
    use itertools::Itertools;

    pub fn get_char_matrix(input: &str) -> Vec<Vec<char>> {
        return input.lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec();
    }

    pub fn get_int_matrix(input: &str) -> Vec<Vec<u32>> {
        return input.lines()
            .map(|line| line.chars()
                .map(|ch| ch.to_digit(10).unwrap())
                .collect_vec()
            )
            .collect_vec();
    }
}
