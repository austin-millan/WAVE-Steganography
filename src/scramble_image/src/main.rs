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

fn test_henon(path: &Path) {
    // Get input image from path
    let img_png_path = String::from("examples/output_henon_image.png");
    let img_jpg_path = String::from("examples/output_henon_image.jpg");
    let mut img = image::open(path).unwrap();

    // Get extension, and determine how to save file based on this extension
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
            "png" => Path::new(&img_png_path),
            "jpg" => Path::new(&img_jpg_path),
            _ => panic!("Unexpected invalid token {:?}", extension)
        }
    };

    // Parameter builder for mapping
    let henon_params = HenonMapParametersBuilder::default().
        build()
        .unwrap();

    // Initialize mapping algorithm with parameters
    let henon = HenonMap{parameters: henon_params};

    // Transform sample images
    henon.transform_image( &mut img, &dest_path);

    // Test differences between input image and transformed image.
    let diff = image_diff(&path,&dest_path); // generated output
    println!("diff <input, output>: {:?} ({:?})", diff, extension);

}

fn test_arnold(path: &Path) {
    // Get input image from path
    let img_png_path = String::from("examples/output_arnold_image.png");
    let img_jpg_path = String::from("examples/output_arnold_image..jpg");
    let mut img = image::open(path).unwrap();

    // Get extension, and determine how to save file based on this extension
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
            "png" => Path::new(&img_png_path),
            "jpg" => Path::new(&img_jpg_path),
            _ => panic!("Unexpected invalid token {:?}", extension)
        }
    };

    // Parameter builder for mapping
    let henon_params = HenonMapParametersBuilder::default().
        build()
        .unwrap();

    // Initialize mapping algorithm with parameters
    let henon = HenonMap{parameters: henon_params};

    // Transform sample images
    henon.transform_image( &mut img, &dest_path);

    // Test differences between input image and transformed image.
    let diff = image_diff(&path,&dest_path); // generated output
    println!("diff <input, output>: {:?} ({:?})", diff, extension);
}

fn main() {
    // Input Image paths
    let str_path_jpg = String::from("examples/secret_image.jpg");
    let str_path_png = String::from("examples/secret_image.png");
    let secret_path_jpg = Path::new(&str_path_jpg);
    let secret_path_png = Path::new(&str_path_png);

    println!("Test Henon");
    test_henon(secret_path_jpg); // do jpg
    test_henon(secret_path_png); // do png
}