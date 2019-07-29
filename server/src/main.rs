use rscam::{Camera, Config};
use std::io::Write;
use std::net::TcpListener;
use std::thread;

fn start_cam(id: u8) {
    let mut cam = Camera::new(&format!("/dev/video{}", id)).unwrap();
    cam.start(&Config {
        interval: (1, 5), // 30fps
        resolution: (1920, 1080),
        format: b"H264",
        ..Default::default()
    }).unwrap();

    let listener = TcpListener::bind(&format!("0.0.0.0:580{}", id)).unwrap();
    println!("Server started for {}", id);
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                stream.write(&cam.capture().unwrap()[..]).unwrap();
            },
            Err(e) => println!("Error: {}", e),
        }
    }
    drop(listener); // close the server
}

fn main() {
    let cam0 = thread::spawn(move || {
        start_cam(0);
        0
    });
    cam0.join();
}
