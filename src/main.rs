extern crate image;
use image::imageops::crop_imm;
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

fn split_pixel(pixel: Rgb<u8>) -> (Rgb<u8>, Rgb<u8>) {
    let r1 = msb(pixel[0]);
    let g1 = msb(pixel[1]);
    let b1 = msb(pixel[2]);

    let r2 = lsb_to_msb(lsb(pixel[0]));
    let g2 = lsb_to_msb(lsb(pixel[1]));
    let b2 = lsb_to_msb(lsb(pixel[2]));

    (Rgb([r1,g1,b1]), Rgb([r2,g2,b2]))
}

/* get the Most significant bits (the 4 rightmost bits) */
fn msb(number: u8) -> u8 {
    // 240 == '11110000'
    number & 240
}

fn lsb(number: u8) -> u8 {
    // 15 = '00001111'
    number & 15
}

fn msb_to_lsb(number: u8) -> u8 {
    // move to left four times
    number >> 4
}

fn lsb_to_msb(number: u8) -> u8 {
    number << 4
}


fn encrypt(image1 : DynamicImage, image2: DynamicImage) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let (width_img1 , height_img1) = image1.dimensions();
    let (width_img2, height_img2) = image2.dimensions();
    
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

fn decrypt(image: DynamicImage, to_crop: bool)  -> (ImageBuffer<Rgb<u8>, Vec<u8>>, ImageBuffer<Rgb<u8>, Vec<u8>>) {
    let (width, height) = image.dimensions();
    
    let image_rbg = image.to_rgb8();
    let mut result_img_rbg1 = RgbImage::new(width, height);
    let mut result_img_rbg2 = RgbImage::new(width, height);

    let (mut img2_width, mut img2_height) = (width, height);


    for x in 0..width {
        for y in 0..height {
            let (rgb1, rgb2) = split_pixel(*image_rbg.get_pixel(x, y));
            result_img_rbg1.put_pixel(x, y, rgb1);
            result_img_rbg2.put_pixel(x, y, rgb2);

            // try to detect if image 2 has black border (meaning that the second image is smaller)
            if rgb2[0] != 0 || rgb2[1] != 0 || rgb2[2] != 0 {
                img2_width = x + 1;
                img2_height = y + 1;
            }
        }
    }

    if to_crop {
        let cropped_image = crop_imm(&result_img_rbg2, 0, 0, img2_width, img2_height);
        return (result_img_rbg1, cropped_image.to_image())
    }
    else {
        return (result_img_rbg1, result_img_rbg2)
    }

}

fn main() {
    let img1 = image::open("./sample/sample1.jpg").expect("failed to open image file");
    let img2 = image::open("./sample/sample2.jpeg").expect("failed to open image file");
    
    let encrypted_image = encrypt(img1, img2);
    encrypted_image.save("results/encrypt.png").ok().expect("Saving image failed");

    let (img_original, img_hidden) = decrypt(image::DynamicImage::ImageRgb8(encrypted_image), true);
    img_original.save("./results/decrypt_1.png").ok().expect("Saving image failed");
    img_hidden.save("./results/decrypt_2.png").ok().expect("Saving image failed");
}
