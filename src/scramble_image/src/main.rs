// For dev
#![allow(dead_code)]
// #![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(unused_mut)]

#[macro_use] extern crate derive_builder;
extern crate image;
extern crate scramble_image;
extern crate imageproc;

pub mod chaotic_maps;

use chaotic_maps::*;
use std::path::Path;


fn test_henon() {
    // Get input image from path
    let path = String::from("examples/secret_image_lena.png");
    let dest_path_encrypted = String::from("examples/output_henon_encrypted.png");
    let dest_path_decrypted = String::from("examples/output_henon_decrypted.png");
    let mut img = image::open(&path).unwrap();

    // Parameter builder for mapping
    let henon_params = HenonMapParametersBuilder::default()
        .build()
        .unwrap();

    let mut henon = HenonMap{parameters: henon_params};

    println!("Encrypting...");
    henon.encrypt(&img, &"examples/output_henon_encrypted.png".to_string()).
        save(&"examples/output_henon_encrypted.png".to_string());
    // Test differences between original and encrypted image.
    println!("diff <original, encrypted>: {:?} ({:?})",
             image_diff(&"examples/secret_image_lena.png".to_string(),&dest_path_encrypted), "png");

    println!("Decrypting...");
    henon.decrypt(&img, &"examples/output_henon_encrypted.png".to_string())
        .save(&"examples/output_henon_decrypted.png".to_string());
    println!("diff <original, decrypted>: {:?} ({:?})",
             image_diff(&"examples/secret_image_lena.png".to_string(), &"examples/output_henon_decrypted.png".to_string()), "png");


}

fn test_arnold(path: &Path) {}

fn main() {
    test_henon(); // do png
}