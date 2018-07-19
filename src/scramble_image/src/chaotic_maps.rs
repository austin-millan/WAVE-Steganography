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
    /// Wrapper
    pub fn transform_image(&mut self, mut img: DynamicImage, dest_path: &Path) -> DynamicImage {
        let valid = self.is_valid();  // use later when it's implemented
        let (width, height) = img.dimensions();
        let mut noisy = img.brighten(0);
        let henon_map = self.generate_map(&img);
        let image_matrix = self.get_image_matrix(&img);

        let mut henon_file = File::create("map_data/henon_map.txt").expect("Unable to create file");
        let mut image_file = File::create("map_data/image_file.txt").expect("Unable to create file");

        for i in 0..henon_map.len(){
            println!("Henon.index({}) {:?}, length: {}", i, henon_map.index(i), henon_map.index(i).len());
            thread::sleep(time::Duration::from_millis(200));
        }

//        for i in henon_map {
//            write!(henon_file, "{:?}", i);
//        }
//        for i in image_matrix.index(i) {
//            write!(image_file, "{:?}", i);
//        }

        // println!("Henon Map: {:?}", henon_map); // println!("Image Matrix: {:?}", image_matrix); // println!("Henon Map: {:?}", henon_map.index(1));
        //println!("Image matrix size: {:?}", image_matrix.len());
        //println!("Henon map size: {:?}", henon_map.len());

        // println!("Image matrix: {:?}", image_matrix);


        for w in 0..(width) {
            for h in 0..(height) {
                // let px = img.get_pixel(w, h);
            }
        }
        // let thumbnail = noisy.resize(120, 120, FilterType::Lanczos3);
        noisy
    }

    /// Returns a vector of vectors containing image pixel values
    pub fn get_image_matrix(&mut self, img: &DynamicImage) -> Vec<Vec<[u8; 4]>> {
        // let mut reference = img;
        let mut image_matrix = Vec::new();
        let (width, height) = img.dimensions();
        for w in 0..(width) {  // for each row
            let mut row = Vec::new();  // start with a fresh row vector.
            for h in 0..(height) {
                let px = img.get_pixel(w, h);  // get pixel
                row.push(px.data);  // push pixel data (array of u8)
            }
            image_matrix.push(row);
        }
        image_matrix
    }

    /// Generate key values using Henon map.
    /// Based on: http://www.tjprc.org/publishpapers/--1382093176-2.%20Image%20encryption.full.pdf
    pub fn generate_map(&mut self, mut img: &DynamicImage) -> Vec<Vec<u8>> {
        let (width, height) = img.dimensions();

        /// (1) choose the initial value of (X1,Y1) for Henon map
        let mut x = 0.1 as f64;
        let mut y = 0.1 as f64;

        /// (2) If the image size is m×n then the number of henon sequence will be 8×m×n obtained by
        /// henon equation (x_n, y_n below).
        let mut sequence_size = width * height * 8;  // correct
        let mut bit_sequence = Vec::new();
        let mut byte_array = Vec::new();
        let mut t_img_matrix = Vec::new();

        for i in 0..sequence_size { // println!("i: {}", i);
            // Henon formula
            let x_n: f64 = -(1.4 * x.powf(2.0)) + y + 1.00;
            let y_n: f64 = 0.3 * x;

            // New x and y values for next iteration of henon formula.
            x = x_n;
            y = y_n;

            // Determine bit value to be push into bit_sequence.
            let mut bit = 0;
            /// (3) Experimental analysis conclude that cut-off point, 0.3992, has been
            /// determined so that the sequence is balanced.
            if x_n < 0.3992 {bit = 0 as u8}
            else {bit = 1 as u8}
            bit_sequence.push(bit);

            /// (4) Henon sequence is then reduced by combining each consecutive 8 bits into one decimal value.
            if i % 8 == 7 { // (e.g. 7%8, 15%8, 23%8, ...)
                let mut decimal_bit_sequence = to_decimal(bit_sequence.clone());
                byte_array.push(decimal_bit_sequence);
                bit_sequence.clear();
            }

            let byte_array_size = width*8;
            if (i % byte_array_size) == (byte_array_size - 1) {
                t_img_matrix.push(byte_array.clone());
                byte_array.clear();
                // thread::sleep(time::Duration::from_millis(5000));
            }
        }
        t_img_matrix
    }
    pub fn is_valid(&self) -> bool {
        // verify parameters field is of correct type
        { match self.parameters { HenonMapParameters{a: ref _a, b: ref _b} => true } }
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