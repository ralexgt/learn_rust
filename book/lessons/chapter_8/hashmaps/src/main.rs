use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    let team_blue = String::from("blue");
    let team_yellow = String::from("yellow");

    scores.insert(team_blue, 10);
    scores.insert(team_yellow, 10);

    let team_blue = String::from("blue");

    let score_blue = scores.get(&team_blue).copied().unwrap_or(0);
    println!("{}", score_blue);

    for (key, value) in &mut scores {
        println!("{key}: {value}");
        *value += 5;
        println!("{key}: {value}");
    }
    println!("{:?}", scores);

    scores.insert(String::from("blue"), 5); // overwrite data without any warning
    println!("{:?}", scores);

    scores.entry(String::from("blue")).or_insert(1); // checks if the key exists, and if not
    // inserts it
    println!("{:?}", scores);

    let text =
        "This is some bunch of random words for a word counter that counts words. This is it.";
    let mut word_counter: HashMap<&str, i32> = HashMap::new();
    for word in text.split_whitespace() {
        // let count = word_counter.get(word).unwrap_or(&0); // this also works
        // word_counter.insert(word, *count + 1); // but rust book does it like this

        let count = word_counter.entry(word).or_insert(0); // this gives us a mutable pointer to
        // the specific memory that we want to update, hence the dereferencing below
        *count += 1;
    }

    println!("{:?}", word_counter);
}
