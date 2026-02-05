enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let v: Vec<i32> = Vec::new();
    println!("{:?}", v);
    let mut v = vec![1u8, 2, 3];
    let x: i32 = 332;
    v.push(4);
    v.push(5);
    v.push(x as u8); // overflows but doesnt error
    println!("{:?}", v);

    let mut v = vec![String::from("aa"), "bb".to_string(), "cc".to_owned()];
    let f = String::from("dd");
    v.push(f);
    println!("{:?}", v);
    // println!("{:?}", f); => f was moved by v.push(f) so the data is in the v vector memory block

    let v = vec![String::from("1"), String::from("2")];
    println!("{:?}", v);
    let first = &v[0];
    println!("{}", first);
    // let second = v[1]; can't move out of vector because String does not implement copy
    // println!("{}", second);
    let forth = v.get(3);
    match forth {
        Some(x) => println!("{}", x),
        None => println!("There aren't so many values"),
    }

    let mut v = [1, 3, 24];
    for i in &v {
        println!("{}", i);
    }
    for i in &mut v {
        *i += 10;
    }
    println!("{:?}", v);

    let mut row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(3.),
        SpreadsheetCell::Int(6),
        SpreadsheetCell::Int(9),
        SpreadsheetCell::Text("abcd".to_string()),
    ];
    for cell in &mut row {
        match cell {
            SpreadsheetCell::Int(val) => println!("Int {}", val),
            SpreadsheetCell::Float(val) => println!("Float {}", val),
            SpreadsheetCell::Text(val) => {
                *val = format!("modified: {}", val);
                println!("Text {}", val)
            }
        }
    }
    if let SpreadsheetCell::Text(val) = &mut row[4] {
        *val = format!("modified: {}", val);
        println!("{}", val);
    }

    for cell in &mut row {
        match cell {
            SpreadsheetCell::Int(val) => println!("Int {}", val),
            SpreadsheetCell::Float(val) => println!("Float {}", val),
            SpreadsheetCell::Text(val) => println!("Text {}", val),
        }
    }
}
