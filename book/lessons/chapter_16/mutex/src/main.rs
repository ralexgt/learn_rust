use std::sync::{Arc, Mutex};
use std::thread;
fn main() {
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num += 1;
        // the lock is dropped automatically at the end of the scope
    }
    println!("m = {:?}", m);
    println!("m = {:?}", m.lock());
    println!("m = {}", m.lock().unwrap());

    let count = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let count = Arc::clone(&count);
        let handle = thread::spawn(move || {
            *count.lock().unwrap() += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *count.lock().unwrap());
}
