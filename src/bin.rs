// For dev
#![allow(dead_code)]
// #![allow(unused_imports)]
#![allow(unused_variables)]
//#![allow(unused_must_use)]
//#![allow(unused_mut)]

extern crate chaos_image_encryption;
extern crate steganography;

use std::fs;
use std::path::Path;
use steganography::stego;

fn main() {
    // Cover paths
    let wav_path = String::from("examples/cover_audio.wav");
    // Payload paths
    let payload_text_in = String::from("examples/secret_text.txt");
    let payload_image_in = String::from("examples/lenna.png");
    // Output paths
    let stego_out = String::from("examples/stego_audio.wav");
    let payload_image_out = String::from("examples/extracted_image.png");
    let payload_text_out = String::from("examples/extracted_text.txt");

    // Remove file if it exists to avoid writing over old stego file
    if Path::new(&stego_out).exists() {
        println!("Removing file.");
        fs::remove_file(&stego_out).unwrap();
    }
    // wav_path: &String, stego_out_path: &String, payload_path: &String
    println!("Encoding text...");
    stego::lsb::enc_length(&wav_path, &stego_out, &payload_text_in);

    println!("Decoding text...");
    stego::lsb::dec_length(&stego_out, &payload_text_out);

    // Remove file
    if Path::new(&stego_out).exists() {
        println!("Removing stego file.");
        fs::remove_file(&stego_out).unwrap();
    }

    println!("Encoding image...");
    stego::lsb::enc_length(&wav_path, &stego_out, &payload_image_in);

    println!("Decoding image...");
    stego::lsb::dec_length(&stego_out, &payload_image_out);

    // Remove file
    if Path::new(&stego_out).exists() {
        println!("Removing stego file.");
        fs::remove_file(&stego_out).unwrap();
    }
}