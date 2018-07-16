// For dev
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(unused_mut)]

#[macro_use]
extern crate derive_builder;

extern crate image;
extern crate scramble_image;

use std::path::Path;
use scramble_image::chaotic_maps::*;

fn main() {
    // Input File
    let file = String::from("examples/secret_image.jpg");
    let mut img = image::open(&Path::new(&file)).unwrap();

    // Parameter builder for mapping
//    let henon_param = ChaoticMapParameters::HenonMapParametersBuilder()
//        .build()
//        .unwrap();


    // Output File
    // let fout = &mut File::create(&Path::new("examples/output.png")).unwrap();
    // im.write_to(fout, image::PNG).unwrap();

    //image_scramble::scramble(&mut img);
}