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
use image::open;


fn test_henon() {
    // Get input image from path
    let mut henon = HenonMap{parameters: HenonMapParametersBuilder::default()
        .build()
        .unwrap()};

    println!("Encrypting...");
    henon.encrypt(&open(&"examples/lena.png").unwrap()).save(&"examples/lena_encrypted.png".to_string());
//        println!("diff <original, encrypted>: {:?} ({:?})",
//                 image_diff(&"examples/lena.png".to_string(),
//                            &"examples/lena_encrypted.png".to_string()), "png");

    println!("Decrypting...");
    henon.decrypt(&open(&"examples/lena_encrypted.png").unwrap()).save(&"examples/lena_decrypted.png".to_string());
//
//        println!("diff <original, decrypted>: {:?} ({:?})",
//                 image_diff(&"examples/lena.png".to_string(),
//                            &"examples/lena_decrypted.png".to_string()), "png");s

    assert_eq!(image_diff(&"examples/lena.png".to_string(), &"examples/lena_decrypted.png".to_string()), 0 as f64);
}

fn test_arnold() {}

fn main() {
    test_henon();
}