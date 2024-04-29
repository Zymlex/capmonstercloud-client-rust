use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use std::io::Cursor;
use std::path::Path;
use image::ImageFormat;

pub(super) fn image_file_to_base64<P>(path: P, format: ImageFormat) -> String
where
    P: AsRef<Path>,
{
    let image = image::open(path).unwrap();

    let mut bytes: Vec<u8> = Vec::new();
    image
        .write_to(&mut Cursor::new(&mut bytes), format)
        .unwrap();

    println!("capacity: {}", bytes.capacity());

    BASE64_STANDARD.encode(bytes)
}