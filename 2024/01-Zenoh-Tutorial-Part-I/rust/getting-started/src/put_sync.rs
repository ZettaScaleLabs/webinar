use zenoh::prelude::sync::*;

fn main() {
    let z = zenoh::open(config::default()).unwrap();
    println!("Putting Data");
    let _ = z.put("zetta/webinar/zenoh/unleashed/time", "15:00");
    let _ = z.put("zetta/webinar/zenoh/unleashed/place", "Online");
    let _ = z
        .put("zetta/webinar/zenoh/unleashed/speaker", "Angelo Corsaro")
        ;
    let _ = z.put("zetta/webinar/zenoh/unleashed/stars", 5); // :-)
}
