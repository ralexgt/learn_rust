use std::time::Duration;

fn main() {
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();
        let tx1 = tx.clone();

        // move makes it so the future takes ownership over the transmitter. It is mandatory
        // because we need to drop tx once the block finished, so the WHILE in the next block
        // doesn't wait forever
        let tx_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        let tx1_fut = async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
                String::from("you"),
                String::from("you"),
                String::from("you"),
                String::from("you"),
                String::from("you"),
                String::from("you"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        trpl::join!(tx_fut, tx1_fut, rx_fut);

        // let one_ms = Duration::from_millis(1);
        let a = async {
            println!("'a' started.");
            slow("a", 30);
            // trpl::sleep(one_ms).await;
            trpl::yield_now().await;
            slow("a", 10);
            // trpl::sleep(one_ms).await;
            trpl::yield_now().await;
            slow("a", 20);
            // trpl::sleep(Duration::from_millis(50)).await;
            trpl::yield_now().await;
            println!("'a' finished.");
        };

        let b = async {
            println!("'b' started.");
            slow("b", 75);
            // trpl::sleep(one_ms).await;
            trpl::yield_now().await;
            slow("b", 10);
            // trpl::sleep(one_ms).await;
            trpl::yield_now().await;
            slow("b", 15);
            // trpl::sleep(one_ms).await;
            trpl::yield_now().await;
            slow("b", 350);
            // trpl::sleep(Duration::from_millis(50)).await;
            trpl::yield_now().await;
            println!("'b' finished.");
        };

        trpl::select(a, b).await;
    });

    println!("Done first part");

    trpl::block_on(async {
        // Closule / ano func that returns a Future::Output<&str>
        let slow2 = async {
            trpl::sleep(Duration::from_secs(5)).await;
            "Finally finished"
        };

        match timeout(slow2, Duration::from_secs(2)).await {
            Ok(message) => println!("Succedeed with msg: {}", message),
            Err(duration) => println!("Failed after {} seconds", duration.as_secs()),
        };
    });
}

async fn timeout<F: Future>(future_to_try: F, max_time: Duration) -> Result<F::Output, Duration> {
    match trpl::select(future_to_try, trpl::sleep(max_time)).await {
        trpl::Either::Left(output) => Ok(output),
        trpl::Either::Right(_) => Err(max_time),
    }
}

fn slow(name: &str, ms: u64) {
    std::thread::sleep(Duration::from_millis(ms));
    println!("'{}' ran for {}", name, ms);
}
