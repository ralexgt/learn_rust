// Generic typed struct
struct Point<T> {
    _x: T,
    _y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { _x: x, _y: y }
    }
}

impl Point<f32> {
    fn new2(x: f32, y: f32) -> Self {
        Point { _x: x, _y: y }
    }
}

fn main() {
    // nothing to prevent code duplication if we add other lists to find max for
    let number_list = [33, 22, 58, 46, 24, 55];
    let mut max = &number_list[0];
    for number in &number_list {
        if number > max {
            max = number;
        }
    }
    println!("{}", max);

    // add a function layer *largest* to avoid code duplication for arrays of i32
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {result}");

    // Adding generics to our function so we can use it of slices of different types
    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = generic_largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = generic_largest(&char_list);
    println!("The largest char is {result}");

    let _p1 = Point::new(3, 7);
    let _p2 = Point::new(6.88, 2.35);
    let _p3 = Point::new2(3., 7.);
}

fn largest(list: &[i32]) -> &i32 {
    let mut max = &list[0];
    for number in list {
        if number > max {
            max = number;
        }
    }

    max
}

// PartialOrd is a trait
fn generic_largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut max = &list[0];
    for number in list {
        if number > max {
            max = number;
        }
    }

    max
}
