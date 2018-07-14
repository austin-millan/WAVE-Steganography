extern crate image;
#[macro_use]
extern crate derive_builder;

mod chaotic_maps;

#[allow(unused_imports)]
use self::image::DynamicImage;
#[allow(unused_imports)]
use chaotic_maps::*;
#[allow(unused_imports)]
use std::path::Path;
#[allow(unused_imports)]
use std::fs::File;
#[allow(unused_imports)]
use std::vec::*;


fn test_henon() {
    // Use sample image
    let mut img = image::open(&Path::new(
        &String::from("examples/secret_image.jpg"))).unwrap();

    // Build parameters
    let henon_params = HenonMapParametersBuilder::default().
        build()
        .unwrap();

    // Initialize mapping algorithm with parameters
    let henon = HenonMap{parameters: henon_params};

    // Transform sample image
    let transformed = henon.transform_image( &mut img);

    transformed.save("examples/output_henon.jpg").unwrap();
}

fn test_arnold() {
    // Use sample image
    let mut img = image::open(&Path::new(
        &String::from("examples/secret_image.jpg"))).unwrap();

    // Build parameters
    let arnold_params = ArnoldCatMapParametersBuilder::default().
        build()
        .unwrap();

    // Initialize mapping algorithm with parameters
    let arnold = ArnoldCatMap{parameters: arnold_params};

    // Transform sample image
    // arnold.transform_image(&img);
}

fn main() {
    test_henon();
}