use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // we could annotate the types, but we do not have to usually. Rust will infer them from how we
    // use the channel later on.
    // let (tx, rx): (mpsc::Sender<i32>, mpsc::Receiver<i32>) = mpsc::channel();
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone(); // mpsc (multiple producers single consumer) means we can create multiple
    // transmitters
    // we move the transmitter into this thread because we use it and gets captured
    thread::spawn(move || -> mpsc::Sender<String> {
        let val = String::from("hi from thread1");
        tx.send(val).unwrap();
        // sending through a channel moves the value
        // println!("{}", val);
        tx
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    thread::spawn(move || {
        let vals = vec![
            "*".to_string(),
            "**".to_string(),
            "***".to_string(),
            "****".to_string(),
            "*****".to_string(),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

    // tx.clone() creates one more transmitter
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
}
