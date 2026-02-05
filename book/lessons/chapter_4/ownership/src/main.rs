fn main() {
    let mut s = String::from("Some random string");
    s.push_str(" with some added context");
    println!("{}", s);

    let s2 = s;
    // println!("{}", s);
    println!("{}", s2);

    let x = 3;
    let x2 = 6;
    println!("{}, {}", x, x2);

    let s = String::from("test");
    takes_ownership(s);
    // println!("{}", s); s was moved to the function and dropped when the function finished. It is
    // not returned to the original owner

    let x = 6;
    makes_copy(x);
    println!("{}", x);

    let new_s = gives_ownership();
    println!("{}", new_s);

    let mut new_s2 = takes_and_gives_back(new_s);
    println!("{}", new_s2);
    // println!("{}", new_s); // borrow of moved value new_s. new_s moved into the function and
    // then into new_s2

    let l = calculate_length(&new_s2);
    println!("{} is {} bytes long.", new_s2, l);

    try_changing(&mut new_s2);

    let r1 = &mut new_s2;
    // let r2 = &mut new_s2; // cannot borrow as mutable more than once at a time
    // println!("{}, {}", r1, r2);
    println!("{}", r1);

    let mut s = String::from("hello world");
    let _word = first_word(&s); // word will get the value 5
    s.clear(); // this empties the String, making it equal to ""
    // word still has the value 5 here, but s no longer has any content that we
    // could meaningfully use with the value 5, so word is now totally invalid!

    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} {}", hello, world);

    let mut s = String::from("hello world");
    let word = first_word_good(&s);
    // s.clear();
    println!("{}", word);

    let x = &s[..];
}

fn takes_ownership(s: String) {
    println!("{}", s);
}

fn makes_copy(n: i32) {
    println!("{}", n);
}

fn gives_ownership() -> String {
    String::from("returned from func")
}

fn takes_and_gives_back(s: String) -> String {
    println!("{}", s);
    s
}

fn calculate_length(s: &str) -> usize {
    s.len()
}

fn try_changing(s: &mut String) {
    s.push_str("tralalala");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_good(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
