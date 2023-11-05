extern crate image;
use image::{open, GenericImageView, ImageBuffer, Pixel, RgbaImage};

fn create_subimages(
    digits: [&str; 16],
    dynamic_image: RgbaImage,
    cell_width: u32,
    cell_height: u32,
    subimage_width: u32,
    subimage_height: u32,
    width: u32,
    height: u32,
) {
    for cell_y in 0..16 {
        for cell_x in 0..30 {
            let cell_start_x = cell_x * cell_width;
            let cell_start_y = cell_y * cell_height;

            // Create an ImageBuffer for the sub-image
            let mut subimage = ImageBuffer::new(subimage_width, subimage_height);

            // Copy the pixel data into the sub-image
            for y in 0..subimage_height {
                for x in 0..subimage_width {
                    if cell_start_x + x < width && cell_start_y + y < height {
                        let pixel = dynamic_image.get_pixel(cell_start_x + x, cell_start_y + y);
                        subimage.put_pixel(x, y, pixel.to_luma());
                    }
                }
            }
            let subimage_path = format!("output\\{}{:02}.bmp", digits[cell_y as usize], cell_x + 1);
            subimage
                .save(subimage_path)
                .expect("something went wrong saving the image");
        }
    }
}

fn main() {
    let digits: [&str; 16] = [
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "A", "B", "C", "D", "E", "F",
    ];
    let bmp_image = open("digits_bmp.bmp").expect("something went wrong opening the image");
    let dynamic_image = bmp_image.to_rgba8();

    let (width, height) = dynamic_image.dimensions();

    let cell_width = width / 30;
    let cell_height = height / 16;

    let subimage_width = 128;
    let subimage_height = 128;

    create_subimages(
        digits,
        dynamic_image,
        cell_width,
        cell_height,
        subimage_width,
        subimage_height,
        width,
        height,
    );
}
