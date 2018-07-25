// For dev
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(unused_mut)]

// In the canonical WAV format using RIFF specification:
// Header: 1:40 bytes, length: 41:43, data = 44:EOF.

extern crate hound;
//extern crate sample;
extern crate bitvector;

pub mod encoder {
    use std::path::Path;
    use std::ffi::OsStr;
    use std::fs::File;
    use std::io::Read;
    use std::io::BufReader;
    use std::io::prelude::*;
    use std::vec::*;
    use std::vec::Vec;
    use std::error;
    use std::num::ParseIntError;
    use std::fmt;
    //use std::collections::BitVec;
    //use utils::bitvector::*;
    use std::iter::FilterMap;
    use utils::hound::*;
    use utils::hound::{WavReader, WavWriter};
    //use utils::sample::*;
    use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
    use std::thread;
    use std::time;
    use std::fs;
    use super::bitvector::BitVector;

    /// param in_path: Path of WAV file
    /// Stores data of file `data_path` in the LSB of every sample found in `cover_in_path`,
    /// where ach sample has type `i16`.
    pub fn lsb_enc(cover_in_path: &String, stego_out_path: &String, data_path: &String) {
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

        display_spec(spec);

        // Get all WAV samples (where to embed secret)
        let mut samples: Vec<i16> = reader.samples().map(|s| s.unwrap()).collect();
        println!("Number of samples (vec size): {:?}", samples.len());

        let data_file_metadata = fs::metadata(&data_path).unwrap();
        println!("Secret data length: {:?}", data_file_metadata.len());


        // Iterate over samples
//        for sample in samples.iter_mut() {
//            println!("Sample: {:?}", sample);
//            thread::sleep(time::Duration::from_millis(100));
//
//        }
    }

    pub fn display_spec(spec: WavSpec) {
        println!("SPECS:\nFormat: {:?}\nSample Rate: {:?}\n# Channels: {:?}\nBits per Sample: {:?}",
        spec.sample_format, spec.sample_rate, spec.channels, spec.bits_per_sample);
    }

    pub fn set_bit(mut bytes: i16, pos: u8, x: u8) -> i16{
        println!("Bytes (before): {:b}, Decimal (after): {}", bytes, bytes);
        //format!("{:b}", x)
        bytes &= !(1 << pos);
        if x.eq(&1) {
            bytes |= (1 << pos);
        }
        //println!("Bytes (after): {:?}", bytes);
        println!("Bytes (after): {:b}, Decimal (after): {}", bytes, bytes);
        bytes
    }

    /// gets the bit at position `n`. Bits are numbered from 0 (least significant) to 31 (most significant).
    pub fn get_bit_at(bytes: i16, pos: u8) -> bool {
        if pos < 16 {
            bytes & (1 << pos) != 0
        }
        else {
            false
        }
    }

    // /// NOT WORKING YET
//    pub fn iterate_over_bits(mut bytes: i16) {
//        println!("Bytes (before): {:b}", bytes);
//        //let mut bitvec = BitVector::new(30)
//        //let bv = BitVector::from_elem(10, false);
//        let bv2 = BitVector::new(8);
//        //let bv3 = BitVec::from_elem(10, false);
//        //assert_eq!(bv2.len(), 8);
//        println!("Now iterating...");
//        for x in bv2.iter() {
//            println!("x: {:?}", x);
//            //assert_eq!(x, false);
//        }
//    }
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
        assert_eq!(encoder::get_bit_at(8, 2), false);
        assert_eq!(encoder::get_bit_at(8, 1), false);
        assert_eq!(encoder::get_bit_at(8, 0), false);
        assert_eq!(encoder::get_bit_at(-1, 0), true);
    }

    #[test]
    #[should_panic]
    fn test_get_bit_should_fail(){
        assert_eq!(encoder::get_bit_at(8, 0), true);
    }
}
