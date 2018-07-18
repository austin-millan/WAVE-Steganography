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
use std::{thread, time};


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
    //#[builder(default = "1.4")]
    #[builder(default = "1.4")]
    pub a: f64,
    #[builder(default = "0.3")]
    pub b: f64
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
                // xN = y + 1 - 1.4 * x**2
                // yN = 0.3 * x
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
        let mut noisy = img.brighten(-25);


        let mut x = 0.6 as f64;
        let mut y = 0.2 as f64;

        self.map(img);

        // let (width, height) = img.dimensions();
//        for w in 0..(width) {
//            for h in 0..(height) {
//                // let offset = rng.gen::<u8>();
//                // let w_f = w as f64;
//                // let h_f = h as f64;
//                let px = img.get_pixel(w, h);
//                // noisy.put_pixel(x, y, px);
//
//                let millis = time::Duration::from_millis(150);
//                let now = time::Instant::now();
//
//                thread::sleep(millis);
//            }
//        }
//        println!("x, y = ({:?}, {:?})",x, y);
        // let thumbnail = noisy.resize(120, 120, FilterType::Lanczos3);
        noisy
    }

    pub fn is_valid(&self) -> bool {
        // verify parameters field is of correct type
        { match self.parameters { HenonMapParameters{a: ref _a, b: ref _b} => true } }
        // ...fill in later
    }

    pub fn map(&mut self, mut img: DynamicImage) {
        let (width, height) = img.dimensions();
        let mut x = 0.6 as f64;
        let mut y = 0.2 as f64;

        let mut sequence_size = width * height * 8;
        let mut bit_sequence = Vec::new();
        let mut byte_array = Vec::new();
        // let mut TImgMatrix = Vec::new();

        for i in 0..sequence_size {
            // Henon formula
            let x_n: f64 = -(1.4 * x.powf(2.0)) + y + 1.00;
            let y_n: f64 = 0.3 * x;

            // New x and y values for next iteration of henon formula.
            x = x_n;
            y = y_n;

            let mut bit = 0;

            if x_n < 0.3992 {bit = 0 as i64}
            else {bit = 1 as i64}

            bit_sequence.push(bit);

            // Convert to decimal
            if i % 8 == 7 {
                let mut decimal_bit_sequence = to_decimal(bit_sequence.clone());
                println!("Decimal sequence: {:?} -> {:?}", bit_sequence, decimal_bit_sequence);
                bit_sequence.clear();
                byte_array.push(decimal_bit_sequence);
                println!("byte_array: {:?} -> ", byte_array);
            }

//            let byte_array_size = width*8;
//            if i % byte_array_size == byte_array_size-1 {
//                println!("ByteArray (i % byteArraySize == byteArraySize-1): {:?}", byte_array);
//                TImgMatrix.push( &mut byte_array);
//                println!("byte_array len: {:?}, values: {:?}", byte_array.len(), byte_array);
//                byte_array.clear();
//            }
//            println!("Resulting TImgMatrix: {:?}", TImgMatrix);
        }
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

pub fn to_decimal(vect: Vec<i64>) -> i64 { // &mut is a hack...
    let mut res: i64 = 0;
    for item in vect.iter() {
        // let c_item = *item as i64;
        res = res * 2 + *item as i64;
    }
    res
}