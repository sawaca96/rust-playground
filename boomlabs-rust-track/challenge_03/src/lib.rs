#[derive(Debug)]
pub struct Matrix<T> {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<T>,
}

impl<T: Clone> Matrix<T> {
    /// Constructs a matrix with the number of rows `rows` and the number of columns `cols`. The data should be
    /// at least `rows * cols`, but we won't test with arrays that are too small or too large.
    /// If you want to panic on invalid input, your choice.
    ///
    /// The data will be arranged in a flat array by rows. You are expected to take ownership of them,
    /// so clone them and arrange them as you see fit.
    ///
    pub fn new(rows: usize, cols: usize, data: &[T]) -> Self {
        if rows * cols != data.len() {
            panic!("Invalid input");
        }
        Matrix {
            rows,
            cols,
            data: data.to_vec(),
        }
    }

    pub fn by_row(&self) -> RowIter<T> {
        RowIter {
            matrix: self,
            row: 0,
            col: 0,
        }
    }

    pub fn by_col(&self) -> ColIter<T> {
        ColIter {
            matrix: self,
            row: 0,
            col: 0,
        }
    }
}

pub struct RowIter<'a, T> {
    matrix: &'a Matrix<T>,
    row: usize,
    col: usize,
}

pub struct ColIter<'a, T> {
    matrix: &'a Matrix<T>,
    row: usize,
    col: usize,
}

impl<'a, T> Iterator for RowIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row == self.matrix.rows {
            return None;
        }
        let result = Some(&self.matrix.data[self.row * self.matrix.cols + self.col]);
        self.col += 1;
        if self.col == self.matrix.cols {
            self.col = 0;
            self.row += 1;
        }
        result
    }
}

impl<'a, T> Iterator for ColIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.col == self.matrix.cols {
            return None;
        }
        let result = Some(&self.matrix.data[self.row * self.matrix.cols + self.col]);
        self.row += 1;
        if self.row == self.matrix.rows {
            self.row = 0;
            self.col += 1;
        }
        result
    }
}
