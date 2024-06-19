use zenoh::prelude::*;

#[tokio::main]
async fn main() {
    let z = zenoh::open(config::default()).await.unwrap();
    let k = "zetta/webinar/zenoh/*";
    
    let results = z.get(k)
        .target(QueryTarget::BestMatching)
        .consolidation(ConsolidationMode::None)
        .await.unwrap();
    
    while let Ok(reply) = results.recv_async().await {
        match reply.sample {
            Ok(sample) => println!(
                ">> Received ('{}': '{}') from: {}",
                sample.key_expr.as_str(),
                sample.value,
                reply.replier_id
            ),
            Err(err) => println!(">> Received (ERROR: '{}')", String::try_from(&err).unwrap()),
        }
    }
}