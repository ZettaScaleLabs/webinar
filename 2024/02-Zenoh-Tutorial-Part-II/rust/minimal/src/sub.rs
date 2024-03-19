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
    let sub = z.declare_subscriber("demo/minimal").res().await.unwrap();
    while let Ok(s) = sub.recv_async().await {
        println!("{:?}", &s);
    }
}
