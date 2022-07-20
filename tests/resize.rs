use std::fs::File;
use std::io::prelude::*;
use libvips::{action, VipsApp};

#[test]
fn test_resize() {
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

        let mut file = File::create(format!("images/resize_{}", i)).unwrap();
        let result = match action::resize(&buffer, 100, 100) {
            Ok(_result) => _result,
            Err(e) => {
                println!("failed to resize {}, err: {:?}", i, e);
                continue
            }
        };
        file.write(&result);
    }
}
