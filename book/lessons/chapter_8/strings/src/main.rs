fn main() {
    let _s = String::new();
    let data = "initial content";
    let s = data.to_string();
    assert_eq!(s, "initial content".to_string());

    let mut s = String::from("foo");
    s.push_str("bar");
    let s2 = "bar";
    s.push_str(s2);
    println!("{}", s2);
    let c = 'l';
    s.push(c);
    println!("{}", c);

    let s1 = String::from("toxic");
    let s2 = String::from("so");
    let s3 = s1 + &s2;
    println!("{}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let tto = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", tto);

    let hello = "Здравствуйте";
    let s = &hello[0..2];
    println!("{s}");

    let hello = "hello";
    let s = &hello[0..1];
    println!("{s}");

    for c in "Зд".chars() {
        println!("{}", c);
    }
    for b in "Зд".bytes() {
        println!("{}", b);
    }
}
