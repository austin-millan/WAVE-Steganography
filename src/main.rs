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

// use scramble_image::chaotic_maps::*;


fn main() {
    // Input File
    let wav_file = String::from("examples/cover_audio.wav");
    let secret_text = String::from("examples/secret_text.txt");

    // let pa = pa::PortAudio::new().unwrap();
    // let x = self::pa::
    utils::encoder::lsb_enc(&wav_file, &secret_text, &"examples/stego_audio.wav".to_string());
    //utils::lsb_encode();
}