// For dev
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(unused_mut)]

// In the canonical WAV format using RIFF specification:
// Header: 1:40 bytes, length: 41:43, data = 44:EOF.


pub mod encoder {
    use std::path::Path;
    use std::ffi::OsStr;
    use std::fs::File;
    use std::io::Read;
    use std::io;
    use std::io::prelude::*;
    use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
    use std::vec::*;

    /// param in_path: Path of WAV file
        ///
    pub fn lsb_enc(cover_in_path: &String, stego_out_path: &String, data_path: &String) {
        let mut header = [0u8; 40];
        let mut data_size_buf= [0u8; 4];

        let path = Path::new(&cover_in_path);
        if !path.exists()
            || Path::new(&cover_in_path.to_string()).extension() != Some(OsStr::new("wav")) {
            panic!("Input file should be a WAV format.");
        }

        // Load WAV header and data size.
        let mut f = File::open(&cover_in_path).unwrap();
        f.read(&mut header).unwrap();
        f.read(&mut data_size_buf).unwrap();
        let mut num = &data_size_buf[..];
        let data_size = num.read_u32::<LittleEndian>().unwrap();

        // Load the data section
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer);


        println!("Data size: {:?}", data_size);
    }
    // fn lsb_dec(in_path: Path, out_path: Path, data_path: Path) { }
}