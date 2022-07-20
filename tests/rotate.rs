use std::fs::File;
use std::io::prelude::*;
use libvips::{action, VipsApp};

#[test]
fn test_rotate() {
    let app = VipsApp::new("Test Libvips", false).expect("Cannot initialize libvips");
    let images = vec!["test.jpeg", "test.png", "test.webp", "test.tiff"];
    for i in images.iter() {
        let mut f = File::open(format!("images/{}", i)).unwrap();
        let mut buffer: Vec<u8> = Vec::new();
        f.read_to_end(&mut buffer);

        let mut file = File::create(format!("images/rotate_90_{}", i)).unwrap();
        let result = match action::rotate(&buffer, 90) {
            Ok(_result) => _result,
            Err(e) => {
                println!("failed to rotate {}, err: {:?}", i, e);
                continue
            }
        };
        file.write(&result);
    }
}
