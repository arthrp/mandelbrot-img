extern crate image;
extern crate num_complex;

use std::env;

use image::ImageBuffer;
use image::Rgb;
use num_complex::Complex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() > 1 { &args[1] } else { "mandelbrot.png" };

    let img_width = 800;
    let img_height = 800;

    let scale_x = 3.0 / img_width as f32;
    let scale_y = 3.0 / img_height as f32;

    let mut img_buffer = ImageBuffer::new(img_width, img_height);

    for (x, y, pixel) in img_buffer.enumerate_pixels_mut() {
        let cy = y as f32 * scale_x - 1.5;
        let cx = x as f32 * scale_y - 1.5;

        let c = Complex::new(-cx, -cy);
        let mut z = Complex::new(0.0, 0.0);

        let mut i = 0;
        for t in 0..256 {
            if z.norm() <= 2.0 {
                z = z * z + c;
                i = t;
            } else {
                break;
            }
        }

        let image = Rgb([
            i as u8,
            i as u8,
            i as u8,
        ]);
        *pixel = image;
    }

    img_buffer.save(filename).unwrap();
}