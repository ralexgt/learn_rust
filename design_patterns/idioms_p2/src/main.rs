fn main() {
    let turing = Some("Turing");
    let mut logicians = vec!["Curry", "Kleene", "Markov"];

    logicians.extend(turing);
    // INFO: Equiv to:
    if let Some(turing_inner) = turing {
        logicians.push(turing_inner);
    }

    for logician in logicians.iter().chain(turing.iter()) {
        println!("{} is a logician", logician);
    }
}
