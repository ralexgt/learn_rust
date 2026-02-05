use std::fmt::Display;
/// `Person` represents a human being
pub struct Person {
    /// Every `Person` needs a name
    name: String,
}

impl Person {
    /// Creates a person with a given name
    ///
    /// # Example
    ///
    /// ```
    /// // You can have rust code between the `
    /// // Passing --test to rustdoc will run your rust code, so you can use it as tests
    /// use crate::Person;
    /// let person = Person::new("name");
    /// assert_eq!("name".to_string(), person.name);
    /// ```
    fn new(name: &str) -> Person {
        Person {
            name: name.to_string(),
        }
    }

    /// Gives a friendly hello to the Person
    fn hello(&self) {
        println!("Hi, {}!", self.name);
    }
}

impl Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

struct List(Vec<i32>);

impl Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;

        for (i, value) in self.0.iter().enumerate() {
            if i != self.0.len() && i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{i}: {value}")?;
        }

        write!(f, "]")
    }
}
fn main() {
    println!("Hello, world!");
    println!("I am a rustacian");

    let x = 5 /* * 3 */ + 9;
    println!("x is: {}", x);

    let p = Person::new("Mocachino");
    println!("{}", p);
    p.hello();

    println!("Base 10:               {}", 69420);
    println!("Base 2 (binary):       {:b}", 69420);
    println!("Base 8 (octal):        {:o}", 69420);
    println!("Base 16 (hexadecimal): {:x}", 69420);
    // output "    1". (Four white spaces and a "1", for a total width of 5.)
    println!("{number:>5}", number = 1);
    // You can pad numbers with extra zeroes,
    println!("{number:0>5}", number = 1); // 00001
    // and left-adjust by flipping the sign. This will output "10000".
    println!("{number:0<5}", number = 1); // 10000
    // You can use named arguments in the format specifier by appending a `$`.
    println!("{number:0>width$}", number = 1, width = 5);

    const PI: f64 = std::f64::consts::PI;
    println!("{:.3}", PI);

    let l = List(vec![3, 78, 2, 1, 15]);
    println!("{}", l);
}
