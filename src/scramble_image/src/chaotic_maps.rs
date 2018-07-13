/***********************************/
extern crate image;

use self::image::*;
#[allow(unused_imports)]
use std::fs::File;
#[allow(unused_imports)]
use std::path::Path;
use std::vec::*;
/***********************************/

pub struct ArnoldCatMap  {
    pub image: image::DynamicImage,
    pub parameters: Vec<MapParameterValue>
}

pub struct HenonMap {
    pub image: image::DynamicImage,
    pub parameters: Vec<MapParameterValue>
}

#[derive(Copy, Clone)]
pub enum ChaoticMaps {
   ArnoldCatMap(),
   HenonMap()
}

pub enum MapParameterValue{
    Integer(i64),
    UInteger(u64),
    Float(f64)
}


impl ChaoticMaps {
    pub fn push_parameter(self, param: MapParameterValue) {
        println!("inside push_parameter");
    }
}


pub trait TransformImage {
    fn transform_image(&self, img: self::image::DynamicImage) -> self::image::DynamicImage;
    // fn push_parameter(self, param: MapParameterValue);
}

///// Implement the Transform trait for the Struct ArnoldCatMap.
//impl TransformImage for ArnoldCatMap <self::image::DynamicImage> {
//    fn transform_image(&self, self::DynamicImage) {
//
//    }
//    #[allow(dead_code)]
//    fn transform_image(&self, img: self::image::DynamicImage) -> self::image::DynamicImage {
//        println!("transforming image");
//        println!("Image Dimensions: {:?}", img.dimensions());
//        println!("Image Color: {:?}", img.color());
//
//
////        let (_x, _y) = img.dimensions();
////        let img_buf = img.as_mut_rgb8().unwrap();
////        let mut v: Vec<(i32, f32)> = Vec::new();
////        v.push((4, 0.4));
////        v.push((5, 0.5));
////        println!("Vector contents: {:?}", v);
//        img
//    }
//}

impl TransformImage for ChaoticMaps {
    #[allow(dead_code)]
    fn transform_image(&self, img: self::image::DynamicImage) -> self::image::DynamicImage {
        println!("transforming image");
        println!("Image Dimensions: {:?}", img.dimensions());
        println!("Image Color: {:?}", img.color());
//        let (_x, _y) = img.dimensions();
//        let img_buf = img.as_mut_rgb8().unwrap();
//        let mut v: Vec<(i32, f32)> = Vec::new();
//        v.push((4, 0.4));
//        v.push((5, 0.5));
//        println!("Vector contents: {:?}", v);
        img
    }
}