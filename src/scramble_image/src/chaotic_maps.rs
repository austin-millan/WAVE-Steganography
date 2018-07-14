extern crate image;

use self::image::*;
use std::fs::File;
use std::path::Path;
use std::vec::*;


#[derive(Clone, Copy)]
pub struct ArnoldCatMap  {
    pub parameters: ArnoldCatMapParameters
}
#[derive(Clone, Copy)]
pub struct HenonMap {
    pub parameters: HenonMapParameters
}
#[derive(Clone, Copy)]
pub struct ArnoldCatMapParameters {
    pub val: i32
}
#[derive(Clone, Copy)]
pub struct HenonMapParameters {
    pub val: i32
}


#[derive(Clone, Copy)]
pub enum ChaoticMapParameters{
    ArnoldCatMapParameters{a: i32},
    HenonMapParameters{a: i32}
}
#[derive(Clone, Copy)]
pub enum ChaoticMapType{
    ArnoldCatMap(ArnoldCatMapParameters),
    HenonMap(HenonMapParameters)
}


impl ArnoldCatMap {
    pub fn transform_image(&self, mut image: &image::DynamicImage) {
        println!("transforming image (Arnold)");
    }
}

impl HenonMap {
    pub fn transform_image(&self, mut image: &image::DynamicImage) {
        println!("transforming image (Henon)");
    }
}