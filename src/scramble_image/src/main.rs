// For dev
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(unused_mut)]

#[macro_use] extern crate derive_builder;
extern crate image;
extern crate scramble_image;
extern crate rand;
extern crate imageproc;

pub mod chaotic_maps;

use chaotic_maps::*;
use image::*;
use std::path::*;
use std::path::Path;
use std::fs::File;
use std::vec::*;


// copy pasta
fn test_henon(path: &Path) {
    // Get input image from path
    // Where to save encrypted results
    let img_png_path_encrypted = String::from("examples/output_henon_encrypted.png");
    let img_jpg_path_encrypted = String::from("examples/output_henon_encrypted.jpg");

    // Where to save decrypted results
    let img_png_path_decrypted = String::from("examples/output_henon_decrypted.png");
    let img_jpg_path_decrypted = String::from("examples/output_henon_decrypted.jpg");
    let mut img = image::open(path).unwrap();

    // Get extension from path, and determine how to save file based on this extension.
    let extension = match path.extension() {
        None => "",
        Some(os_str) => {
            match os_str.to_str() {
                Some("png") => "png",
                Some("jpg") => "jpg",
                _ => "NOT SUPPORTED",
            }
        }
    };

    let dest_path_encrypted = {
        match extension {
            "png" => {
                // println!("Destination path is PNG.");
                Path::new(&img_png_path_encrypted)
            },
            "jpg" => {
                // println!("Destination path is JPG.");
                Path::new(&img_jpg_path_encrypted)
            },
            _ => panic!("Unexpected invalid token {:?}", extension)
        }
    };

    let dest_path_decrypted = {
        match extension {
            "png" => {
                // println!("Destination path is PNG.");
                Path::new(&img_png_path_decrypted)
            },
            "jpg" => {
                // println!("Destination path is JPG.");
                Path::new(&img_jpg_path_decrypted)
            },
            _ => panic!("Unexpected invalid token {:?}", extension)
        }
    };

    // Parameter builder for mapping
    let henon_params = HenonMapParametersBuilder::default()
        .build()
        .unwrap();

    let mut henon = HenonMap{parameters: henon_params};

    println!("Encrypting...");
    henon.encrypt(&img, &dest_path_encrypted).save(dest_path_encrypted);
    // Test differences between original and encrypted image.
    println!("diff <original, encrypted>: {:?} ({:?})", image_diff(&path,&dest_path_encrypted), extension);

    println!("Decrypting...");
    henon.decrypt(&img, &dest_path_encrypted).save(dest_path_decrypted);
    println!("diff <original, decrypted>: {:?} ({:?})", image_diff(&path, &dest_path_decrypted), extension);
}

fn test_arnold(path: &Path) {}

fn main() {
    // Input Image paths
    // let str_path_jpg = String::from("examples/secret_image.jpg");
    let str_path_png = String::from("examples/secret_image_lena.png");
    let str_path_jpg2 = String::from("examples/secret_image_lena2.jpg");
    let secret_path_png = Path::new(&str_path_png);
    let secret_path_jpg = Path::new(&str_path_jpg2);

    //println!("Test Henon (JPG)");
    //test_henon(secret_path_jpg); // do jpg
    test_henon(secret_path_png); // do png
    // test_henon(secret_path_jpg); // do jpg
}