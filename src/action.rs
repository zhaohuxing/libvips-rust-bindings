use crate::VipsImage;
use crate::Result;
use crate::bindings;
use crate::ops;
use crate::ops::image_get_string;

use std::cmp;
use std::collections::HashMap;
use std::ffi::{CString};

pub fn format(buf: &[u8], format_type: &str) -> Result<Vec<u8>> {
    let image = VipsImage::new_from_buffer(buf, "")?;
    return image.image_write_to_buffer(&format!(".{}", format_type))
}

pub fn crop(buf: &[u8], width: i32, height: i32, gravity: i32) -> Result<Vec<u8>> {
    let _type = vips_image_type(buf);
    let input = VipsImage::new_from_buffer(buf, "")?;

    let _width: i32 = input.get_width();
    let _height: i32 = input.get_height();
    let width_in: i32 = cmp::min(_width, width);
    let height_in: i32 = cmp::min(_height, height);
    let (left, top) = calculate_crop(_width, _height, width, height, gravity);
    let image = ops::extract_area(&input, left, top, width_in, height_in)?; 
    return image.image_write_to_buffer(&format!(".{}", _type))
}

#[derive(Debug)]
pub struct Metadata {
    pub width: i32,
    pub height: i32,
    pub orientation: String,
    pub alpha: bool,
    pub ttype: String,
    pub space: String,
    pub make: String,
    pub mode: String,
    pub datetime: String,
    pub exif_version: String,
    pub focal_length: String,
    pub gps_latitude_ref: String,
    pub gps_latitude: String,
    pub gps_longitude_ref: String,
    pub gps_longitude: String,
    pub gps_altitude_ref: String,
    pub gps_altitude: String,
}  

pub fn get_metadata(buf: &[u8]) -> Result<Metadata> {
    let ttype = vips_image_type(buf);
    let image = VipsImage::new_from_buffer(buf, "")?;

    let orientation = image_get_string(&image, "exif-ifd0-Orientation");
    let make = image_get_string(&image, "exif-ifd0-Make");
    let mode = image_get_string(&image, "exif-ifd0-Model");
    let datetime = image_get_string(&image, "exif-ifd0-DateTime");
    let exif_version = image_get_string(&image, "exif-ifd2-ExifVersion");
    let focal_length = image_get_string(&image, "exif-ifd2-FocalLength");
    let gps_latitude_ref = image_get_string(&image, "exif-ifd3-GPSLatitudeRef");
    let gps_latitude = image_get_string(&image, "exif-ifd3-GPSLatitude");
    let gps_longitude_ref = image_get_string(&image, "exif-ifd3-GPSLongitudeRef");
    let gps_longitude = image_get_string(&image, "exif-ifd3-GPSLongitude");
    let gps_altitude_ref = image_get_string(&image, "exif-ifd3-GPSAltitudeRef");
    let gps_altitude = image_get_string(&image, "exif-ifd3-GPSAltitude");
    let width = image.get_width();
    let height = image.get_height();
    let alpha = false;
    let space = String::from("");
    let metadata = Metadata {
        width,
        height,
        ttype,
        space,
        orientation,
        alpha,
        make,
        mode,
        datetime,
        exif_version,
        focal_length,
        gps_latitude_ref,
        gps_latitude,
        gps_longitude_ref,
        gps_longitude,
        gps_altitude_ref,
        gps_altitude,
    };

    return Ok(metadata);
}

pub fn resize(buf: &[u8], width: i32, height: i32) -> Result<Vec<u8>> {
    let _type = vips_image_type(buf);
    let image = ops::thumbnail_buffer_with_opts1(buf, width, height, &_type)?;
    if _type == "gif" {
        return image.image_write_to_magicksave_buffer(); 
    }
    return image.image_write_to_buffer(&format!(".{}", _type))
}

pub fn rotate(buf: &[u8], angle: i32) -> Result<Vec<u8>> {
    let _type = vips_image_type(buf);
    let input = VipsImage::new_from_buffer(buf, "")?;
    let angle_in = match angle {
        90_i32 => ops::Angle::D90,  // Angle 90
        180_i32 => ops::Angle::D180, // Angle 180
        270_i32 => ops::Angle::D270, // Angle 270
        _ => ops::Angle::D0,       // Angle 0
    };
    let image = ops::rot(&input, angle_in)?;
    return image.image_write_to_buffer(&format!(".{}", _type))
}

pub fn watermark_text(buf: &[u8], text: &str, dpi: i32, _opacity: f32, color: &str) -> Result<Vec<u8>> {
    let _type = vips_image_type(buf);
    let overlay = ops::text_with_opts1(text, dpi, color)?;
    let input = VipsImage::new_from_buffer(buf, "")?;
    let image = ops::composite_2(&input, &overlay, ops::BlendMode::Over)?;
    return image.image_write_to_buffer(&format!(".{}", _type))
}

pub fn watermark_image(buf: &[u8], overlay: &[u8], _x: i32, _y: i32, _opacity: i32) -> Result<Vec<u8>> {
    let _type = vips_image_type(buf);
    let overlay_in = VipsImage::new_from_buffer(overlay, "")?;
    let input = VipsImage::new_from_buffer(buf, "")?;
    let image = ops::composite_2(&input, &overlay_in, ops::BlendMode::Over)?;
    return image.image_write_to_buffer(&format!(".{}", _type))
}

fn calculate_crop(
    in_width: i32,
    in_height: i32,
    out_width: i32,
    out_height: i32,
    gravity: i32,
) -> (i32, i32) {
    let mut left = 0;
    let mut top = 0;
    match gravity {
        1 => {
            left = (in_width - out_width + 1) / 2;
        }
        2 => {
            left = in_width - out_width;
            top = (in_height - out_height + 1) / 2;
        }
        3 => {
            left = (in_width - out_width + 1) / 2;
            top = in_height - out_height;
        }
        4 => {
            top = (in_width - out_width + 1) / 2;
        }
        _ => {
            left = (in_width - out_width + 1) / 2;
            top = (in_height - out_height + 1) / 2;
        }
    }
    return (left, top);
} 

lazy_static! {
    static ref ImageTypes: HashMap<String, bool> = {
        let mut map = HashMap::new();
        map.insert("JPEG".to_string(), true);
        map.insert("PNG".to_string(), true);
        map.insert("WEBP".to_string(), true);
        map.insert("TIFF".to_string(), true);
        map.insert("GIF".to_string(), true);
        map.insert("PDF".to_string(), true);
        map.insert("SVG".to_string(), true);
        map.insert("MAGICK".to_string(), true);
        map.insert("HEIF".to_string(), true);
        map.insert("AVIF".to_string(), true);
        map
    };
}

fn vips_image_type(buf: &[u8]) -> String {
    if buf.len() < 12 {
        return "unknown".to_string();
    }
    if buf[0] == 0xFF && buf[1] == 0xD8 && buf[2] == 0xFF {
        return "jpeg".to_string();
    }

    if is_type_supported("GIF") && buf[0] == 0x47 && buf[1] == 0x49 && buf[2] == 0x46 {
        return "gif".to_string();
    }
    if buf[0] == 0x89 && buf[1] == 0x50 && buf[2] == 0x4E && buf[3] == 0x47 {
        return "png".to_string();
    }
    if is_type_supported("TIFF")
        && ((buf[0] == 0x49 && buf[1] == 0x49 && buf[2] == 0x2A && buf[3] == 0x0)
            || (buf[0] == 0x4D && buf[1] == 0x4D && buf[2] == 0x0 && buf[3] == 0x2A))
    {
        return "tiff".to_string();
    }
    if is_type_supported("PDF")
        && buf[0] == 0x25
        && buf[1] == 0x50
        && buf[2] == 0x44
        && buf[3] == 0x46
    {
        return "pdf".to_string();
    }
    if is_type_supported("WEBP")
        && buf[8] == 0x57
        && buf[9] == 0x45
        && buf[10] == 0x42
        && buf[11] == 0x50
    {
        return "webp".to_string();
    }
    // TODO: implement is_svg_image
    //if IsTypeSupported(SVG) && IsSVGImage(buf) {
    //	return SVG
    //}

    // TODO: Not implement magick
    //if IsTypeSupported(MAGICK) && strings.HasSuffix(readImageType(buf), "MagickBuffer") {
    //	return MAGICK
    //}
    //
    // NOTE: libheif currently only supports heic sub types; see:
    //   https://github.com/strukturag/libheif/issues/83#issuecomment-421427091
    if is_type_supported("HEIF")
        && buf[4] == 0x66
        && buf[5] == 0x74
        && buf[6] == 0x79
        && buf[7] == 0x70
        && buf[8] == 0x68
        && buf[9] == 0x65
        && buf[10] == 0x69
        && buf[11] == 0x63
    {
        // This is a HEIC file, ftypheic
        return "heif".to_string();
    }
    if is_type_supported("HEIF")
        && buf[4] == 0x66
        && buf[5] == 0x74
        && buf[6] == 0x79
        && buf[7] == 0x70
        && buf[8] == 0x6d
        && buf[9] == 0x69
        && buf[10] == 0x66
        && buf[11] == 0x31
    {
        // This is a HEIF file, ftypmif1
        return "heif".to_string();
    }
    if is_type_supported("HEIF")
        && buf[4] == 0x66
        && buf[5] == 0x74
        && buf[6] == 0x79
        && buf[7] == 0x70
        && buf[8] == 0x6d
        && buf[9] == 0x73
        && buf[10] == 0x66
        && buf[11] == 0x31
    {
        // This is a HEIFS file, ftypmsf1
        return "heif".to_string();
    }
    if is_type_supported("HEIF")
        && buf[4] == 0x66
        && buf[5] == 0x74
        && buf[6] == 0x79
        && buf[7] == 0x70
        && buf[8] == 0x68
        && buf[9] == 0x65
        && buf[10] == 0x69
        && buf[11] == 0x73
    {
        // This is a HEIFS file, ftypheis
        return "heif".to_string();
    }
    if is_type_supported("HEIF")
        && buf[4] == 0x66
        && buf[5] == 0x74
        && buf[6] == 0x79
        && buf[7] == 0x70
        && buf[8] == 0x68
        && buf[9] == 0x65
        && buf[10] == 0x76
        && buf[11] == 0x63
    {
        // This is a HEIFS file, ftyphevc
        return "heif".to_string();
    }
    if is_type_supported("HEIF")
        && buf[4] == 0x66
        && buf[5] == 0x74
        && buf[6] == 0x79
        && buf[7] == 0x70
        && buf[8] == 0x61
        && buf[9] == 0x76
        && buf[10] == 0x69
        && buf[11] == 0x66
    {
        return "avif".to_string();
    }
    return "unknown".to_string();
}

fn is_type_supported(t: &str) -> bool {
    return ImageTypes.contains_key(t) && vips_type_find_bridge(t) != 0;
}

fn vips_type_find_bridge(t: &str) -> u64 {
    match t {
        "GIF" => return vips_type_find("VipsOperation", "gifload"),
        "PDF" => return vips_type_find("VipsOperation", "pdfload"),
        "TIFF" => return vips_type_find("VipsOperation", "tiffload"),
        "SVG" => return vips_type_find("VipsOperation", "svgload"),
        "WEBP" => return vips_type_find("VipsOperation", "webpload"),
        "PNG" => return vips_type_find("VipsOperation", "pngload"),
        "JPEG" => return vips_type_find("VipsOperation", "jpegload"),
        "MAGICK" => return vips_type_find("VipsOperation", "magickload"),
        "HEIF" => return vips_type_find("VipsOperation", "heifload"),
        _ => return 0,
    }
}

fn vips_type_find(name: &str, nickname: &str) -> u64 {
    unsafe {
        let name_in = CString::new(name).unwrap();
        let nickname_in = CString::new(nickname).unwrap();
        let ret = bindings::vips_type_find(name_in.as_ptr(), nickname_in.as_ptr());
        return ret;
    }
}
