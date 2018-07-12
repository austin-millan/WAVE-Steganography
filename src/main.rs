#[allow(unused_imports)]
extern crate image;
#[allow(unused_imports)]
extern crate scramble_image;
#[allow(unused_imports)]
use std::path::Path;

fn main() {
    // Input File
    let file = String::from("examples/secret_image.jpg");
    let mut img = image::open(&Path::new(&file)).unwrap();

    // Output File
    // let fout = &mut File::create(&Path::new("examples/output.png")).unwrap();
    // im.write_to(fout, image::PNG).unwrap();

    //image_scramble::scramble(&mut img);
}