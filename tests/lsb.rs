extern crate steganography;

#[cfg(test)]
mod test_encoder_decoder {
    use steganography::stego::lsb::*;
    use std::fs::metadata;
//    #[test]
//    #[ignore]
//    fn test_enc_length_valid(){
//        // get length
//        let length = metadata(&String::from("examples/secret_text.txt")).unwrap().len();
//        assert_eq!(enc_length(&String::from("examples/cover_audio.wav"),
//                              &String::from("examples/stego_audio.wav"),
//                              &String::from("examples/secret_text.txt")),
//                   metadata(&String::from("examples/secret_text.txt")).unwrap().len() as u32);
//    }
//
//    #[test]
//    #[ignore]
//    fn test_dec_length_valid() {
//        let length = metadata(&String::from("examples/secret_text.txt")).unwrap().len();
//        assert_eq!(dec_length(&String::from("examples/stego_audio.wav"),
//                              &String::from("examples/extracted_text.txt")),
//                   metadata(&String::from("examples/secret_text.txt")).unwrap().len() as u32);
//    }
}