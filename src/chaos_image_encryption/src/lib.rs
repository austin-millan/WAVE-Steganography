// Copyright © 2018 Austin Millan
// [This software is released under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

#[macro_use] extern crate derive_builder;
extern crate image;
extern crate imageproc;
use derive_builder::*;

pub mod image_obfuscation {
    use image::open;
    use image::{DynamicImage, GenericImage};
    use imageproc;
    use std::path::Path;
    use std::vec::*;
    use std::ops::*;
    use vec_to_dec;

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

    impl ArnoldCatMap { }
    /// Henon transformation using DynamicImage
    impl HenonMap {
        /// Wrapper
        pub fn transform(&mut self, input_path: &String, output_path: &String) -> Result<(), &'static str> {
            let mut input_img = match open(input_path) {
                Err(e) => return Err("Cannot open image."),
                Ok(f) => f,
            };
            self.transform_image(input_img.clone()).map(|i|i.save(output_path));
            Ok(())
        }

        fn transform_image(&mut self, mut img: DynamicImage) -> Result<DynamicImage, &'static str> {
            let (width, height) = img.dimensions();
            let mut noisy = img.brighten(0);
            let henon_map = self.generate_keystream(width, height);

            for w in 0..(width) {
                for h in 0..(height) {
                    let mut px = img.get_pixel(w, h);
                    let henon_val: u8 = *henon_map.index(h as usize).index(w as usize);
                    px.data[0] ^= henon_val;
                    px.data[1] ^= henon_val;
                    px.data[2] ^= henon_val;
                    let _res = noisy.put_pixel(w, h, px);
                }
            }
            Ok(noisy)
        }

        /// Generates key stream for encryption/decryption process using Henon chaotic map.
        /// param sequence_size: width*height*8
        /// param encryption: determines whether inverse algorithm is used or not.
        pub fn generate_keystream(&mut self, width: u32, height: u32) -> Vec<Vec<u8>> {
            // (1) choose the initial value of (X1,Y1) for Henon map
            let (mut x, mut y) = (self.parameters.x, self.parameters.y);
             let (mut x_n, mut y_n) = (self.parameters.x, self.parameters.y);

            // (2) If the image size is m×n then the number of henon sequence will be 8×m×n obtained by
            // henon equation (x_n, y_n below).
            let mut sequence_size = width * height * 8;  // correct

            let (mut bit_sequence, mut byte_array) = (Vec::new(), Vec::new());
            let mut t_img_matrix = Vec::new();

            // Generate key values using Henon map.
            // Based on: http://www.tjprc.org/publishpapers/--1382093176-2.%20Image%20encryption.full.pdf
            for i in 0..sequence_size { // println!("i: {}", i);
                // Henon formula
                x_n = -(self.parameters.a * x.powi(2)) + y + 1.0f64;
                y_n = self.parameters.b * x;

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
                    let mut decimal_bit_sequence = vec_to_dec(bit_sequence.clone());
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
    }
}

/// This function returns the amount of difference between two images as a float.
/// Based on: https://gkbrk.com/2018/01/evolving-line-art/
pub fn image_diff(l_path: &String, r_path: &String) -> f64 {
    imageproc::stats::root_mean_squared_error(
        &image::open(&l_path).unwrap(),
        &image::open(&r_path).unwrap())
}

pub fn vec_to_dec(vect: Vec<u8>) -> u8 {
    let mut res: u8 = 0;
    for item in vect.iter() {
        res = res * 2 + *item as u8;
    }
    res
}
