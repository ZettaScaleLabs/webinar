use zenoh::prelude::r#async::*;

#[async_std::main]
async fn main() {
    let z = zenoh::open(config::default()).await.unwrap();
    let webinar = r#"
    {
        "title": "Understanding Zenoh",
        "speaker": "Angelo Corsaro"
        "date": "26/11/2023",
        "time": "15.00",
        "timezone": "CET"
        "abstract": "some cute abstract here"
        "stars": 5.0,
        "episodes": 1
        "live-audio": "zetta/webinar/zenoh/unleashed/audio"
        "live-video": "zetta/webinar/zenoh/unleashed/video"
    }"#;

    let _ = z
        .put("zetta/webinar/zenoh/unleashed", webinar)
        .encoding(KnownEncoding::AppJson)

        .await;
}
