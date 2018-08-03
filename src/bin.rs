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

//    println!("Payload (text) filename length: {}", payload_text_in.chars().count());
//    println!("Payload (image) filename length: {}", payload_text_in.chars().count());
//    println!("Cover filename length: {}", wav_path.chars().count());
//    println!("Stego filename length: {}", stego_out.chars().count());
//    println!("Extracted (text) filename length: {}", payload_text_out.chars().count());


    // Remove file if it exists to avoid writing over old stego file
    if Path::new(&stego_out).exists() {
        println!("Removing file.");
        fs::remove_file(&stego_out).unwrap();
    }

    println!("Encoding text...");
    stego::lsb::enc_payload(&wav_path, &stego_out, &payload_text_in, 2u8);
    //println!("Decoding text...");
    stego::lsb::dec_payload(&stego_out, &payload_text_out, 2u8);

    // Remove file
    if Path::new(&stego_out).exists() {
        println!("Removing stego file.");
        fs::remove_file(&stego_out).unwrap();
    }

    println!("Encoding image...");
    stego::lsb::enc_payload(&wav_path, &stego_out, &payload_image_in, 1u8);

    println!("Decoding image...");
    stego::lsb::dec_payload(&stego_out, &payload_image_out, 1u8);


    // Remove file
    if Path::new(&stego_out).exists() {
        println!("Removing stego file.");
        fs::remove_file(&stego_out).unwrap();
    }
}