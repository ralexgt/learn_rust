use std::time::Duration;

fn main() {
    trpl::block_on(async {
        let handle = trpl::spawn_task(async {
            for i in 1..10 {
                println!("hi number {} from the first task", i);
                trpl::sleep(Duration::from_millis(200)).await;
            }
        })
        .await;

        // handle.await.unwrap();

        for i in 1..5 {
            println!("hi number {} from the second task", i);
            trpl::sleep(Duration::from_millis(200)).await;
        }

        handle.unwrap();

        let fut1 = async {
            for i in 1..10 {
                println!("hi number {} from async 1", i);
                trpl::sleep(Duration::from_millis(200)).await;
            }
        };
        let fut2 = async {
            for i in 1..5 {
                println!("hi number {} from async 2", i);
                trpl::sleep(Duration::from_millis(200)).await;
            }
        };

        trpl::join(fut1, fut2).await;
    });

    for i in 1..5 {
        println!("hi number {} from the main", i);
        std::thread::sleep(Duration::from_millis(200));
    }
}
