extern crate image;
#[macro_use]
extern crate derive_builder;

mod chaotic_maps;

use self::image::DynamicImage;
use chaotic_maps::*;
use std::path::Path;
use std::fs::File;
use std::vec::*;

fn main() {
    let mut img = image::open(&Path::new(&String::from("examples/secret_image.jpg"))).unwrap();

    // Building
    let henon_params = HenonMapParameters{val: 9999};
    let arnold_params = ArnoldCatMapParameters{val: 22222};
    let henon_params_builder = HenonMapParametersBuilder::default().
        build()
        .unwrap();

    // Using structs
    let henon_map = HenonMap{parameters: henon_params_builder};
    println!("DEFAULT VALUE ? {}", henon_map.parameters.val);
    let arnold_map = ArnoldCatMap{parameters: arnold_params};
    henon_map.transform_image(&img);
    arnold_map.transform_image(&img);
    // arnold_map.whoami();

    // Using manual scoping of enum
    let henon_enum = ChaoticMapType::HenonMap{parameters: henon_params};
    let arnold_enum = ChaoticMapType::ArnoldCatMap{parameters: arnold_params};
    println!("whoami: {:?}", henon_enum.whoami());
    println!("whoami: {:?}", arnold_enum.whoami());


}