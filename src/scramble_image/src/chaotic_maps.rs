/***********************************/
extern crate image;

#[allow(unused_imports)]
use self::image::*;
#[allow(unused_imports)]
use self::image::GenericImage;
#[allow(unused_imports)]
use self::image::DynamicImage;
#[allow(unused_imports)]
use std::fs::File;
#[allow(unused_imports)]
use std::path::Path;
#[allow(unused_imports)]
use std::vec::*;
/***********************************/


pub struct Arnold { pub name: String }
pub struct Henon { pub name: String }



pub enum ChaoticMaps {
   Arnold(String, ), // @todo: num params
   Henon {name: String}   // @todo: num params
}

impl ChaoticMaps {
    #[allow(dead_code)]
    pub fn get_name(&self) -> &'static str {
        match *self { // *self has type ChaoticMaps
            ChaoticMaps::Arnold{name: ref str} => "Arnold",
            ChaoticMaps::Henon{name: ref str}  => "Henon"
        }
    }
    #[allow(dead_code)]
    pub fn transform_image(&self, img: &mut self::image::DynamicImage) {
        println!("transforming image");
        println!("Image Dimensions: {:?}", img.dimensions());
        println!("Image Color: {:?}", img.color());
        let (_x, _y) = img.dimensions();
        let img_buf = img.as_mut_rgb8().unwrap();
        let mut v: Vec<(i32, f32)> = Vec::new();
        v.push((4, 0.4));
        v.push((5, 0.5));
        println!("Vector contents: {:?}", v);
    }
}