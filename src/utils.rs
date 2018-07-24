// For dev
#![allow(dead_code)]
// #![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(unused_mut)]

// In the canonical WAV format using RIFF specification:
// Header: 1:40 bytes, length: 41:43, data = 44:EOF.

extern crate hound;
extern crate sample;
extern crate portaudio as pa;


pub mod encoder {
    use std::path::Path;
    use std::ffi::OsStr;
    use std::fs::File;
    use std::io::Read;
    use std::io::BufReader;
    use std::io::prelude::*;
    use std::vec::*;
    use std::vec::Vec;
    use std::iter::FilterMap;
    use utils::hound::*;
    use utils::sample::*;
    use utils::sample::{signal, Signal, ToFrameSliceMut};
    use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
    use super::sample::signal::UntilExhausted;
    use super::sample::signal::FromInterleavedSamplesIterator;

    /// param in_path: Path of WAV file
    pub fn lsb_enc(cover_in_path: &String, stego_out_path: &String, data_path: &String) {
        let path = Path::new(&cover_in_path);
        if !path.exists()
            || Path::new(&cover_in_path.to_string()).extension() != Some(OsStr::new("wav")) {
            panic!("Input file must be in WAV format.");
        }

        let mut header = [0u8; 40];
        let mut data_size_buf= [0u8; 4];

        // Load WAV header and data size.
        let reader = WavReader::open(&cover_in_path).unwrap();

        // Get the wav spec and create a target with the new desired sample rate.
        let port_audio = super::pa::PortAudio::new().unwrap();
        let spec = reader.spec();

        // Read the interleaved samples and convert them to a signal.
        let samples = reader.into_samples::<i16>().filter_map(Result::ok);
//        for sample in samples. {
//            println!("Samples: {:?}", samples);  // fails
//        }




        println!("Bits per sample: {:?}", spec.bits_per_sample);
        println!("Channels: {:?}", spec.channels);
        println!("Sample Rate: {:?}", spec.sample_rate);
        println!("Sample Format: {:?}", spec.sample_format);

        let mut f = File::open(&cover_in_path).unwrap();
        f.read(&mut header).unwrap();
        f.read(&mut data_size_buf).unwrap();
        let mut num = &data_size_buf[..];
        let data_size = num.read_u32::<LittleEndian>().unwrap();
        println!("Data size: {:?}", data_size);

        // Load the data section
        let mut buffer: Vec<i16> = Vec::new();
    }
}
