use std::fs::File;
use std::io::prelude::*;
use libvips::{action, VipsApp};

#[test]
fn test_watermark() {
    let app = VipsApp::new("Test Libvips", false).expect("Cannot initialize libvips");
    let images = vec!["test.jpeg", "test.webp", "test.tiff"];
    for i in images.iter() {
        let mut f = File::open(format!("images/{}", i)).unwrap();
        let mut buffer: Vec<u8> = Vec::new();
        f.read_to_end(&mut buffer);

        let mut file = File::create(format!("images/watermark_text_{}", i)).unwrap();
        let result = match action::watermark_text(&buffer, "文字水印", 300, 0.32, "#FF1493"){
            Ok(_result) => _result,
            Err(e) => {
                println!("failed to watermark test {}, err: {:?}", i, e);
                continue
            }
        };
        file.write(&result);
    }

    let i = "test.jpeg";
    let mut f = File::open(format!("images/{}", i)).unwrap();
    let mut buffer: Vec<u8> = Vec::new();
    f.read_to_end(&mut buffer);

    let mut file = File::create(format!("images/watermark_image_{}", i)).unwrap();
    let overlay = match action::resize(&buffer, 100, 100) {
        Ok(_result) => _result,
        Err(e) => {
            println!("failed to resize {}, err: {:?}", i, e);
            return;
        }
    };
    let result = match action::watermark_image(&buffer, &overlay, 0, 0, 0) {
        Ok(_result) => _result,
        Err(e) => {
            println!("failed to watermark image, err: {:?}", e);
            return;
        }
    };
    file.write(&result);
}
