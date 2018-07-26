// For dev
#![allow(dead_code)]
//#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_must_use)]
//#![allow(unused_mut)]

// In the canonical WAV format using RIFF specification:
// Header: 1:40 bytes, length: 41:43, data = 44:EOF.

extern crate hound;

pub mod encoder {
    use std::fs;
    use std::path::Path;
    use std::ffi::OsStr;
    use std::vec::Vec;
    use utils::hound::*;
    use utils::hound::{WavReader, WavWriter};
    use utils::set_bit;
    use utils::get_bit_at;

    /// param in_path: Path of WAV file
    /// Stores data of file `data_path` in the LSB of every sample found in `cover_in_path`,
    /// where ach sample has type `i16`.
    pub fn lsb_enc(wav_path: &String, stego_out_path: &String, payload_path: &String) {
        check_usage(wav_path, stego_out_path, payload_path);

        // IO for reading/writing wav files, samples, ....
        let mut reader = WavReader::open(&wav_path).unwrap();
        let spec = reader.spec();
        let mut writer = WavWriter::create(&stego_out_path,spec).unwrap();

        // Read 16-bit samples
        let mut samples: Vec<i16> = reader.samples().map(|s| s.unwrap()).collect();
        let (data_file_metadata, cover_file_metadata) = (fs::metadata(&payload_path).unwrap(), fs::metadata(&wav_path).unwrap());
        // Get length for payload and cover.
        let (secret_len, cover_len) = (data_file_metadata.len() as i32, cover_file_metadata.len() as i32);
        println!("-> Cover length: {}\n-> Secret data length: {:?} -> (as bytes): {:b}",
                 cover_len/2, data_file_metadata.len(), data_file_metadata.len());

        // `32+(secret_len * 8)`: 4 bytes for storing payload size, + payload data.
        // `samples.len()`: total number of samples available to store 1bit/sample for LSB.
        if 32+(secret_len * 8) as i32 > samples.len() as i32{
            panic!("Secret is too large for cover audio: {} bits cannot be stored {} samples.",
                   secret_len*16, samples.len());
        }

        // Iterate over samples, store message length
        let mut i: i64 = 0;
        for mut sample in samples.iter_mut() {
            if i < 16 {
                let bit_to_store = get_bit_at(secret_len, i as u8);
                let bit_to_replace = get_bit_at(**&sample as i32, 0 as u8); // LSB
                let sample = set_bit(**&sample as i32, 0, bit_to_store as u8);
                let res = writer.write_sample(sample as i16).unwrap();
            }
            i += 1;
        }
        writer.finalize().unwrap();
    }

    pub fn display_spec(spec: WavSpec) {
        println!("SPECS:\nFormat: {:?}\nSample Rate: {:?}\n# Channels: {:?}\nBits per Sample: {:?}",
        spec.sample_format, spec.sample_rate, spec.channels, spec.bits_per_sample);
    }

    pub fn check_usage(cover_in_path: &String, stego_out_path: &String, data_path: &String) {
        let path = Path::new(&cover_in_path);
        if !path.exists()
            || Path::new(&cover_in_path.to_string()).extension() != Some(OsStr::new("wav")) {
            panic!("Cover audio must be WAV format.");
        }
        // Get WAV spec
        let mut reader = WavReader::open(&cover_in_path).unwrap();
        let spec = reader.spec();
        if spec.bits_per_sample != 16 {
            panic!("Cover audio must have 16-bit depth.");
        }
    }
}

pub mod decoder {
    use std::fs;
    use std::path::Path;
    use std::ffi::OsStr;
    use std::vec::Vec;
    use utils::hound::*;
    use utils::hound::{WavReader, WavWriter};
    use std::thread;
    use std::time;
    use std::fmt;
    use utils::set_bit;
    use utils::get_bit_at;
    use utils::to_decimal;
    //use utils::to_decimal_arr;

    pub fn lsb_dec(stego_in_path: &String, payload_out_path: &String) {
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
        let decimal_v = to_decimal(&len_payload);
        println!("Decimal: {}", decimal_v);
    }
}


pub fn set_bit(mut bytes: i32, pos: u8, x: u8) -> i32{
    //println!("Bytes (before): {:b}, Decimal (after): {}, x: {:b}", bytes, bytes, x);
    bytes &= !(1 << pos);
    if x.eq(&1) {
        bytes |= (1 << pos);
    }
    //println!("Bytes (after): {:b}, Decimal (after): {}", bytes, bytes);
    bytes
}

/// gets the bit at position `n`. Bits are numbered from 0 (least significant) to 31 (most significant).
pub fn get_bit_at(bytes: i32, pos: u8) -> bool {
    if pos < 32 { bytes & (1 << pos) != 0 }
    else { false }
}

/// arr should be 4 bytes
pub fn to_decimal(arr: &[u8]) -> i32 { // @todo: make more generic
    let mut res: i32 = 0;
    for item in arr.iter() {
        res = res * 2 + *item as i32;
    }
    res
}


#[cfg(test)]
mod test_set_bit {
    use utils::encoder;
    #[test]
    fn test_set_bit(){
        assert_eq!(encoder::set_bit(8, 0, 1), 9);
        assert_eq!(encoder::set_bit(0, 1, 1), 2);
        assert_eq!(encoder::set_bit(-8, 1, 1), -6);
    }
}

#[cfg(test)]
mod test_get_bit {
    use utils::encoder;
    #[test]
    fn test_get_bit(){
        assert_eq!(encoder::get_bit_at(8, 10000), false);
        assert_eq!(encoder::get_bit_at(8, 8), false);
        assert_eq!(encoder::get_bit_at(8, 3), true);
        assert_eq!(encoder::get_bit_at(8, 0), false);
        assert_eq!(encoder::get_bit_at(-1, 0), true);
    }
}
