use std::fmt::{Debug, Formatter};
use std::str::FromStr;

pub fn parse_lines<T: FromStr>(input: Option<String>) -> Vec<T> {
    input
        .unwrap_or_default()
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect()
}

pub struct Matrix<T: Default> {
    pub width: usize,
    pub height: usize,
    data: Vec<T>,
}

impl<T: Debug + Default> Debug for Matrix<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Matrix({}x{})", self.width, self.height)?;

        for y in 0..self.height {
            for x in 0..self.width {
                match self.get(x, y) {
                    Some(x) => write!(f, "{:?}\t", x)?,
                    None => write!(f, " \t")?,
                };
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl<T: Default> Matrix<T> {
    #[allow(dead_code)]
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: Vec::from_iter((0..width * height).map(|_| T::default())),
        }
    }

    fn get_index<P>(&self, x: P, y: P) -> Option<usize>
    where
        P: TryInto<i32>,
    {
        let position = x.try_into().ok().unwrap() + y.try_into().ok().unwrap() * self.width as i32;

        if position < 0 || position >= self.data.len() as i32 {
            None
        } else {
            Some(position as usize)
        }
    }

    pub fn get<P>(&self, x: P, y: P) -> Option<&T>
    where
        P: TryInto<i32>,
    {
        self.get_index(x, y).map(|index| &self.data[index])
    }

    #[allow(dead_code)]
    pub fn set<P>(&mut self, x: P, y: P, value: T)
    where
        P: TryInto<i32>,
    {
        if let Some(index) = self.get_index(x, y) {
            self.data[index] = value;
        }
    }

    pub fn neighbours<P>(&self, x: P, y: P) -> Vec<&T>
    where
        P: TryInto<i32>,
    {
        let x = x.try_into().ok().unwrap();
        let y = y.try_into().ok().unwrap();
        let offsets = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        offsets
            .iter()
            .filter_map(|(dx, dy)| self.get(x + dx, y + dy))
            .collect()
    }
}

impl<T: Default + FromStr> Matrix<T> {
    #[allow(dead_code)]
    pub fn from(s: &str) -> Option<Matrix<T>> {
        let lines = s.lines().map(|l| l.to_string()).collect::<Vec<String>>();
        let width = match lines.get(0) {
            None => return None,
            Some(s) => s.len(),
        };
        let height = lines.len();

        let data = lines
            .iter()
            .map(|line| {
                line.chars()
                    .map(|ch| ch.to_string().parse::<T>().unwrap_or_default())
                    .collect::<Vec<T>>()
            })
            .flatten()
            .collect::<Vec<T>>();

        Some(Self {
            width,
            height,
            data,
        })
    }

    #[allow(dead_code)]
    pub fn from_separated(s: &str, pat: &str) -> Option<Matrix<T>> {
        let lines = s.lines().map(|l| l.to_string()).collect::<Vec<String>>();
        let width = match lines.get(0) {
            None => return None,
            Some(s) => s.len(),
        };
        let height = lines.len();
        let mut data: Vec<T> = vec![];

        for line in lines {
            data.extend(line.split(pat).map(|s| s.parse::<T>().unwrap_or_default()));
        }

        Some(Self {
            width,
            height,
            data,
        })
    }
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

#[cfg(test)]
mod test {
    use crate::helpers::Matrix;

    #[test]
    fn test_matrix() {
        let matrix: Matrix<i32> = Matrix::new(5, 5);
        println!("{:?}", matrix);
    }
}
