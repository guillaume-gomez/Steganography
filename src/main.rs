use clap::{Arg, App};
use crate::steganography::Steganography;

mod steganography;


fn main() {

    let matches = App::new("Steganography converter")
      .version("1.0")
      .author("Guillaume G. <guillaume.gomez846@gmail.com>")
      .about("Hide an image inside another")
      .arg(Arg::with_name("action")
           .short("a")
           .long("action")
           .required(true)
           .help("Whether encrypt or decrypt. action=['encrypt' | 'decrypt']")
           .index(1))
      .arg(Arg::with_name("input")
           .short("i")
           .long("input")
           .required(true)
           .multiple(true)
           .takes_value(true)
           .help("input files to be encrypted or file to be decrypted. input=[filename1| filename2] or input=[filename]"))
       .arg(Arg::with_name("output")
           .short("o")
           .long("output")
           .required(true)
           .multiple(true)
           .takes_value(true)
           .help("outut files to be encrypted or file to be decrypted. input=[filename] or input=[filename1| filename2]"))
      .get_matches();

    let inputs: Vec<&str> = matches.values_of("input").unwrap().collect();
    let outputs: Vec<&str> = matches.values_of("output").unwrap().collect();

    match  matches.value_of("action").unwrap() as &str {
        "encrypt" => {

            let img1 = image::open(inputs[0]).expect("failed to open the first image file");
            let img2 = image::open(inputs[1]).expect("failed to open the second image file");
            
            let encrypted_image = Steganography::encrypt(img1, img2);
            encrypted_image.save(outputs[0]).ok().expect("Saving the ouput image failed");
        },
        "decrypt" => {
            let encrypted_image = image::open(inputs[0]).expect("failed to open the input image file");
            let (img_original, img_hidden) = Steganography::decrypt(encrypted_image, true);
            img_original.save(outputs[0]).ok().expect("Saving the first image failed");
            img_hidden.save(outputs[1]).ok().expect("Saving the second image failed");
        },
        _ => println!("Unknow Action. Please set 'encrypt' or 'decrypt'"),
    }

}
