fn main() {
    let cat = (String::from("Furry McFurson"), 3.5);

    // TODO: Destructure the `cat` tuple in one statement so that the println works.
    // let /* your pattern here */ = cat;
    println!("{:?}", cat);
    let (name, age) = cat;

    println!("{name} is {age} years old");
}
