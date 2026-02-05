use std::env::args;
use trpl::{Either, Html};

async fn get_page_title(url: &str) -> (&str, Option<String>) {
    // these 2 lines can actually be chained together, thanks to await being a postfix keyword
    // let response = trpl::get(url).await;
    // let response = response.text().await;
    let response = trpl::get(url).await.text().await;
    let title = Html::parse(&response)
        .select_first("title")
        // here is Option::map, not iterator. Returns the item inside Option or does nothing if
        // None
        .map(|title| title.inner_html());
    (url, title)
}

fn main() {
    let args: Vec<String> = args().collect();

    trpl::block_on(async {
        let title1 = get_page_title(&args[1]);
        let title2 = get_page_title(&args[2]);

        let (url, maybe_title) = match trpl::select(title1, title2).await {
            Either::Left(left) => left,
            Either::Right(right) => right,
        };

        println!("{} returned first", url);
        match maybe_title {
            Some(title) => println!("Its page title was {}", title),
            None => println!("It had no page title"),
        }
    });
}
