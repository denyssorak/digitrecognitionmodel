#[derive(Debug)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<f64>,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Matrix {
        Matrix {
            rows,
            cols,
            data: vec![0.0; rows * cols],
        }
    }

    pub fn get(&self, r: usize, c: usize) -> f64 {
        self.data[r * self.cols + c]
    }

    pub fn set(&mut self, r: usize, c: usize, val: f64) {
        self.data[r * self.cols + c] = val;
    }

    pub fn add(&self, other: &Matrix) -> Matrix {
        if self.rows != other.rows || self.cols != other.cols {
            panic!("{:?}", self)
        }

        let mut result = Matrix::new(self.rows, self.cols);
        for i in 0..(result.rows * result.cols) {
            result.data[i] = self.data[i] + other.data[i];
        }

        result
    }

    pub fn sub(&self, other: &Matrix) -> Matrix {
        if self.rows != other.rows || self.cols != other.cols {
            panic!("{:?}", self)
        }

        let mut result = Matrix::new(self.rows, self.cols);
        for i in 0..(result.rows * result.cols) {
            result.data[i] = self.data[i] - other.data[i];
        }

        result
    }

    pub fn mul(&self, other: &Matrix) -> Matrix {
        if self.rows != other.rows || self.cols != other.cols {
            panic!("{:?}", self)
        }

        let mut result = Matrix::new(self.rows, self.cols);
        for i in 0..(result.rows * result.cols) {
            result.data[i] = self.data[i] * other.data[i];
        }

        result
    }
}
