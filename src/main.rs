extern crate image;
use image::ImageBuffer;
use image::{RgbImage, Rgb};
use image::DynamicImage;
use image::GenericImageView;

fn merge_bit(pixel1 : Rgb<u8>, pixel2: Rgb<u8>) -> Rgb<u8> {
    let r1 = msb(pixel1[0]);
    let g1 = msb(pixel1[1]);
    let b1 = msb(pixel1[2]);

    let r2 = msb_to_lsb(msb(pixel2[0]));
    let g2 = msb_to_lsb(msb(pixel2[1]));
    let b2 = msb_to_lsb(msb(pixel2[2]));
    Rgb([r1 + r2, g1 + g2, b1 + b2])
}


/* get the Most significant bits (the 4 rightmost bits) */
fn msb(number: u8) -> u8 {
    // 240 == '11110000'
    number & 240
}

fn msb_to_lsb(number: u8) -> u8 {
    // move to left four times
    number >> 4
}

fn encrypt(image1 : DynamicImage, image2: DynamicImage) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let (width_img1 , height_img1) = image1.dimensions();
    let (width_img2, height_img2) = image2.dimensions();
    println!("image1 ({:?}, {:?})", width_img1, height_img1);
    println!("image2 ({:?}, {:?})", width_img2, height_img2);

    if (width_img1 < width_img2) || (height_img1 < height_img2) {
        panic!("Image 1 should not be larger than Image 2");
    }

    let img1_rgb = image1.to_rgb8();
    let img2_rgb = image2.to_rgb8();
    let mut result_img_rgb = RgbImage::new(width_img1, height_img1);

    for x in 0..width_img1 {
        for y in 0..height_img1 {
            let rbg_converted = if x < width_img2 && y < height_img2 {
                *img2_rgb.get_pixel(x, y)
            } else {
                Rgb([0,0,0])
            };
            result_img_rgb.put_pixel(x, y, merge_bit(*img1_rgb.get_pixel(x, y), rbg_converted));
        }
    }
    return result_img_rgb;
}

fn main() {
    let img1 = image::open("./sample1.jpg").expect("failed to open image file");
    let img2 = image::open("./sample2.jpeg").expect("failed to open image file");
    
    let encrypted_image = encrypt(img1, img2);
    encrypted_image.save("test.png").ok().expect("Saving image failed");
}
