#[derive(Debug)]
pub struct Matrix<T> {
    // TODO
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
        todo!()
    }

    pub fn by_row(&self) -> RowIter<T> {
        todo!()
    }

    pub fn by_col(&self) -> ColIter<T> {
        todo!()
    }
}

pub struct RowIter<T> {
    // TODO
}

pub struct ColIter<T> {
    // TODO
}
