use std::{default, fmt::{Debug, Display}, ops::{Add, Div, Mul, Sub}};

pub struct Vector<T> {
    content: Box<[T]>
}

pub struct Matrix<T> {
    rows: Box<[Vector<T>]>
}

impl <T: Copy + Add<T, Output = T>> Add<T> for Vector<T> {
    type Output = Vector<T>;

    fn add(self, rhs: T) -> Self::Output {
        Vector {
            content: self.content.iter().map(move |x| *x + rhs).collect()
        }
    }
}

impl <T: Copy + Sub<T, Output = T>> Sub<T> for Vector<T> {
    type Output = Vector<T>;

    fn sub(self, rhs: T) -> Self::Output {
        Vector {
            content: self.content.iter().map(move |x| *x - rhs).collect()
        }
    }
}

impl <T: Copy + Mul<T, Output = T>> Mul<T> for Vector<T> {
    type Output = Vector<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vector {
            content: self.content.iter().map(move |x| *x * rhs).collect()
        }
    }
}

impl <T: Copy + Div<T, Output = T>> Div<T> for Vector<T> {
    type Output = Vector<T>;

    fn div(self, rhs: T) -> Self::Output {
        Vector {
            content: self.content.iter().map(move |x| *x / rhs).collect()
        }
    }
}

impl <T: Copy + Add<T, Output = T>> Add<Vector<T>> for Vector<T> {
    type Output = Vector<T>;

    fn add(self, rhs: Vector<T>) -> Self::Output {
        Vector {
            content: self.content.iter().zip(rhs.content).map(move |(&x, y)| x + y).collect()
        }
    }
}

impl <T: Copy + Sub<T, Output = T>> Sub<Vector<T>> for Vector<T> {
    type Output = Vector<T>;

    fn sub(self, rhs: Vector<T>) -> Self::Output {
        Vector {
            content: self.content.iter().zip(rhs.content).map(move |(&x, y)| x - y).collect()
        }
    }
}

impl <T: Display> Display for Vector<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;

        for v in self.content.iter() {
            write!(f, "{} ", v)?
        }

        write!(f, ")")
    }
}

impl <T: Debug> Debug for Vector<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;

        for v in self.content.iter() {
            write!(f, "{:?} ", v)?
        }

        write!(f, ")")
    }
}



impl <T: Clone> Clone for Vector<T> {
    fn clone(&self) -> Self {
        Vector {
            content: self.content.clone()
        }
    }
}

impl <T: Copy> Vector<T> {
    pub fn get(&self, index: usize) -> T {
        self.content[index]
    }

    pub fn new(size: usize, value: T) -> Vector<T> {
        Vector {
            content: vec![value; size].into_boxed_slice()
        }
    }
}


impl <T: Clone> Clone for Matrix<T> {
    fn clone(&self) -> Self {
        Matrix {
            rows: self.rows.clone()
        }
    }
}


impl <T: Display> Display for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.rows.iter() {
            write!(f, "{}\n", row)?
        }

        Result::Ok(())
    }
}

impl <T: Debug> Debug for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.rows.iter() {
            write!(f, "{:?}\n", row)?
        }

        Result::Ok(())
    }
}



impl <T: Copy> Matrix<T> {
    pub fn new(rows: usize, columns: usize, value: T) -> Matrix<T> {
        let row_content: Vec<Vector<T>> = (0..rows).map(|_| Vector::new(columns, value)).collect();

        Matrix {
            rows: row_content.into_boxed_slice()
        }
    }
}

impl <T: Copy> Matrix<T> {
    pub fn get(&self, row: usize, column: usize) -> T {
        self.rows[row].content[column]
    }

    pub fn set(&mut self, row: usize, column: usize, value: T) {
        self.rows[row].content[column] = value;
    }
    
    pub fn get_column(&self, column: usize) -> Vector<T> {
        Vector {
            content: self.rows.iter().map(|x| x.content[column]).collect()
        }
    }

    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    pub fn column_count(&self) -> usize {
        self.rows[0].content.len()
    }
}


impl Matrix<f64> {
    pub fn solve(&self) -> Option<Vector<f64>> {
        let rows = self.row_count();
        let cols = self.column_count();
        let mut m = self.clone();
        let mut result_content: Vec<f64> = vec![0.0; rows];
        let mut row = 0;

        for col in 0..(cols - 1) {
            let mut column_data: Vec<(usize, f64)> = self.get_column(col).content.iter().enumerate().skip(row).map(|(i, &v)| (i, v)).collect();
            column_data.sort_by(|a, b| a.1.abs().partial_cmp(&b.1.abs()).unwrap_or(std::cmp::Ordering::Equal));
            let max_index = column_data.last().unwrap().0;

            if (m.rows[max_index].content[col]).abs() < 0.000001 {
                return None
            }

            let temp = m.rows[max_index].clone();
            m.rows[max_index] = m.rows[row].clone();
            m.rows[row] = temp;

            for i in (row + 1)..rows {
                let fraction = m.rows[i].content[col] / m.rows[row].content[col];

                for j in (col + 1)..cols {
                    m.rows[i].content[j] = m.rows[i].content[j] -  m.rows[row].content[j] * fraction;
                }

                m.rows[i].content[col] = 0.0;
            }

            row += 1;
        }

        for i in (0..=(rows - 1)).rev() {
            let mut sum = 0.0;
            for j in (i..=(rows - 1)).rev() {
                sum = sum + result_content[j] * m.rows[i].content[j];
            }
            result_content[i] = (m.rows[i].content[cols - 1] - sum) / m.rows[i].content[i];
        }

        Some(Vector {
            content: result_content.into_boxed_slice()
        })
    }
}
