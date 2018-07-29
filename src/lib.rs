// For dev
#![allow(dead_code)]
//#![allow(unused_imports)]
#![allow(unused_variables)]
//#![allow(unused_must_use)]
//#![allow(unused_mut)]

extern crate hound;

pub mod stego {
    pub mod lsb {

        use std::fs::metadata;
        //use std::path::Path;
        //use std::ffi::OsStr;
        use std::vec::Vec;
        use hound::*;
        use hound::{WavReader, WavWriter};
        use set_bit_at;
        use get_bit_at;
        use bin_to_dec;

        pub fn enc_payload(wav_path: &String, stego_out_path: &String, payload_path: &String) {

        }
        pub fn dec_payload(stego_in_path: &String, payload_out_path: &String) {

        }
        pub fn enc_length(wav_path: &String, stego_out_path: &String, payload_path: &String) {
            // check_usage(wav_path, stego_out_path, payload_path);

            // IO for reading/writing wav files, samples, ....
            let mut reader = WavReader::open(&wav_path).unwrap();
            let spec = reader.spec();
            let mut writer = WavWriter::create(&stego_out_path,spec).unwrap();

            // Read 16-bit samples
            let mut samples: Vec<i16> = reader.samples().map(|s| s.unwrap()).collect();
            let (data_file_metadata, cover_file_metadata) = (metadata(&payload_path).unwrap(), metadata(&wav_path).unwrap());
            // Get length for payload and cover.
            let (secret_len, cover_len) = (data_file_metadata.len() as i32, cover_file_metadata.len() as i32);
            println!("-> Cover length: {}\n-> Secret data length: {:?}",
                     cover_len/2, data_file_metadata.len());

            // `32+(secret_len * 8)`: 4 bytes for storing payload size, + payload data.
            // `samples.len()`: total number of samples available to store 1bit/sample for LSB.
            if 32+(secret_len * 8) as i32 > samples.len() as i32{
                panic!("Secret is too large for cover audio: {} bits cannot be stored {} samples.",
                       secret_len*16, samples.len());
            }

            // Iterate over samples, store message length
            let mut i: i64 = 0;
            for mut sample in samples.iter_mut() {
                if i < 32 {
                    let bit_to_store = get_bit_at(secret_len, i as u8);
                    let bit_to_replace = get_bit_at(**&sample as i32, 0 as u8); // LSB
                    let sample = set_bit_at(**&sample as i32, 0, bit_to_store as u8);
                    let res = writer.write_sample(sample as i16).unwrap();
                }
                i += 1;
            }
            writer.finalize().unwrap();
            println!("-> Done");
        }
        pub fn dec_length(stego_in_path: &String, payload_out_path: &String) {
            // IO for reading wav files, samples, ...
            let mut reader = WavReader::open(&stego_in_path).unwrap();
            let spec = reader.spec();

            // Read 16-bit samples
            let mut samples: Vec<i16> = reader.samples().map(|s| s.unwrap()).collect();

            // Iterate over samples, retrieve message length
            let mut i: i32 = 0;
            let mut len_payload = [0u8;32];
            for mut sample in samples.iter_mut() {
                if i < 32 {
                    let bit_i = get_bit_at(**&sample as i32, 0);
                    if bit_i { len_payload[(31-i) as usize] = 1; }
                    else { len_payload[(31-i) as usize] = 0; }
                    // thread::sleep(time::Duration::from_millis(100));
                }
                else{ break; }
                i += 1;
            }
            let decimal_v = bin_to_dec(&len_payload);
            println!("-> Secret data length: {}", decimal_v);
            println!("-> Done");
        }
        pub fn display_spec(spec: WavSpec) {
            println!("SPECS:\nFormat: {:?}\nSample Rate: {:?}\n# Channels: {:?}\nBits per Sample: {:?}",
            spec.sample_format, spec.sample_rate, spec.channels, spec.bits_per_sample);
        }
    }
}

pub fn set_bit_at(mut bytes: i32, pos: u8, x: u8) -> i32 {
    bytes &= !1 << pos;
    if x.eq(&1) {
        bytes |= 1 << pos;
    }
    bytes
}

/// gets the bit at position `n`. Bits are numbered from 0 (least significant) to 31 (most significant).
pub fn get_bit_at(bytes: i32, pos: u8) -> bool {
    if pos < 32 { bytes & (1 << pos) != 0 }
    else { false }
}

/// Converts array of u8 (having values 0 or 1) to a decimal number (from little-endian).
/// It only skips over non-binary values.
pub fn bin_to_dec(arr: &[u8]) -> i32 { // @todo: make more generic
    let mut res: i32 = 0;
    for item in arr.iter() {
        if *item == 1 as u8 || *item == 0 as u8 {
            res = res * 2 + *item as i32;
        }
    }
    res
}