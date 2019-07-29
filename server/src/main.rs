use rscam::{Camera, Config};
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut cam = Camera::new("/dev/video0").unwrap();
    cam.start(&Config {
        interval: (1, 5), // 30fps
        resolution: (1920, 1080),
        format: b"H264",
        ..Default::default()
    }).unwrap();

    let mut test = File::create("test.h264").unwrap();
    loop {
        test.write_all(&cam.capture().unwrap()[..]);
    }
}
