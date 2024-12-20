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

    pub fn find_position_in_matrix(matrix: &Vec<Vec<char>>, target: char) -> (usize, usize) {
        let x = matrix.iter()
            .position(|row| row.contains(&target))
            .unwrap();

        let y = matrix[x].iter().position(|ch| ch == &target).unwrap();

        (x, y)
    }
}
