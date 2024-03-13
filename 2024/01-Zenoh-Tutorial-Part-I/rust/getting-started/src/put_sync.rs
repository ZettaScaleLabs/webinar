use zenoh::prelude::sync::*;

fn main() {
    let z = zenoh::open(config::default()).res().unwrap();
    println!("Putting Data");
    let _ = z.put("zetta/webinar/zenoh/unleashed/time", "15:00").res();
    let _ = z.put("zetta/webinar/zenoh/unleashed/place", "Online").res();
    let _ = z
        .put("zetta/webinar/zenoh/unleashed/speaker", "Angelo Corsaro")
        .res();
    let _ = z.put("zetta/webinar/zenoh/unleashed/stars", 5).res(); // :-)
}
