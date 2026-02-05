use std::fmt::Display;

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "({:.1} {:.1})", self.0, self.1)?;
        write!(f, "({:.1} {:.1})", self.2, self.3)
    }
}

impl Matrix {
    fn reverse(&mut self) {
        std::mem::swap(&mut self.1, &mut self.2);
    }
}

#[derive(Debug)]
struct Number {
    val: i32,
}

impl From<i32> for Number {
    fn from(val: i32) -> Self {
        Number { val }
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Number - {}", self.val)
    }
}
fn main() {
    let short_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
    // let long_tuple = (1, 11, 12, 13, 14, 2, 3, 4, 5, 6, 7, 8, 9, 10);

    println!("{:?}", short_tuple);
    // println!("{:?}", long_tuple);

    let mut m = Matrix(3., 2., 1., 0.);
    println!("{:?}", m);
    println!("{}", m);
    m.reverse();
    println!("{:?}", m);
    println!("{}", m);

    let n = Number::from(32);
    println!("{:?} is a wrapper for {:?}", n, n.val);
    println!("{:} is a wrapper for {}", n, n.val);

    let mut count = 0;
    loop {
        println!("{count}");
        count += 1;

        if count == 10 {
            break;
        }
    }
}
