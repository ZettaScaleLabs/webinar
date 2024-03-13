use zenoh::prelude::r#async::*;

#[async_std::main]
async fn main() {
    let z = zenoh::open(zenoh::config::default()).res().await.unwrap();
    let sub = z
        .declare_subscriber("zetta/webinar/**")
        .res()
        .await
        .unwrap();
    println!("Waiting for data...");

    while let Ok(s) = sub.recv_async().await {
        println!(
            " ({}, {}) -- Encoding: {}",
            s.key_expr.as_str(),
            s.value,
            s.encoding
        );
    }
}
