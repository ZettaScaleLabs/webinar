use zenoh::prelude::r#async::*;

fn get_qid() -> String {
    let args : Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        args[1].clone()    
    } else { String::from("Webinar-Queryable")}
}
#[async_std::main]
async fn main() {
    let qid =  get_qid();   
    let z = zenoh::open(config::default()).await.unwrap();
    let queryable = 
        z.declare_queryable("zetta/webinar/zenoh/**")
        .complete(false)
        .await.unwrap();

    while let Ok(query) = queryable.recv_async().await {
        println!("Received Query: {}", query);
        let _ = query.reply(Ok(Sample::new(query.key_expr().clone(), qid.clone()))).await;
    }
}