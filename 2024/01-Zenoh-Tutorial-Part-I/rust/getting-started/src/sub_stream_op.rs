use zenoh::prelude::r#async::*;
use futures::stream::StreamExt;

#[async_std::main]
async fn main() -> zenoh::Result<()> {
    let ke = "zetta/webinar/zenoh/**";
    let z = zenoh::open(config::default()).await?;
    let  sub = z.declare_subscriber(ke).await?;
    
    let stream = sub.stream();
    let mut kvs = stream.map(|s: Sample| { (s.key_expr, s.value.payload) });
    while let Some((k, v)) = kvs.next().await {
        println!("key: {}, value: {:?}", k, v);
    }
    
    Ok(())
}