use opencv::{prelude::*, videoio};
use std::env;

pub fn get_camera_id() -> i32 {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        0
    } else {
        args[1].parse().unwrap()
    }
}

pub fn get_camera(camera_id: i32) -> videoio::VideoCapture {
    let cam = videoio::VideoCapture::new(camera_id, videoio::CAP_ANY).unwrap();
    let opened = videoio::VideoCapture::is_opened(&cam).unwrap();
    if !opened {
        panic!("Unable to open default camera!");
    }
    cam
}

pub fn get_encode_options() -> opencv::types::VectorOfi32 {
    let mut encode_options = opencv::types::VectorOfi32::new();
    encode_options.push(opencv::imgcodecs::IMWRITE_JPEG_QUALITY);
    encode_options.push(90);
    encode_options
}

pub fn capture_frame(
    cam: &mut videoio::VideoCapture,
    width: i32,
    height: i32,
    encode_options: &opencv::types::VectorOfi32,
) -> Option<opencv::types::VectorOfu8> {
    let mut frame = Mat::default();
    cam.read(&mut frame).unwrap();

    if !frame.empty() {
        let mut reduced = Mat::default();
        opencv::imgproc::resize(
            &frame,
            &mut reduced,
            opencv::core::Size::new(width, height),
            0.0,
            0.0,
            opencv::imgproc::INTER_LINEAR,
        )
        .unwrap();

        let mut buf = opencv::types::VectorOfu8::new();
        opencv::imgcodecs::imencode(".jpeg", &reduced, &mut buf, encode_options).unwrap();
        Some(buf)
    } else {
        None
    }
}
