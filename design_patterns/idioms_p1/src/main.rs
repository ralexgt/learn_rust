use std::path::PathBuf;
use std::time::Duration;

fn three_vowels(word: &str) -> bool {
    let mut vowel_count = 0;
    for c in word.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                vowel_count += 1;
                if vowel_count >= 3 {
                    return true;
                }
            }
            _ => vowel_count = 0,
        }
    }
    false
}

fn say_hello(name: &str) -> String {
    format!("Hello, {}", name)
}

#[derive(Default, Debug, PartialEq)]
struct MyConfiguration {
    output: Option<PathBuf>,
    // Vecs default to empty vector
    search_path: Vec<PathBuf>,
    // Duration defaults to zero time
    timeout: Duration,
    // bool defaults to false
    check: bool,
}

fn main() {
    let ferris = "Ferris".to_string();
    let curious = "Curious".to_string();
    println!("{}: {}", ferris, three_vowels(&ferris));
    println!("{}: {}", curious, three_vowels(&curious));

    println!("Ferris: {}", three_vowels("Ferris"));
    println!("Curious: {}", three_vowels("Curious"));

    let sentence_string =
        "Once upon a time, there was a friendly curious crab named Ferris".to_string();

    sentence_string.split_whitespace().for_each(|word| {
        if three_vowels(word) {
            println!("{}: {}", word, three_vowels(word));
        }
    });

    let greeting = say_hello("Bro");
    println!("{}", greeting);

    let conf = MyConfiguration::default();

    println!("{:?}", conf);
}
