use std::fmt::{Debug, Formatter};
use std::str::FromStr;

pub struct Matrix<T> {
    pub width: usize,
    pub height: usize,
    pub(crate) data: Vec<T>,
}

#[allow(dead_code)]
pub const MATRIX_NEIGHBOURS_4: [(i32, i32); 4] = [(1, 0), (0, 1), (0, -1), (-1, 0)];

#[allow(dead_code)]
pub const MATRIX_NEIGHBOURS_8: [(i32, i32); 8] = [
    (1, 0),
    (0, 1),
    (0, -1),
    (-1, 0),
    (-1, -1),
    (1, 1),
    (-1, 1),
    (1, -1),
];

impl<T> Matrix<T> {
    pub fn size(&self) -> usize {
        self.width * self.height
    }

    #[allow(dead_code)]
    pub fn iter(&self) -> impl Iterator<Item = (&T, (usize, usize))> {
        self.data.iter().enumerate().map(|(index, value)| {
            let x = index % self.width;
            let y = index / self.width;

            (value, (x, y))
        })
    }

    #[allow(dead_code)]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&mut T, (usize, usize))> {
        self.data.iter_mut().enumerate().map(|(index, value)| {
            let x = index % self.width;
            let y = index / self.width;

            (value, (x, y))
        })
    }

    #[allow(dead_code)]
    pub fn iter_with_self(&self) -> impl Iterator<Item = (&T, (usize, usize), &Matrix<T>)> {
        self.data.iter().enumerate().map(move |(index, value)| {
            let x = index % self.width;
            let y = index / self.width;

            (value, (x, y), self)
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

impl<T> Matrix<T> {
    #[allow(dead_code)]
    pub fn new(width: usize, height: usize) -> Matrix<T>
    where
        T: Default,
    {
        let mut data = Vec::new();
        data.resize_with(width * height, T::default);

        Self {
            width,
            height,
            data,
        }
    }

    #[allow(dead_code)]
    pub fn from_iter<'a>(iter: impl Iterator<Item = &'a T>, width: usize) -> Matrix<T>
    where
        T: 'a + Default + Copy,
    {
        let mut data: Vec<T> = iter.copied().collect();
        let mut height = data.len() / width;
        height += (width * height < data.len()).then(|| 1).unwrap_or(0);

        data.resize_with(width * height, T::default);

        Self {
            width,
            height,
            data,
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

    pub fn neighbours4<P>(&self, x: P, y: P) -> Vec<&T>
    where
        P: TryInto<i32>,
    {
        let x = x.try_into().ok().unwrap();
        let y = y.try_into().ok().unwrap();

        MATRIX_NEIGHBOURS_4
            .iter()
            .filter_map(|(dx, dy)| self.get(x + dx, y + dy))
            .collect()
    }

    #[allow(dead_code)]
    pub fn neighbours8<P>(&self, x: P, y: P) -> Vec<&T>
    where
        P: TryInto<i32>,
    {
        let x = x.try_into().ok().unwrap();
        let y = y.try_into().ok().unwrap();

        MATRIX_NEIGHBOURS_8
            .iter()
            .filter_map(|(dx, dy)| self.get(x + dx, y + dy))
            .collect()
    }

    #[allow(dead_code)]
    pub fn render_to_string<F>(&self, renderer: F) -> String
    where
        F: Fn(Option<&T>) -> String,
    {
        let rows = (0..self.height)
            .map(|y| (0..self.width).map(|x| renderer(self.get(x, y))).collect())
            .collect::<Vec<Vec<_>>>();

        rows.iter()
            .map(|row| row.join(""))
            .collect::<Vec<_>>()
            .join("\n")
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
            .flat_map(|line| {
                line.chars()
                    .map(|ch| ch.to_string().parse::<T>().unwrap_or_default())
                    .collect::<Vec<T>>()
            })
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

    #[test]
    fn matrix_from_iter() {
        let v: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7];
        let matrix = Matrix::<u8>::from_iter(v.iter(), 3);

        let v: Vec<_> = matrix.iter().map(|(&v, _)| v).collect();
        assert_eq!(v, vec![1, 2, 3, 4, 5, 6, 7, 0, 0]);
    }
}
