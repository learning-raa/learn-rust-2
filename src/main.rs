
fn main() {
    let a_matrix = Matrix ( 1., 0.5, -0.5, 2. );
    println!( "dbg\n{:?}\n", a_matrix );
    println!( "show\n{}\n", a_matrix );
    println!( "show\n{}\n", a_matrix.transpose() );
}

#[derive(Debug)]
struct Matrix(f32,f32,f32,f32);

impl core::fmt::Display for Matrix {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(\t{:+.2}\t{:+.2}\t)\n(\t{:+.2}\t{:+.2}\t)", self.0, self.1, self.2, self.3 )
    }
}
impl Matrix {
    fn transpose(&self) -> Self {
        Self( self.0, self.2, self.1, self.3 )
    }
}
