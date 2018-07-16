// For dev
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(unused_mut)]

use image::*;
use rand::Rng;
use imageproc;
use std::fs::File;
use std::path::Path;
use std::vec::*;
use std::borrow::Borrow;
use std::ops::*;

pub struct ArnoldCatMap  {
    pub parameters: ArnoldCatMapParameters
}

pub struct HenonMap {
    pub parameters: HenonMapParameters
}

pub struct SingerMap {
    pub parameters: SingerMapParameters
}

#[derive(Default, Builder, Clone, Copy, Debug)]
#[builder(setter(into))]
pub struct ArnoldCatMapParameters {
    #[builder(default = "42")]
    pub val: i32
}

#[derive(Default, Builder, Clone, Copy, Debug)]
#[builder(setter(into))]
pub struct HenonMapParameters {
    #[builder(default = "42")]
    pub val: i32
}

#[derive(Default, Builder, Clone, Copy, Debug)]
#[builder(setter(into))]
pub struct SingerMapParameters {
    #[builder(default = "42")]
    pub val: i32
}

#[derive(Clone, Copy)]
pub enum ChaoticMapParameters {
    ArnoldCatMapParameters{
        val: i32
    },
    HenonMapParameters{
        val: i32
    },
    SingerMapParameters{
        val: i32
    }
}
#[derive(Clone, Copy)]
pub enum ChaoticMapType {
    ArnoldCatMap {parameters: ArnoldCatMapParameters },
    HenonMap{parameters: HenonMapParameters },
    SingerMapParameters{parameters: SingerMapParameters }
}


impl ArnoldCatMap {
    pub fn transform_image<'a>(&self, img: &'a DynamicImage) -> &'a DynamicImage{
        println!("ArnoldCatMap::transform_image(...)");
        let res = self.is_valid();
        let transformed = img.borrow();
        &transformed
    }
    pub fn is_valid(&self) -> bool {
        // verify parameters field is of correct type
        { match self.parameters { ArnoldCatMapParameters{val: ref _a} => true } }
        // ...fill in later
    }
}

impl HenonMap {
    pub fn transform_image<'a>(&self, img: &'a mut DynamicImage, dest_path: &Path) {
    //pub fn transform_image<'a>(&self, img: &'a mut image::DynamicImage) -> &'a image::DynamicImage{
        let color = img.color();
        let res = self.is_valid();
        let mut img1 = DynamicImage::as_mut_rgb8(img).unwrap();
        let (width, height) = img1.dimensions();

        println!("Original color: {:?}", color);
        // iterates over the pixels of an image, assigning random values to each pixel
        /*
        for (x, y, pixel) in img1.enumerate_pixels_mut() { // valid
            let rand_i: u8 = rand::thread_rng().gen();
            let mut pixel_copy = pixel;
            *pixel_copy = image::Rgb([rand_i;3]);
        }
        img1.save("examples/test_henon.png");
        */

        /*
        for x in 0..width { // not so valid
            for y in 0..height {
                let px = img.get_pixel(x, y).map(|v| v + 4);
                noisy.put_pixel(x, y, px);
                //println!("{:?},{:?}", x, y);
            }
        }
        */

        // Save buffer to destination file.
        img1.save(dest_path);
}
    pub fn is_valid(&self) -> bool {
        // verify parameters field is of correct type
        { match self.parameters { HenonMapParameters{val: ref _a} => true } }
        // ...fill in later
    }
}


impl ChaoticMapType {
    pub fn whoami(&self) -> String {
        match * self {
            ChaoticMapType::ArnoldCatMap{parameters: ref _a} => format!("ArnoldCatMap"),
            ChaoticMapType::HenonMap{parameters: ref _a} => format!("HenonMap"),
            ChaoticMapType::SingerMapParameters{parameters: ref _a} => format!("SingerMapParameters"),
            _ => format!("")
        }
    }
}

//This function returns the amount of difference between two images as a float.
//Based on: https://gkbrk.com/2018/01/evolving-line-art/
pub fn image_diff(l_path: &Path, r_path: &Path) -> f64 {
    imageproc::stats::root_mean_squared_error(
        &open(&l_path).unwrap(),
        &open(&r_path).unwrap())
}