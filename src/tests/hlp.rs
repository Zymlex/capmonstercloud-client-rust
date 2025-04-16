use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use image::ImageFormat;
use std::io::Cursor;
use std::path::Path;
use tracing::info;

// pub(super) struct TaskCounter<'a> {
//     tasks: Vec<&'a str>,
// }

// impl<'a> TaskCounter<'a> {
//     pub(super) fn new() -> Self {
//         Self {
//             tasks: Vec::with_capacity(30),
//         }
//     }

//     pub(super) fn get_count(&self) -> usize {
//         self.tasks.len()
//     }

//     fn register_task(mut self, name: &'a str) {
//         self.tasks.push(name);
//     }

//     fn unregister_task(mut self, name: &'a str) {
//         // self.tasks.retain(|&x| x == name);
//         let index = self.tasks.iter().position(|&x| x == name).unwrap();
//         self.tasks.remove(index);
//     }
// }

pub(super) fn image_file_to_base64<P>(path: P, format: ImageFormat) -> String
where
    P: AsRef<Path>,
{
    let image = image::open(path).unwrap();

    let mut bytes: Vec<u8> = Vec::new();
    image
        .write_to(&mut Cursor::new(&mut bytes), format)
        .unwrap();

    info!("Image size: {}", bytes.capacity());

    BASE64_STANDARD.encode(bytes)
}
