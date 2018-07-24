// For dev
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(unused_mut)]

// In the canonical WAV format using RIFF specification:
// Header: 1:40 bytes, length: 41:43, data = 44:EOF.

extern crate hound;
// extern crate portaudio as pa;

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
    use std::iter::FilterMap;
    use utils::hound::*;
    use utils::hound::{WavReader, WavWriter};
    use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
    // use core::result;
    // use utils::cpal::*;


    /// param in_path: Path of WAV file
    pub fn lsb_enc(cover_in_path: &String, stego_out_path: &String, data_path: &String) {
        let path = Path::new(&cover_in_path);
        if !path.exists()
            || Path::new(&cover_in_path.to_string()).extension() != Some(OsStr::new("wav")) {
            panic!("Input file must be in WAV format.");
        }

        // Get WAV spec
        let mut reader = WavReader::open(&cover_in_path).unwrap();
        let spec = reader.spec();
        let mut data_size_buf= [0u8; 4];
        println!("SPECS:\nFormat: {:?}\nSample Rate: {:?}\n# Channels: {:?}\nBits per Sample: {:?}",
                 spec.sample_format, spec.sample_rate, spec.channels, spec.bits_per_sample);

        // Get WAV samples
        let samples: Vec<i16> = reader.samples().map(|s| s.unwrap()).collect();

    }



}
