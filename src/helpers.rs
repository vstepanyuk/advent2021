use std::str::FromStr;

pub fn parse_lines<T: FromStr>(input: Option<String>) -> Vec<T> {
    input
        .unwrap_or_default()
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect()
}

pub trait VecAsMatrix<T> {
    fn neighbour_indexes(&self, index: usize, width: usize) -> Vec<usize>;
    fn neighbours(&self, index: usize, width: usize) -> Vec<&T>;
    fn at_position(&self, x: i32, y: i32, width: usize) -> Option<&T>;
}

impl<T> VecAsMatrix<T> for [T] {
    fn neighbour_indexes(&self, index: usize, width: usize) -> Vec<usize> {
        let x = index % width;
        let y = index / width;

        let mut result = vec![];
        if x > 0 {
            result.push(x - 1 + y * width);
        }

        if y > 0 {
            result.push(x + (y - 1) * width);
        }

        if (x + 1) < width {
            result.push(x + 1 + y * width);
        }

        if (x + (y + 1) * width) < self.len() {
            result.push(x + (y + 1) * width);
        }

        result
    }

    fn neighbours(&self, index: usize, width: usize) -> Vec<&T> {
        self.neighbour_indexes(index, width)
            .iter()
            .map(|&index| &self[index])
            .collect()
    }

    fn at_position(&self, x: i32, y: i32, width: usize) -> Option<&T> {
        let index = x + y * width as i32;

        if index < 0 || index >= self.len() as i32 {
            return None;
        }

        Some(&self[index as usize])
    }
}
