// For dev
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(unused_mut)]


mod utils;

#[macro_use]
extern crate derive_builder;

extern crate image;
extern crate scramble_image;
extern crate rand;
extern crate byteorder;

use std::path::Path;
use scramble_image::chaotic_maps::*;


fn main() {
    // Input File
    let wav_file = String::from("examples/cover_audio.wav");
    let secret_text = String::from("examples/secret_text.txt");


    utils::encoder::lsb_enc(&wav_file, &secret_text, &"examples/stego_audio.wav".to_string());
    //utils::lsb_encode();
}