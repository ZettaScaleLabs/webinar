use std::io::Write;
use std::time::Duration;
use rand::prelude::*;
use zenoh::prelude::r#async::*;


#[tokio::main]
async fn main() {
    let greetings = std::vec!["Ohayô Gozaimasu", "Konnichiwa", "Konbanwa", "Oyasuminasai"];
    let z = zenoh::open(zenoh::config::default()).res().await.unwrap();

    loop {
        let idx= random::<usize>() % greetings.len();
        let _ = z.put("demo/tokio/greeting", greetings[idx]).res().await;
        tokio::time::sleep(Duration::from_secs(1)).await;
        print!(".");
        let _ = std::io::stdout().flush();
    }
}
