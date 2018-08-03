// For dev
//#![allow(dead_code)]
//#![allow(unused_imports)]
//#![allow(unused_variables)]
//#![allow(unused_must_use)]
//#![allow(unused_mut)]

extern crate hound;
extern crate read_byte_slice;

pub mod stego {
    pub mod lsb {
        use std::fs::metadata;
        use std::io::prelude::*;
        use std::fs::File;
        use std::ops::Index;
        use std::vec::Vec;
        use hound::{WavReader, WavWriter, WavSpec};
        use read_byte_slice::{ByteSliceIter, FallibleStreamingIterator};
        use set_bit_at;
        use get_bit_at;
        use bin_to_dec;

        pub fn enc_payload(wav_path: &String, stego_out_path: &String, payload_path: &String, lsb_depth: u8) {
            let mut reader = WavReader::open(&wav_path).unwrap();
            let spec = reader.spec();
            let mut writer = WavWriter::create(&stego_out_path,spec).unwrap();

            let payload = File::open(payload_path).unwrap();

            // Read 16-bit samples
            let samples: Vec<i16> = reader.samples()
                .map(|s| s.unwrap())
                .collect();
            // Metadata
            let payload_len = metadata(&wav_path).unwrap().len() as i32;

            // Encode metadata about payload (payload data length)
            let mut i: i64 = 0;
            for mut sample in &samples[0..32 as usize] {
                let bit_to_store = get_bit_at(payload_len, i as u8);
                let sample = set_bit_at(**&sample as i32, 0, bit_to_store as u8);
                writer.write_sample(sample as i16).unwrap();
                i += 1;
            }
            let mut data_iterator = ByteSliceIter::new(payload, 1 as usize);
            let mut sample_i = 0;
            while let Some(chunk) = data_iterator.next().unwrap() {
                 for byte in chunk.iter(){
                     for bit_i in 0..8{
                         let bit_to_store = get_bit_at(*byte as i32, bit_i);
                         let mut steg_sample = set_bit_at(**&samples.index(sample_i) as i32, 0, bit_to_store as u8);
                         writer.write_sample(steg_sample as i16).unwrap();
                         sample_i += 1;
                     }
                }
            }
            writer.finalize().unwrap();
        }

        pub fn dec_payload(stego_in_path: &String, payload_out_path: &String, lsb_depth: u8) {
            // IO for reading wav files, samples, ...
            let mut reader = WavReader::open(&stego_in_path).unwrap();
            let spec = reader.spec();
            let mut i = 0;
            let mut bits = [0u8; 8];
            let mut payload_vec: Vec<u8> = Vec::new();

            // Read 16-bit samples
            let samples: Vec<i16> = reader.samples()
                .map(|s| s.unwrap())
                .collect();

            // Decode length of payload
            let mut len_payload = [0u8; 32];
            for mut sample in &samples[0..32 as usize] {
                if get_bit_at(**&sample as i32, 0)
                    { len_payload[(31 - i as u8) as usize] = 1; } else { len_payload[(31 - i as u8) as usize] = 0; }
                i += 1;
            }

            let len_payload = bin_to_dec(&len_payload);
            let mut file_buffer = File::create(payload_out_path).unwrap();
            for mut sample in &samples[32..32+(len_payload*8) as usize] {
                if get_bit_at(**&sample as i32, 0)
                    { bits[(7-i as u8) as usize] = 1;}
                else
                    { bits[(7-i as u8) as usize] = 0;}
                i += 1;
                if (i% 8 == 0)  && i != 0 {
                    payload_vec.push(bin_to_dec(&bits) as u8);
                    bits = [0u8; 8];
                    i = 0;
                }
            }
            File::write_all(&mut file_buffer, &payload_vec).unwrap();
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