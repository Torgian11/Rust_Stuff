use trpl::Html;
use std::time::Duration;

async fn page_title(url: &str) -> (&str, Option<String>) {
    let text = trpl::get(url).await.text().await;
    let title = Html::parse(&text).select_first("title").map(|title| title.inner_html());
    (url, title)
}

fn basic_async_scraping() {
    let args: Vec<String> = std::env::args().collect();        
    trpl::run(async {
        let title_fut_1 = page_title(&args[1]);
        let title_fut_2 = page_title(&args[2]);

        let (url, maybe_title) = 
            match trpl::race(title_fut_1, title_fut_2).await {
                trpl::Either::Left(left) => left,
                trpl::Either::Right(right) => right,
            };
        
        println!("{url} returned first");
        match maybe_title {
            Some(title) => println!("Its page title is: '{title}'"),
            None => println!("Its title could not be parsed"),
        }
    })
}

fn main() {
    // concurrency
    trpl::run(async {
        // Like JOIN, but just awaiting both async tasks
        // let handle = trpl::spawn_task(async {
        //     for i in 1..10 {
        //         println!("number {i} from the first task!");
        //         trpl::sleep(Duration::from_millis(500)).await;
        //     }
        // });

        // for i in 1..5 {
        //     println!("number {i} from the second task!");
        //     trpl::sleep(Duration::from_millis(500)).await;
        // }   

        // handle.await.unwrap(); 

        // USING trpl::join
        let fut1 = async {
            for i in 1..10 {
                println!("number {i} in first task");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let fut2 = async {
            for i in 1..5 {
                println!("number {i} in second task");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        // Exact same order is displayed every time in this case
        // Each future is checked _fairly_, and checked equally as often, unlike threads
        trpl::join(fut1, fut2).await;
    });

    trpl::run(async {
        // MESSAGE PASSING
        
        // This will NEVER exit
        let (tx, mut rx) = trpl::channel();

        // let vals = vec![
        //     String::from("hi"),
        //     String::from("from"),
        //     String::from("the"),
        //     String::from("future"),
        // ];

        // for val in vals {
        //     tx.send(val).unwrap();
        //     trpl::sleep(Duration::from_millis(500)).await;
        // }

        // while let Some(value) = rx.recv().await {
        //     println!("received: {value}");
        // }

        let tx1 = tx.clone();

        let tx1_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received: {value}");
            }
        };

        let tx_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("again "),
                String::from("with"),
                String::from("candy"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        trpl::join3(tx1_fut, tx_fut, rx_fut).await;
    })
        
}
