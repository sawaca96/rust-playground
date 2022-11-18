use std::ops::{Add, Mul};

#[derive(Debug)]
pub struct Matrix<T: Clone> {
    pub data: [T; 4],
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cell<T>(pub T);

impl Add<Cell<String>> for Cell<i32> {
    type Output = Cell<String>;

    fn add(self, rhs: Cell<String>) -> Self::Output {
        if self.0 >= 0 {
            Cell(format!("{} {}", self.0, rhs.0))
        } else {
            Cell(format!(
                "{} {}",
                rhs.0.chars().rev().collect::<String>(),
                self.0 * -1
            ))
        }
    }
}

impl Mul<Cell<String>> for Cell<i32> {
    type Output = Cell<String>;

    fn mul(self, rhs: Cell<String>) -> Self::Output {
        if self.0 >= 0 {
            Cell(format!("{}", rhs.0.repeat(self.0.try_into().unwrap())))
        } else {
            Cell(format!(
                "{}",
                rhs.0
                    .chars()
                    .rev()
                    .collect::<String>()
                    .repeat((self.0 * -1).try_into().unwrap())
            ))
        }
    }
}

impl<T: Clone> Matrix<T> {
    pub fn new(data: &[T; 4]) -> Matrix<T> {
        Matrix {
            data: [
                data[0].clone(),
                data[1].clone(),
                data[2].clone(),
                data[3].clone(),
            ],
        }
    }

    pub fn by_row(&self) -> Vec<Cell<T>> {
        let mut result = Vec::new();
        for i in 0..self.data.len() {
            result.push(Cell(self.data[i].clone()));
        }
        result
    }

    pub fn by_col(&self) -> Vec<Cell<T>> {
        let mut result = Vec::new();
        for i in (0..self.data.len()).step_by(2) {
            result.push(Cell(self.data[i].clone()));
        }
        for i in (1..self.data.len()).step_by(2) {
            result.push(Cell(self.data[i].clone()));
        }
        result
    }
}

impl Add<Matrix<String>> for Matrix<i32> {
    type Output = Matrix<String>;

    fn add(self, rhs: Matrix<String>) -> Self::Output {
        let mut result = Vec::new();
        for i in 0..self.data.len() {
            result.push((Cell(self.data[i]) + Cell(rhs.data[i].clone())).0);
        }
        Matrix::new(&result.try_into().unwrap())
    }
}

impl Mul<Matrix<String>> for Matrix<i32> {
    type Output = String;

    fn mul(self, rhs: Matrix<String>) -> Self::Output {
        let mut result = Vec::new();
        for i in 0..self.data.len() {
            result.push((self.by_row()[i].clone() * rhs.by_col()[i].clone()).0);
        }
        result.join(" ")
    }
}
