pub mod utils;

use std::env::args;
use zenoh::prelude::r#async::*;

#[async_std::main]
async fn main() {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        println!("Usage:\n\tascii-image-converter <image-path>\n");
        return;
    }

    let img = utils::proto::Image::from(utils::img2ascii(&args[1]));

    let c = Config::default();
    let z = zenoh::open(c).res().await.unwrap();
    let _ = z.put("demo/bulletin/ascii-image", utils::img_to_bytes(&img)).res().await;
}