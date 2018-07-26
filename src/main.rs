// For dev
#![allow(dead_code)]
// #![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(unused_mut)]


mod utils;

extern crate scramble_image;

use std::fs;
use std::path::Path;

// use scramble_image::chaotic_maps::*;


fn main() {
    // Cover paths
    let wav_path = String::from("examples/cover_audio.wav");
    // Payload paths
    let payload_text_in = String::from("examples/secret_text.txt");
    let payload_image_in = String::from("examples/secret_image_lena.png");
    // Output paths
    let stego_out = String::from("examples/stego_audio.wav");
    let payload_image_out = String::from("examples/extracted_image.png");
    let payload_text_out = String::from("examples/extracted_text.txt");

    // Remove file if it exists to avoid writing over old stego file
    if Path::new(&stego_out).exists() {
        println!("Removing file.");
        fs::remove_file(&stego_out);
    }

    println!("Encoding text...");
    utils::encoder::lsb_enc(&wav_path, &stego_out, &payload_text_in);

    println!("Decoding text...");
    utils::decoder::lsb_dec(&stego_out, &payload_text_out);

    // Remove file
    if Path::new(&stego_out).exists() {
        println!("Removing stego file.");
        fs::remove_file(&stego_out);
    }

    println!("Encoding image...");
    utils::encoder::lsb_enc(&wav_path, &stego_out, &payload_image_in);

    println!("Decoding image...");
    utils::decoder::lsb_dec(&stego_out, &payload_image_out);

    // Remove file
    if Path::new(&stego_out).exists() {
        println!("Removing stego file.");
        fs::remove_file(&stego_out);
    }
}