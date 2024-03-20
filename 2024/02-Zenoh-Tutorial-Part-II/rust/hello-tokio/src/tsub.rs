use zenoh::prelude::r#async::*;
#[tokio::main]
async fn main() {
    let z = zenoh::open(zenoh::config::default()).res().await.unwrap();
    let sub =
        z.declare_subscriber("demo/tokio/*").res().await.unwrap();

    while let Ok(s) = sub.recv_async().await {
        println!("({}, {})",
                 String::from_utf8_lossy(s.key_expr.as_bytes()),
                 String::from_utf8_lossy(&s.payload.contiguous()));
    }
}