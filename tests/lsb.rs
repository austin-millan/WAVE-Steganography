extern crate steganography;
extern crate file_diff;

#[cfg(test)]
mod test_encoder_decoder {
    use steganography::stego::lsb::*;
    use file_diff::{diff_files};
    use std::fs::{File};
    use std::fs;

    #[test]
    fn test_steg_unsteg_text() {

        match fs::create_dir_all("assets/temp_text") {
            Ok(f) => f,
            Err(e) => println!("Error: {}.", e),
        };
        enc_payload(&"assets/cover_audio.wav".to_string(),
                    &"assets/temp_text/stego_audio.wav".to_string(),
                    &"assets/secret_text.txt".to_string(),
                    1u8).unwrap();
        dec_payload(&"assets/temp_text/stego_audio.wav".to_string(),
                    &"assets/temp_text/secret_text_extracted.txt".to_string(),
                    1u8).unwrap();
        let mut file1 = match File::open("assets/secret_text.txt") {
            Ok(f) => f,
            Err(e) => panic!("{}", e),
        };
        let mut file2 = match File::open("assets/temp_text/secret_text_extracted.txt") {
            Ok(f) => f,
            Err(e) => panic!("{}", e),
        };
        assert!(diff_files(&mut file1, &mut file2), true);
        match fs::remove_dir_all("assets/temp_text") {
            Ok(f) => f,
            Err(e) => println!("Error: {}.", e),
        };
    }

    #[test]
    fn test_steg_unsteg_binary() {
        match fs::create_dir_all("assets/temp_bin") {
            Ok(f) => f,
            Err(e) => println!("Error: {}.", e),
        };
        enc_payload(&"assets/cover_audio.wav".to_string(),
                    &"assets/temp_bin/stego_audio.wav".to_string(),
                    &"assets/lenna.png".to_string(),
                    1u8).unwrap();
        dec_payload(&"assets/temp_bin/stego_audio.wav".to_string(),
                    &"assets/temp_bin/lenna_extracted.png".to_string(),
                    1u8).unwrap();
        let mut file1 = match File::open("assets/lenna.png") {
            Ok(f) => f,
            Err(e) => panic!("{}", e),
        };
        let mut file2 = match File::open("assets/temp_bin/lenna_extracted.png") {
            Ok(f) => f,
            Err(e) => panic!("{}", e),
        };
        assert!(diff_files(&mut file1, &mut file2), true);
        match fs::remove_dir_all("assets/temp_bin") {
            Ok(f) => f,
            Err(e) => println!("Error: {}.", e),
        };
    }
}
