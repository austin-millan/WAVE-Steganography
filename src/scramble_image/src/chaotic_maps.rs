// For dev
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(unused_mut)]


extern crate rand;

use image::*;
use image::{GenericImage, Pixel};
use rand::{Rng};
use imageproc;
use std::fs::File;
use std::path::Path;
use std::vec::*;
use std::borrow::Borrow;
use std::ops::*;
use image::Pixels;


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
    pub a: i32
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
    pub fn transform_image(&mut self, mut img: DynamicImage, dest_path: &Path) -> DynamicImage {
        let valid = self.is_valid();  // use later when it's implemented
        let color = img.color();
        let (width, height) = img.dimensions();
        let mut noisy = img.brighten(-25);
        let mut rng = super::rand::thread_rng();

        let (width, height) = img.dimensions();
        for x in 0..(width) {
            for y in 0..(height) {
                let offset = rng.gen::<u8>();
                let px = img.get_pixel(x, y).map(|v| if v <= 255 - offset { v + offset } else { 255 });
                noisy.put_pixel(x, y, px);
            }
        }
        // let thumbnail = noisy.resize(120, 120, FilterType::Lanczos3);
        noisy
    }
    pub fn is_valid(&self) -> bool {
        // verify parameters field is of correct type
        { match self.parameters { ArnoldCatMapParameters{val: ref _a} => true } }
        // ...fill in later
    }
}


/// Henon transformation using DynamicImage
impl HenonMap {
    pub fn transform_image(&mut self, mut img: DynamicImage, dest_path: &Path) -> DynamicImage {
        let valid = self.is_valid();  // use later when it's implemented
        let color = img.color();
        let (width, height) = img.dimensions();
        let mut noisy = img.brighten(-25);
        let mut rng = rand::thread_rng();

        // let (width, height) = img.dimensions();
        for x in 0..(width) {
            for y in 0..(height) {
                let offset = rng.gen::<u8>();
                let px = img.get_pixel(x, y);
                self.parameters.a += 1;
                println!("a: {:?}", self.parameters.a);
                // let px = img.get_pixel(x, y).map(|v| if v <= 255 - offset { v + offset } else { 255 });

                // xN = y + 1 - 1.4 * x**2
                // yN = 0.3 * x



                // noisy.put_pixel(x, y, px);
            }
        }
        // let thumbnail = noisy.resize(120, 120, FilterType::Lanczos3);
        noisy
    }

    pub fn is_valid(&self) -> bool {
        // verify parameters field is of correct type
        { match self.parameters { HenonMapParameters{a: ref _a} => true } }
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

/// This function returns the amount of difference between two images as a float.
/// Based on: https://gkbrk.com/2018/01/evolving-line-art/
pub fn image_diff(l_path: &Path, r_path: &Path) -> f64 {
    imageproc::stats::root_mean_squared_error(
        &open(&l_path).unwrap(),
        &open(&r_path).unwrap())
}