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

    pub fn trans(&self) -> Matrix {
        let mut result = Matrix::new(self.cols, self.rows);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.set(j, i, self.get(i, j));
            }
        }

        result
    }

    pub fn matmul(&self, other: &Matrix) -> Matrix {
        if self.cols != other.rows {
            panic!("{:?}", self)
        }

        let mut result = Matrix::new(self.rows, other.cols);

        let mut sum = 0.0;

        for i in 0..other.cols {
            for j in 0..self.rows {
                for k in 0..self.cols {
                    sum += self.get(j, k) * other.get(k, i);
                }
                result.set(j, i, sum);
                sum = 0.0;
            }
        }

        result
    }

    pub fn apply(&self, f: fn(f64) -> f64) -> Matrix {
        let mut result = Matrix::new(self.rows, self.cols);

        for i in 0..(self.rows * self.cols) {
            result.data[i] = f(self.data[i]);
        }

        result
    }
}
