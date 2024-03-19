use std::time::Duration;
use zenoh::prelude::r#async::*;

#[async_std::main]
async fn main() {
    let z = zenoh::open(zenoh::config::default()).res().await.unwrap();
    let value = "This is a message whose integrity needs to be verified";

    loop {
        let _ = z
            .put("demo/attachment", value)
            .res().await;
        async_std::task::sleep(Duration::from_secs(1)).await;
    }

}
