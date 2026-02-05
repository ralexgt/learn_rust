use std::thread;
use std::time::Duration;

fn main() {
    let handle_thread_1 = thread::spawn(|| {
        for i in 1..5 {
            println!("first thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // this blocks the program here and waits for the thread to end, just after that spawning the
    // second thread
    // handle_thread_1.join().unwrap();
    let handle_thread_2 = thread::spawn(|| {
        for i in 1..10 {
            println!("second thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    // main ends here, so the spawned threads end as well because the program exits.
    // We need to add something to make the main thread wait for the others to finish

    // the two threads continue alternating, but the main thread waits for them to finish
    handle_thread_1.join().unwrap();
    handle_thread_2.join().unwrap();

    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("{:?}", v);
    });

    // println!("{:?}", v); v was moved in the thread above
    handle.join().unwrap();
}
