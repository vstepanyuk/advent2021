use std::fmt::{Debug, Formatter};
use std::str::FromStr;

pub struct Matrix<T> {
    pub width: usize,
    pub height: usize,
    pub(crate) data: Vec<T>,
}

pub struct MatrixIter<'a, T> {
    data: &'a [T],
    index: usize,
    width: usize,
}

impl<T> Matrix<T> {
    pub fn iter(&self) -> impl Iterator<Item = (&T, (usize, usize))> {
        self.data.iter().enumerate().map(|(index, value)| {
            let x = index % self.width;
            let y = index / self.width;

            (value, (x, y))
        })
    }
}

impl<'a, T: 'a> Matrix<T> {
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&mut T, (usize, usize))> {
        self.data.iter_mut().enumerate().map(|(index, value)| {
            let x = index % self.width;
            let y = index / self.width;

            (value, (x, y))
        })
    }
}

impl<T: Debug + Default> Debug for Matrix<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Matrix({}x{})", self.width, self.height)?;

        for y in 0..self.height {
            for x in 0..self.width {
                match self.get(x, y) {
                    Some(x) => write!(f, "{:?}", x)?,
                    None => write!(f, " ")?,
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
        let x = x.try_into().ok().unwrap();
        let y = y.try_into().ok().unwrap();
        if x < 0 || x as usize >= self.width || y < 0 || y as usize >= self.height {
            return None;
        }
        let position = x + y * self.width as i32;
        Some(position as usize)
    }

    pub fn get<P>(&self, x: P, y: P) -> Option<&T>
    where
        P: TryInto<i32>,
    {
        self.get_index(x, y).and_then(|index| self.data.get(index))
    }

    pub fn get_mut<P>(&mut self, x: P, y: P) -> Option<&mut T>
    where
        P: TryInto<i32>,
    {
        self.get_index(x, y)
            .and_then(|index| self.data.get_mut(index))
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
    //
    // pub fn neighbours_pos<P>(&self, x: P, y: P) -> Vec<(usize, usize)>
    // where
    //     P: TryInto<i32>,
    // {
    //     let x = x.try_into().ok().unwrap();
    //     let y = y.try_into().ok().unwrap();
    //     let offsets = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    //
    //     offsets
    //         .iter()
    //         .filter(|(dx, dy)| self.get_index(x + dx, y + dy).is_some())
    //         .map(|(dx, dy)| ((x + dx) as usize, (y + dy) as usize))
    //         .collect()
    // }
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

#[cfg(test)]
mod tests {
    use crate::matrix::Matrix;

    #[test]
    fn matrix_iter() {
        let matrix = Matrix::<u8>::from("123\n456").unwrap();
        assert_eq!(matrix.width, 3);
        assert_eq!(matrix.height, 2);

        let result = matrix
            .iter()
            .take(3)
            .map(|(&v, (x, y))| (v, (x, y)))
            .collect::<Vec<(u8, (usize, usize))>>();

        assert_eq!(
            [
                (1u8, (0usize, 0usize)),
                (2u8, (1usize, 0usize)),
                (3u8, (2usize, 0usize))
            ]
            .to_vec(),
            result
        );
    }
}
