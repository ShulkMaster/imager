use image::{self, GenericImageView};

fn main() {
    let img = image::open("images/test.png").unwrap();
    let mut buff: image::ImageBuffer<image::Luma<u8>, Vec<u8>> = image::ImageBuffer::new(img.width(), img.height());
    for x in 0..img.width() {
        for y in 0..img.height() {
            let pixel = img.get_pixel(x, y);
            let r = pixel[0] as u16;
            let g = pixel[1] as u16;
            let b = pixel[2] as u16;
            let sum = r + g + b;
            let average = (sum / 3) as u8;
            let new_pix = image::Luma([average]);
            buff.put_pixel(x, y, new_pix);
        }
    }
    buff.save("output.png").unwrap();
}
