// For dev
#![allow(dead_code)]
//#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(unused_mut)]

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
    //use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
    //use std::thread;

    /// param in_path: Path of WAV file
    /// Stores data of file `data_path` in the LSB of every sample found in `cover_in_path`,
    /// where ach sample has type `i16`.
    pub fn lsb_enc(cover_in_path: &String, stego_out_path: &String, data_path: &String) {
        check_usage(cover_in_path, stego_out_path, data_path);

        // Get WAV spec from file
        let mut reader = WavReader::open(&cover_in_path).unwrap();
        let spec = reader.spec();

        // Setup output file for writing
        let mut writer = WavWriter::create(&stego_out_path,spec).unwrap();

        // Get all WAV samples (where to embed secret)
        let mut samples: Vec<i16> = reader.samples().map(|s| s.unwrap()).collect();
        // println!("Number of samples (vec size): {:?}", samples.len());

        let data_file_metadata = fs::metadata(&data_path).unwrap();
        let cover_file_metadata = fs::metadata(&cover_in_path).unwrap();
        println!("Secret data length: {:?} -> (as bytes): {:b}", data_file_metadata.len(), data_file_metadata.len());

        let secret_len = data_file_metadata.len() as i32;
        let cover_len = cover_file_metadata.len() as i32;


        println!("Cover length: {}", cover_len/2);

        // Verify secret can be stored in data section.
        // (secret_len * 16) = number of bits in data to be stored
        // samples.len() = total number of samples available to store 1bit/sample for LSB.
        if (secret_len * 16) as i32 > samples.len() as i32{
            panic!("Secret is too large for cover audio: {} bits cannot be stored {} samples.",
                   secret_len*16, samples.len());
        }

//        // Can be removed
//        if (secret_len * 16) as i32 <= samples.len() as i32{
//            println!("# bits to hide: {}.\n# altered samples: {} (1 bit per sample).\n\
//                      # total available samples in cover audio: {}",
//                     secret_len * 16, secret_len * 16, samples.len());
//        }

        // Iterate over samples
        let mut sample_num = 0;
        for mut sample in samples.iter_mut() {
            writer.write_sample(*&*sample).unwrap();
//            println!("Sample(#{}): D: {:?} B: {:b}, 16th position: {}",
//                     sample_num, *sample, *sample, get_bit_at(*&*sample, 15));

            //thread::sleep(time::Duration::from_millis(100));
            sample_num += 1;
        }
        writer.finalize().unwrap();
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
        if pos < 16 { bytes & (1 << pos) != 0 }
        else { false }
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
