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
    let img_png_path = String::from("examples/output_henon_image.png");
    let img_jpg_path = String::from("examples/output_henon_image.jpg");
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

    let dest_path = {
        match extension {
            "png" => {
                // println!("Destination path is PNG.");
                Path::new(&img_png_path)
            },
            "jpg" => {
                // println!("Destination path is JPG.");
                Path::new(&img_jpg_path)
            },
            _ => panic!("Unexpected invalid token {:?}", extension)
        }
    };

    // Parameter builder for mapping
    let henon_params = HenonMapParametersBuilder::default()
        .build()
        .unwrap();

    // Transform sample images
    let mut henon = HenonMap{parameters: henon_params};
    // let mut henon = ChaoticMapType::HenonMap{parameters: henon_params};
    henon.transform_image(img, &dest_path).save(dest_path);
    // HenonMap{parameters: henon_params}.transform_image(img, &dest_path).save(dest_path);

    // Test differences between original and transformed image.
    println!("diff <input, output>: {:?} ({:?})", image_diff(&path,&dest_path), extension);

}

fn test_arnold(path: &Path) {
    // Get input image from path
    let img_png_path = String::from("examples/output_henon_image.png");
    let img_jpg_path = String::from("examples/output_henon_image.jpg");
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

    let dest_path = {
        match extension {
            "png" => {
                // println!("Destination path is PNG.");
                Path::new(&img_png_path)
            },
            "jpg" => {
                // println!("Destination path is JPG.");
                Path::new(&img_jpg_path)
            },
            _ => panic!("Unexpected invalid token {:?}", extension)
        }
    };

    // Parameter builder for mapping
    let arnold_params = ArnoldCatMapParametersBuilder::default()
        .build()
        .unwrap();

    // Transform sample images
    ArnoldCatMap{parameters: arnold_params}.transform_image(img, &dest_path).save(dest_path);

    // Test differences between original and transformed image.
    println!("diff <input, output>: {:?} ({:?})", image_diff(&path,&dest_path), extension);
}

fn main() {
    // Input Image paths
    // let str_path_jpg = String::from("examples/secret_image.jpg");
    let str_path_png = String::from("examples/secret_image_lena.png");
    // let secret_path_jpg = Path::new(&str_path_jpg);
    let secret_path_png = Path::new(&str_path_png);

    //println!("Test Henon (JPG)");
    //test_henon(secret_path_jpg); // do jpg
    println!("Test Henon (PNG)");
    test_henon(secret_path_png); // do png
}