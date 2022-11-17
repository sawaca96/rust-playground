#[derive(Debug)]
pub struct Matrix<T: Clone> {
    // Fields you need
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cell<T>(pub T);

impl<T: Clone> Matrix<T> {
    /// Data is expected to be passed with a static array -- see below for examples of
    /// construct. What might the elements be? We will only test with two types: String and i32.
    ///
    /// Expected to be passed in rows, from left to right. That is, if we pass as input a list
    /// with elements: 1, 2, 3, 4, the constructed matrix is ​​expected:
    ///
    /// | 1 2 |
    /// | 3 4 |
    ///
    /// Note that we pass as input some slice -- reference type. We do not expect the matrix to
    /// holds a reference, clone your data to have ownership.
    ///
    pub fn new(data: &[T; 4]) -> Matrix<T> {
        todo!()
    }

    /// Returns a vector that contains all 4 elements of the matrix, arranged in rows,
    /// left to right and top to bottom wrapped in `Cell`. That is, if the matrix looks like this:
    ///
    /// | 1 2 |
    /// | 3 4 |
    ///
    /// then `.by_row` to return the elements in order: 1, 2, 3, 4
    ///
    pub fn by_row(&self) -> Vec<Cell<T>> {
        todo!()
    }

    /// Returns a vector that contains all 4 elements of the matrix, arranged by column,
    /// from top to bottom and from left to right, Wrapped in `Cell`. That is, if the matrix looks like this:
    ///
    /// | 1 2 |
    /// | 3 4 |
    ///
    /// then `.by_col` to return the elements in order: 1, 3, 2, 4
    ///
    pub fn by_col(&self) -> Vec<Cell<T>> {
        todo!()
    }
}
