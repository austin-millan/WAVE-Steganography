// Copyright © 2018 Austin Millan
// [This software is released under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

extern crate chaos_image_encryption;
extern crate image;

#[cfg(test)]
mod test_to_decimal {
    use chaos_image_encryption::vec_to_dec;

    #[test]
    fn vec_to_decimal() {
        assert_eq!(vec_to_dec(vec![0, 0, 0, 0, 0, 0, 0, 0]), 0);
        assert_eq!(vec_to_dec(vec![1, 1, 1, 0, 1, 0, 0, 1]), 233);
    }
}

#[cfg(test)]
mod test_encrypt_decrypt_henon {
    use chaos_image_encryption::image_diff;
    use chaos_image_encryption::image_obfuscation::HenonMap;
    use chaos_image_encryption::image_obfuscation::HenonMapParametersBuilder;
    use std::fs;

    #[test]
    fn encrypt_decrypt_lossless() {
        let mut henon = HenonMap{parameters: HenonMapParametersBuilder::default()
                .build()
                .unwrap()};
        match fs::create_dir_all("assets/lossless") {
            Ok(f) => f,
            Err(e) => println!("Error: {}.", e),
        };
        henon.transform(&"assets/lenna.png".to_string(), &"assets/lossless/lenna_encrypted.png".to_string()).unwrap();
        henon.transform(&"assets/lossless/lenna_encrypted.png".to_string(), &"assets/lossless/lenna_decrypted.png".to_string()).unwrap();
        assert_eq!(image_diff(&"assets/lenna.png".to_string(), &"assets/lossless/lenna_decrypted.png".to_string()), 0 as f64);
        match fs::remove_dir_all("assets/lossless") {
            Ok(f) => f,
            Err(e) => println!("Error: {}.", e),
        };
    }

    #[test]
    fn encrypt_decrypt_lossy() {
        let mut henon = HenonMap{parameters: HenonMapParametersBuilder::default()
                .build()
                .unwrap()};
        match fs::create_dir_all("assets/lossy") {
            Ok(f) => f,
            Err(e) => println!("Error: {}.", e),
        };
        henon.transform(&"assets/lenna.jpg".to_string(), &"assets/lossy/lenna_encrypted_conv.png".to_string()).unwrap();
        henon.transform(&"assets/lossy/lenna_encrypted_conv.png".to_string(), &"assets/lossy/lenna_decrypted.jpg".to_string()).unwrap();
        assert!(image_diff(&"assets/lenna.jpg".to_string(), &"assets/lossy/lenna_decrypted.jpg".to_string()) < 5.0f64);
        match fs::remove_dir_all("assets/lossy") {
            Ok(f) => f,
            Err(e) => println!("Error: {}.", e),
        };
    }
}
