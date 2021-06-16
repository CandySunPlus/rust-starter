use hwd::rectangle;
use std::{fmt, mem};
use hwd::linked_list;

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Matrix(a, b, c, d) = self;
        write!(f, "({}, {})\n({}, {})", a, b, c, d)
    }
}

fn transpose(matrix: Matrix) -> Matrix {
    let Matrix(a, b, c, d) = matrix;
    return Matrix(a, c, b, d);
}

fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
    println!("Matrix:\n{}", matrix);
    println!("Transpose:\n{}", transpose(matrix));

    let xs = [1, 2, 3, 4, 5];
    let ys = [0; 500];

    println!("array occupies {} bytes", mem::size_of_val(&xs));
    analyze_slice(&xs);
    analyze_slice(&ys[1..4]);

    let p1 = rectangle::Point::new(0.0, 0.0);
    let sq = rectangle::square(p1, 10.0);
    println!("seuare {:#?} area is {}", sq, sq.rect_area());

    let mut link = linked_list::List::new();
    link = link.prepend(20);

    println!("linked list {} length: {}", link.stringify(), link.len());
}
