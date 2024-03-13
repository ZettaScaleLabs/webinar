use zenoh::prelude::r#async::*;

#[async_std::main]
async fn main() {
    match zenoh::open(config::default()).res().await {
        Ok(z) => {
            println!("Putting Data");
            let _ = z
                .put("zetta/webinar/zenoh/unleashed/time", "15:00")
                .priority(Priority::DataHigh)
                .res()
                .await;
            let _ = z
                .put("zetta/webinar/zenoh/unleashed/place", "Online")
                .res()
                .await;
            let _ = z
                .put("zetta/webinar/zenoh/unleashed/speaker", "Angelo Corsaro")
                .res()
                .await;
            let _ = z
                .put("zetta/webinar/zenoh/unleashed/stars", 5.0)
                .res()
                .await;

            let _ = z
                .put("zetta/webinar/zenoh/unleashed/episodes", 1)
                .res()
                .await;

            let vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

            let _ = z
                .put("zetta/webinar/zenoh/unleashed/attachment", vec)
                .res()
                .await;
        }
        Err(e) => println!("Failed to open the session because of:\n\t {}", e),
    }
}
