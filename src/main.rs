use image::{self, GenericImageView, DynamicImage, ImageBuffer, Luma};
use std::{fs};

fn main() {
    const OUT_DIR: &str = "output";
    if !std::path::Path::new(OUT_DIR).exists() {
        let msg = "Cannot crate dir ".to_string() + OUT_DIR;
        fs::create_dir(OUT_DIR)
        .expect(msg.as_str());
    }

    let img = image::open("images/test4.jpg").unwrap();
    let gray = gray_scale(&img);
    gray.save(OUT_DIR.to_owned() + "/gray.png").unwrap();
}

fn gray_scale(img: &DynamicImage) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let mut buff: image::ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::new(img.width(), img.height());
    for x in 0..img.width() {
        for y in 0..img.height() {
            let pixel = img.get_pixel(x, y);
            let r = (pixel[0] as f32) * 0.2126;
            let g = (pixel[1] as f32) * 0.7152;
            let b = (pixel[2] as f32) * 0.0722;
            let weight_sum = r + g + b;
            let weight_sum_u8 = weight_sum as u8;
            let new_pix = image::Luma([weight_sum_u8]);
            buff.put_pixel(x, y, new_pix);
        }
    }
    return buff;
}
