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
pub struct SingerMap {
    pub parameters: SingerMapParameters
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
pub struct SingerMapParameters {
    pub val: i32
}


#[derive(Clone, Copy)]
pub enum ChaoticMapParameters {
    ArnoldCatMapParameters{val: i32},
    HenonMapParameters{val: i32},
    SingerMapParameters{val: i32}
}
#[derive(Clone, Copy)]
pub enum ChaoticMapType {
    ArnoldCatMap {
        parameters: ArnoldCatMapParameters
    },
    HenonMap{
        parameters: HenonMapParameters
    },
    SingerMapParameters{
        parameters: SingerMapParameters
    }
}

impl ChaoticMapType {
    pub fn whoami(&self) -> String {
        match * self {
            ChaoticMapType::ArnoldCatMap{parameters: ref a} => format!("ArnoldCatMap"),
            ChaoticMapType::HenonMap{parameters: ref a} => format!("HenonMap"),
            ChaoticMapType::SingerMapParameters{parameters: ref a} => format!("SingerMapParameters"),
            _ => format!("NULL")
        }
    }
}

impl ArnoldCatMap {
    pub fn transform_image(&self, mut image: &image::DynamicImage) {
        println!("ArnoldCatMap::transform_image(...)");
        let res = self.is_valid();
        if self.is_valid() { println!("ArnoldCatMap is valid."); }
    }

    pub fn is_valid(&self) -> bool {
        // verify parameters field is of correct type
        { match self.parameters { ArnoldCatMapParameters{val: ref a} => true } }
        // ...fill in later
    }
}

impl HenonMap {
    pub fn transform_image(&self, mut image: &image::DynamicImage) {
        println!("HenonMap::transform_image(...)");
        let res = self.is_valid();
        if self.is_valid() { println!("HenonMap is valid."); }
    }

    pub fn is_valid(&self) -> bool {
    // verify parameters field is of correct type
    { match self.parameters { HenonMapParameters{val: ref a} => true } }
    // ...fill in later
    }
}