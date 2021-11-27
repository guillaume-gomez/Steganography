use crate::steganography::Steganography;

mod steganography;


fn main() {
    let img1 = image::open("./sample/sample1.jpg").expect("failed to open image file");
    let img2 = image::open("./sample/sample2.jpeg").expect("failed to open image file");
    
    let encrypted_image = Steganography::encrypt(img1, img2);
    encrypted_image.save("results/encrypt.png").ok().expect("Saving image failed");

    let (img_original, img_hidden) = Steganography::decrypt(image::DynamicImage::ImageRgb8(encrypted_image), true);
    img_original.save("./results/decrypt_1.png").ok().expect("Saving image failed");
    img_hidden.save("./results/decrypt_2.png").ok().expect("Saving image failed");
}
