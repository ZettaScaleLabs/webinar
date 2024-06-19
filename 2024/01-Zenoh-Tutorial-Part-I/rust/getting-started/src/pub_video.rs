use zenoh::prelude::sync::*;
mod utils;
use utils::*;

// Hint: to list the cameras on your computer you can do:
// $ ffmpeg -hide_banner -list_devices true -f avfoundation -i dummy
fn main() {
    let camera_id = get_camera_id();
    let width = 1280;
    let height = 720;
    let capture_period = 40; // msec

    let z = zenoh::open(config::default()).unwrap();
    let z_pub = z
        .declare_publisher("zetta/webinar/zenoh/unleashed/video")
        .priority(Priority::RealTime)
        .congestion_control(CongestionControl::Drop)

        .unwrap();

    let mut cam = get_camera(camera_id);
    let encode_options = get_encode_options();

    loop {
        if let Some(buf) = capture_frame(&mut cam, width, height, &encode_options) {
            z_pub.put(buf.to_vec()).unwrap();
            std::thread::sleep(std::time::Duration::from_millis(capture_period));
        } else {
            println!("Reading empty buffer from camera... Waiting some more....");
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }
}
