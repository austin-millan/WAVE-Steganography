// For dev
#![allow(dead_code)]
// #![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(unused_mut)]


mod utils;

extern crate image;
extern crate scramble_image;
extern crate byteorder;

use std::fs;
use std::path::Path;

// use scramble_image::chaotic_maps::*;


fn main() {
    // Cover
    let wav_path = String::from("examples/cover_audio.wav");
    // Secrets
    let secret_text_path = String::from("examples/secret_text.txt");
    let secret_image_path = String::from("examples/secret_image_lena.png");
    // Output
    let secret_out_path = String::from("examples/stego_audio.wav");
    let extracted_image_out_path = String::from("examples/extracted_image.png");
    let extracted_text_out_path = String::from("examples/extracted_text.txt");
    // utils::encoder::lsb_enc(&wav_file, &"examples/stego_audio.wav".to_string(), &secret_text);

    // Setup output file for writing
    if Path::new(&secret_out_path).exists() {
        println!("Removing file.");
        fs::remove_file(&secret_out_path);
    }

    println!("Encoding text...");
    utils::encoder::lsb_enc(&wav_path, &secret_out_path, &secret_text_path);

    println!("Decoding text...");
    utils::decoder::lsb_dec(&secret_out_path, &extracted_text_out_path);

    // Setup output file for writing
    if Path::new(&secret_out_path).exists() {
        println!("Removing file.");
        fs::remove_file(secret_out_path);
    }
}