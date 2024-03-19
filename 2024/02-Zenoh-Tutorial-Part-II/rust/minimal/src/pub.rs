mod sub;

use std::time::Duration;
use zenoh::prelude::r#async::*;
use zenoh::prelude::Config;

#[async_std::main]
async fn main() {

    let arg: std::vec::Vec<String> = std::env::args().collect();
    let c = if arg.len() < 2 {
        zenoh::config::default()
    } else {
        Config::from_file(&arg[1]).unwrap()
    };

    let z = zenoh::open(c).res().await.unwrap();
    loop {
        z.put("demo/minimal", "minimal").res().await.unwrap();
        async_std::task::sleep(Duration::from_secs(1)).await;
    }
}
