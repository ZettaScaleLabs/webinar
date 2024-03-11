pub mod utils;

use std::env::args;
use zenoh::prelude::r#async::*;

#[async_std::main]
async fn main() {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        println!("Usage:\n\tascii_image_queryable <image-path>");
        return;
    }
    let z = zenoh::open(zenoh::config::default()).res().await.unwrap();
    let value = String::from(&args[1]);
    let recv =
        z.get("demo/transcoder/image2ascii")
            .with_value(value)
            .res().await.unwrap();

    while let Ok(s) = recv.recv_async().await {
        if let Ok(s) = s.sample {
            let bs = s.payload.contiguous().to_vec();
            let img = utils::bytes_to_img(bs);
            println!("{}", String::from_utf8(img.img).unwrap());
        }
    }
}