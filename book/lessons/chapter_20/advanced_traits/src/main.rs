use std::fmt;
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human {}

impl Human {
    fn fly(&self) {
        println!("haha");
    }
}

impl Pilot for Human {
    fn fly(&self) {
        println!("heyooo we are going uppppp");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("brrr up up up wooden stick");
    }
}

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 6));
        println!("* {}*", " ".repeat(len + 3));
        println!("*  {output}  *");
        println!("* {}*", " ".repeat(len + 3));
        println!("{}", "*".repeat(len + 6));
    }
}

impl OutlinePrint for Point {}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.x, self.y)
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    let h = Human {};
    h.fly();
    Wizard::fly(&h);
    Pilot::fly(&h);

    let p = Point { x: 1, y: 2 };
    println!("{}", p);
    p.outline_print();
}
