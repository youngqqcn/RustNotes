use std::ops::{
    Add,
    Mul, 
};
use std::fmt;
// use std::fmt::{Display, Formatter};


#[derive(Debug)]
struct Complex {
    real: f64,
    image: f64,
}

impl Add for Complex {
    type Output = Complex;
    fn add(self, rhs: Complex) -> Complex{
        Complex {
            real: self.real + rhs.real,
            image: self.image + rhs.image,
        }
    }
}



#[derive(Debug)]
struct Matrix(Vec<Vec<i32>>);

impl Add for Matrix {
    type Output = Matrix;

    fn add(self, rhs: Matrix) -> Matrix {
        assert_eq!(self.0.len(), rhs.0.len());

        let mut new_matrx = Vec::new();
        for row in 0..self.0.len() {
            let mut new_row = Vec::new();
            for col in 0..self.0[row].len() {
                new_row.push( self.0[row][col] + rhs.0[row][col] );
            }
            new_matrx.push(new_row);
        }
        Matrix(new_matrx)
    }

}


impl Mul for Matrix {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        assert_eq!(self.0.len(), rhs.0[0].len()); 
        assert_eq!(self.0[0].len(), rhs.0.len());

        let a = &self.0;
        let b = &rhs.0;

        let mut new_matrx = Vec::new();
        for row in 0..a.len() {
            let mut tmp_row = Vec::new();
            for col in 0..b[0].len() {
                tmp_row.push(|i: usize, j: usize| -> i32 {
                    let mut sum = 0;
                    for idx in 0..j{
                        sum += a[i][idx] * b[idx][i];
                    }
                    sum
                }(row , col ));
            }
            new_matrx.push(tmp_row);
        }

        Matrix(new_matrx) 
    }
}



impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for row in 0..self.0.len() {
            s.push_str("| ");
            for col in 0..self.0[row].len() {
                s.push_str( &self.0[row][col].to_string()[..] );
                s.push_str(" ")
            }
            s.push_str("|\n");
        }
        s.push_str("\n");
        write!(f, "\n{}", s)
    }
}



fn main() {

    let c1 = Complex { real:5.1, image:-1.2 };
    let c2 = Complex { real:0.123 , image: 9.19};

    let c3 = c1 + c2;
    println!("c3 is {:?}", c3);



    let mtx1 = Matrix(vec![
        vec![1, 2, 3], 
        vec![1, 0, 2],
        vec![1, 0, 2],
    ]);

    let mtx2 = Matrix(vec![
        vec![0, 2, 3], 
        vec![1, 0, 2],
        vec![1, 1, 2],
    ]);

    // let mtx3 = mtx1 + mtx2;
    let mtx3 = mtx1 * mtx2;
    println!("mtx3 is {}", mtx3);


    let mtxa = Matrix(vec![
        vec![1, 3], 
        vec![5, 2],
        vec![1, 2],
    ]);

    let mtxb = Matrix(vec![
        vec![5, 2, 3], 
        vec![4, 1, 2],
        // vec![1, 1, 2],
    ]);

    println!("mtxa * mtxb = {}", mtxa * mtxb);
    // println!("mtxb * mtxa = {}", mtxb * mtxa);

}