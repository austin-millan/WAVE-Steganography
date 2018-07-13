extern crate image;

mod chaotic_maps;

use chaotic_maps::*;
use std::path::Path;
use self::image::DynamicImage;
use std::fs::File;

fn main() {
    let img = image::open(&Path::new(&String::from("examples/secret_image.jpg"))).unwrap();
    let henon = ChaoticMaps::HenonMap();
    henon.push_parameter(MapParameterValue::Integer(42));
    henon.transform_image(img);
}