use std::fs::File;
use std::io::prelude::*;
use libvips::{action, VipsApp};

#[test]
fn test_get_metadata() {
    let app = VipsApp::new("Test Libvips", false).expect("Cannot initialize libvips");
    let images = vec![
        "test.jpeg",
        "test.png",
        "test.webp",
        "test.tiff",
        "test.gif",
    ];
    for i in images.iter() {
        let mut f = File::open(format!("images/{}", i)).unwrap();
        let mut buffer: Vec<u8> = Vec::new();
        f.read_to_end(&mut buffer);
        println!("{:?}", action::get_metadata(&buffer));
    }
}
