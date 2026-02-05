fn add(x: &mut i32) {
    *x += 1;
}

fn do_twice(f: fn(&mut i32), arg: &mut i32) {
    f(arg);
    f(arg);
}

fn main() {
    let mut x = 1;
    println!("{}", x);
    add(&mut x);
    println!("{}", x);

    do_twice(add, &mut x);
    println!("{}", x);
}
