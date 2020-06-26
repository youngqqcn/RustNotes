use std::fmt;

fn reverse(pair : (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;
    (boolean, integer)
}


#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {

    fn fmt( &self,  f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {})\n({} {})", self.0, self.1, self.2, self.3)
    }

}


fn transpose( matrix : Matrix ) -> Matrix {
    let mtx = Matrix(matrix.0, matrix.3, matrix.2, matrix.1 );
    mtx
    // Matrix(matrix.0, matrix.2, matrix.1, matrix.3 )
}


fn main() {

    let pair = (1, true);
    println!("reversed pair is : {:?}", reverse( pair ));

    println!("{}", Matrix(1.1, 1.2, 2.1, 2.2));
    println!("Transpose:\n{}", transpose( Matrix(1.1, 1.2, 2.1, 2.2)) );

}