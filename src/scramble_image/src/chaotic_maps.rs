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
use std::io::prelude::*;
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
    // Classical parameters for chaotic behavior.
    #[builder(default = "1.4")]
    pub a: f64,
    #[builder(default = "0.3")]
    pub b: f64,
    // Initial values for henon map. This value works as an initial secret symmetric key
    #[builder(default = "0.1")]
    pub x: f64,
    #[builder(default = "0.1")]
    pub y: f64,
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
        a: f64,
        b: f64,
        x: f64,
        y: f64
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
    pub fn encrypt(&mut self, mut img: &DynamicImage, dest_path: &Path) -> DynamicImage {
        let (width, height) = img.dimensions();
        let henon_map = self.generate_keystream(width, height, true);
        self.transform_image(img.clone(), dest_path, henon_map)
    }

    pub fn decrypt(&mut self, mut img: &DynamicImage, dest_path: &Path) -> DynamicImage {
        let (width, height) = img.dimensions();
        let henon_map = self.generate_keystream(width, height, false);
        self.transform_image(img.clone(), dest_path, henon_map)
    }

    /// Wrapper
    pub fn transform_image(&mut self, mut img: DynamicImage, dest_path: &Path, henon_map: Vec<Vec<u8>>) -> DynamicImage {
        let valid = self.is_valid();  // use later when it's implemented
        let (width, height) = img.dimensions();
        let mut noisy = img.brighten(0);

        for w in 0..(width) {
            for h in 0..(height) {
                let mut px = img.get_pixel(w, h);
                let henon_val = henon_map.index(h as usize).index(w as usize);
                px.data[0] = px.data[0] ^ henon_val;
                px.data[1] = px.data[1] ^ henon_val;
                px.data[2] = px.data[2] ^ henon_val;
                let res = noisy.put_pixel(w, h, px);
            }
        }
        noisy
    }

    /// Generates key stream for encryption/decryption process using Henon chaotic map.
    /// param sequence_size: width*height*8
    /// param encryption: determines whether inverse algorithm is used or not.
    pub fn generate_keystream(&mut self, width: u32, height: u32, encryption: bool) -> Vec<Vec<u8>> {
        // (1) choose the initial value of (X1,Y1) for Henon map
        let mut x = self.parameters.x;
        let mut y = self.parameters.y;
        let mut x_n: f64 = 0.00;
        let mut y_n: f64 = 0.00;

        // (2) If the image size is m×n then the number of henon sequence will be 8×m×n obtained by
        // henon equation (x_n, y_n below).
        let mut sequence_size = width * height * 8;  // correct
        let mut bit_sequence = Vec::new();
        let mut byte_array = Vec::new();
        let mut t_img_matrix = Vec::new();

        /// Generate key values using Henon map.
        /// Based on: http://www.tjprc.org/publishpapers/--1382093176-2.%20Image%20encryption.full.pdf
        for i in 0..sequence_size { // println!("i: {}", i);
            // Henon formula
            if encryption == true {
                x_n = -(1.4 * x.powf(2.0)) + y + 1.00;
                y_n = 0.3 * x;
            }
            else {
                x_n = (x - y.powf(2.0) - 1.4)/0.3;
                y_n = x;
            }

            // New x and y values for next iteration of henon formula.
            x = x_n;
            y = y_n;

            // Determine bit value to be push into bit_sequence.
            // (3) cut-off point, 0.3992, has been determined so that the sequence is balanced.
            let mut bit = 0;
            if x_n < 0.3992 {bit = 0 as u8}
            else {bit = 1 as u8}
            bit_sequence.push(bit);

            // (4) Henon sequence is then reduced by combining each consecutive 8 bits into one decimal value.
            if i % 8 == 7 { // (e.g. 7%8, 15%8, 23%8, ...)
                let mut decimal_bit_sequence = to_decimal(bit_sequence.clone());
                byte_array.push(decimal_bit_sequence);
                bit_sequence.clear();
            }

            let byte_array_size = width*8;
            if (i % byte_array_size) == (byte_array_size - 1) {
                t_img_matrix.push(byte_array.clone());
                byte_array.clear();
            }
        }
        t_img_matrix
    }

    pub fn is_valid(&self) -> bool {
        // verify parameters field is of correct type
        { match self.parameters { HenonMapParameters{a: ref _a, b: ref _b, x: ref _x, y: ref_y} => true }}
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

pub fn to_decimal(vect: Vec<u8>) -> u8 { // @todo: make more generic
    let mut res: u8 = 0;
    for item in vect.iter() {
        res = res * 2 + *item as u8;
    }
    res
}

#[cfg(test)]
mod test_to_decimal {
    use super::to_decimal;
    #[test]
    fn it_works() {
        assert_eq!(to_decimal(vec![0, 0, 0, 0, 0, 0, 0, 0]), 0);
        assert_eq!(to_decimal(vec![1, 1, 1, 0, 1, 0, 0, 1]), 233);
    }
}