use std::fs::File;
use std::io::prelude::*;
use libvips::{action, VipsApp};

#[test]
fn test_crop() {
    let app = VipsApp::new("Test Libvips", false).expect("Cannot initialize libvips");
    let images = vec!["test.jpeg", "test.png", "test.webp", "test.tiff"];
    for i in images.iter() {
        let mut f = File::open(format!("images/{}", i)).unwrap();
        let mut buffer: Vec<u8> = Vec::new();
        f.read_to_end(&mut buffer);

        let mut file = File::create(format!("images/crop_{}", i)).unwrap();
        let result = match action::crop(&buffer, 100, 100, 0) {
            Ok(_result) => _result,
            Err(e) => {
                println!("failed to crop {}, err: {:?}", i, e);
                return;
            }
        };
        file.write(&result);
    }
}
