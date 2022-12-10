use image;

fn main() {
    let img = image::open("images/test.png").unwrap();
    let mut buff = image::ImageBuffer::new(img.width(), img.height());
    for x in 0..img.width() {
        for y in 0..img.height() {
            if y >= img.height() / 2 {
                let pixe = image::Rgb([100u8, 100u8, 200u8]);
                buff.put_pixel(x, y, pixe);
            } else {
                let pixe = image::Rgb([50u8, 50u8, 100u8]);
                buff.put_pixel(x, y, pixe);
            }
        }
    }
    buff.save("output.png").unwrap();
}
