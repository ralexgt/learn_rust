use std::ops::Deref;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

#[derive(Debug)]
enum ListWithRc {
    Cons(i32, Rc<ListWithRc>),
    Nil,
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomPointer with data `{}`", self.data);
    }
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("{}", name);
}

use crate::List::{Cons, Nil};
use crate::ListWithRc::{Cons as C, Nil as N};

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    let l = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:#?}", l);

    let x = 5;
    let y = &x;
    assert_eq!(x, 5);
    assert_eq!(*y, 5);
    assert_eq!(y, &5);

    let mut x = 3;
    // y is a pointer to a copy of x, not to x itself
    let y = Box::new(x);
    assert_eq!(x, 3);
    assert_eq!(*y, 3);
    x = 6;
    println!("{} | {}", x, y);

    let x = 9;
    let y = MyBox::new(x);
    assert_eq!(9, x);
    assert_eq!(9, *y);

    let name = MyBox::new(String::from("Holala"));
    hello(&name);

    let _c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created");

    let mut a = Rc::new(C(5, Rc::new(C(10, Rc::new(N)))));
    let _b = C(7, Rc::clone(&a));
    let _c = C(9, Rc::clone(&a));
    let _d = C(12, Rc::clone(&mut a));
    let _e = C(9, Rc::clone(&a));
    println!("{:?}", a);
    println!("{:?}", _b);
    println!("{:?}", _d);
    println!("{:?}", _e);
}
