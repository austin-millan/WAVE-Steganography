// For dev
#![allow(dead_code)]
//#![allow(unused_imports)]
#![allow(unused_variables)]
//#![allow(unused_must_use)]
//#![allow(unused_mut)]

extern crate hound;
extern crate byteorder;
extern crate read_byte_slice;

pub mod stego {
    pub mod lsb {

        use std::fs::metadata;
        use std::vec::Vec;
        use std::io::prelude::*;
        use hound::*;
        use hound::{WavReader, WavWriter};
        use set_bit_at;
        use get_bit_at;
        use bin_to_dec;
        use std::io::BufWriter;
        use std::fs::File;
        use std::io::BufReader;
        use std::convert::*;
        use std::io::Cursor;
        use std::ops::Index;
        use read_byte_slice::{ByteSliceIter, FallibleStreamingIterator};
        use std::path::Path;
        use byteorder::{WriteBytesExt, LittleEndian};
        use to_u32;
        use byteorder::BigEndian;

        pub fn enc_payload(wav_path: &String,
                           stego_out_path: &String,
                           payload_path: &String,
                           lsb_depth: u8) {
            let mut reader = WavReader::open(&wav_path).unwrap();
            let spec = reader.spec();
            let mut writer = WavWriter::create(&stego_out_path,spec).unwrap();

            let payload = File::open(payload_path).unwrap();

            // Read 16-bit samples
            let mut samples: Vec<i16> = reader.samples()
                .map(|s| s.unwrap())
                .collect();
            // Metadata
            let (payload_len, cover_len) =
                (metadata(&payload_path).unwrap().len() as i32, metadata(&wav_path).unwrap().len() as i32);
            // let path_length = payload_path.chars().count() as i32;

            // Encode metadata about payload (payload data length)
            let mut i: i64 = 0;
            for mut sample in &samples[0..32 as usize] {
//                println!("E({}): sample: {}", i, sample);
                let bit_to_store = get_bit_at(payload_len, i as u8);
                let bit_to_replace = get_bit_at(**&sample as i32, 0); // LSB
                let sample = set_bit_at(**&sample as i32, 0, bit_to_store as u8);
                let res = writer.write_sample(sample as i16).unwrap();
                i += 1;
            }
            let mut data_iterator = ByteSliceIter::new(payload, 1 as usize);
            let mut sample_i = 0;
            while let Some(chunk) = data_iterator.next().unwrap() {
                 for byte in chunk.iter(){
                     for bit_i in 0..8{
                        let bit_to_store = get_bit_at(*byte as i32, bit_i);
                        let bit_to_replace = get_bit_at(**&samples.index(sample_i) as i32, 0); // LSB
                        let mut steg_sample = set_bit_at(**&samples.index(sample_i) as i32, 0, bit_to_store as u8);
                        if sample_i < 30 {
                            println!("E({}): sample: {}", sample_i, steg_sample);
                        }
                        writer.write_sample(steg_sample as i16).unwrap();
                        sample_i += 1;
                     }
                }
            }
            writer.finalize().unwrap();
        }

        pub fn dec_payload(stego_in_path: &String,
                           payload_out_path: &String,
                           lsb_depth: u8) -> u32 {
            // IO for reading wav files, samples, ...
            let mut reader = WavReader::open(&stego_in_path).unwrap();
            let spec = reader.spec();

            // Read 16-bit samples
            let mut samples: Vec<i16> = reader.samples()
                .map(|s| s.unwrap())
                .collect();

            // Decode length of payload
            let mut i: i64 = 0;
            let mut len_payload = [0u8; 32];
            for mut sample in &samples[0..32 as usize] {
                if get_bit_at(**&sample as i32, 0)
                    { len_payload[(31 - i as u8) as usize] = 1; } else { len_payload[(31 - i as u8) as usize] = 0; }
                i += 1;
            }
            i = 0;

            let len_payload = bin_to_dec(&len_payload);
            let mut bits = [0u8; 8];
            let mut file_buffer = File::create("examples/extracted.txt").unwrap();
             // let mut file_buffer = BufWriter::new(File::create(&Path::new("examples/extracted.txt")).unwrap());
            for mut sample in &samples[32..32+(len_payload*8) as usize] {
//                if i < 30 {
//                    println!("D({}): sample: {}", i, sample);
//                }
                if get_bit_at(**&sample as i32, 0)
                    { bits[(0+i as u8) as usize] = 1; }
                else
                    { bits[(0+i as u8) as usize] = 0; }
                if (i+1)%8==0{
                     let val = to_u32(&bits) as u8;
//                    let val = bin_to_dec(&bits) as u8;
                    println!("Byte to write (d: {:?}), (b from bin_to_dec): {:b}), (bin_to_dec: {})", bits, val, val);
                    let x= File::write_u8(&mut file_buffer, val).unwrap();
                    bits = [0u8; 8];
                    i = 0;
                }
                i += 1;
            }
            0u32
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

/// gets the bit at position `pos`. Bits are numbered from 0 (least significant) to 31 (most significant).
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

/// https://www.reddit.com/r/rust/comments/36ixl0/converting_a_vector_of_bits_to_an_integer/cred4au
pub fn to_u32(slice: &[u8]) -> u32 {
    slice.iter().fold((0,1),|(acc,mul),&bit|(acc+(mul*(1&bit as u32)),mul.wrapping_add(mul))).0
}