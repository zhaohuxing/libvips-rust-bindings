use std::fs::File;
use std::io::prelude::*;
use libvips::{action, VipsApp};

#[test]
fn test_format() {
    let app = VipsApp::new("Test Libvips", false).expect("Cannot initialize libvips");

    let images = vec!["test.png", "test.webp", "test.tiff"];
    format_type(images, "jpeg");

    let images = vec!["test.jpeg", "test.webp", "test.tiff"];
    format_type(images, "png");

    let images = vec!["test.jpeg", "test.png", "test.tiff"];
    format_type(images, "webp");

    let images = vec!["test.jpeg", "test.png", "test.webp"];
    format_type(images, "tiff");
}

fn format_type(images: Vec<&str>, ftype: &str) {
    for i in images.iter() {
        let mut f = File::open(format!("images/{}", i)).unwrap();
        let mut buffer: Vec<u8> = Vec::new();
        f.read_to_end(&mut buffer);

        let strs: Vec<&str> = i.split(".").collect();
        let mut file = File::create(format!(
            "images/format_{}_from_{}.{}",
            ftype, strs[1], ftype
        ))
        .unwrap();

        let result = match action::format(&buffer, ftype) {
            Ok(_result) => _result,
            Err(e) => {
                println!("i format to ftype failed, err: {:?}", e);
                return
            }
        };
        file.write(&result);
    }
}
