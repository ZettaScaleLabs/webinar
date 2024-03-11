pub mod utils;

use std::env::args;
use zenoh::prelude::r#async::*;
use async_std::io;

#[async_std::main]
async fn main() {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        println!("Usage:\n\tascii_image_queryable <images-directory>");
        return;
    }
    let z = zenoh::open(zenoh::config::default()).res().await.unwrap();
    let queryable =
        z.declare_queryable("demo/transcoder/image2ascii").res().await.unwrap();

    while let Ok(q) = queryable.recv_async().await {

        let r = match q.value() {
            Some(v) => {
                // let img_name: String = (v.payload.clone()).into();
                let img_name: String = String::from_utf8(v.payload.contiguous().to_vec()).unwrap();
                let str_image = utils::img2ascii(&img_name);
                println!("Converted Image:\n{}", &str_image);
                let img  = utils::proto::Image::from(str_image);
                Ok(Sample::new(q.key_expr().clone(), utils::img_to_bytes(&img)))
            }
            _ => {
                let error_msg = String::from("image not found");
                Ok(Sample::new(q.key_expr().clone(), error_msg))
            }
        };
        let _ = q.reply(r).res().await;
    }
    println!("Type something to exit...");
    let mut str = String::new();
    let _ = io::stdin().read_line(&mut str).await.unwrap();
}