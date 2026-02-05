struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn _level(&self) -> i32 {
        3
    }

    fn _announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}
fn main() {
    let s1 = String::from("abc");
    let s2 = "xyz";

    println!(
        "The longest string of the two is: {}",
        longest(s1.as_str(), s2)
    );

    {
        let s3 = "de";
        let result = longest(s1.as_str(), s3);
        println!("{}", result);
    }

    // let result;
    // {
    // let s3 = String::from("de33");
    // result = longest(s1.as_str(), s3.as_str());
    // }
    // println!("{}", result); // s3 doesn't live enough

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{}", i.part)
}

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}
