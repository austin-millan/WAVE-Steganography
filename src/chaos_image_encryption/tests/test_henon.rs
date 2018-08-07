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
    use image::open;
    use chaos_image_encryption::image_diff;
    use chaos_image_encryption::image_obfuscation::HenonMap;
    use chaos_image_encryption::image_obfuscation::HenonMapParametersBuilder;

    #[test]
    fn encrypt_decrypt_lossless() {
        let mut henon = HenonMap{parameters: HenonMapParametersBuilder::default()
                .build()
                .unwrap()};
        henon.transform(&"assets/lenna.png".to_string(), &"assets/lenna_encrypted.png".to_string());
        henon.transform(&"assets/lenna_encrypted.png".to_string(), &"assets/lenna_decrypted.png".to_string());
        assert_eq!(image_diff(&"assets/lenna.png".to_string(), &"assets/lenna_decrypted.png".to_string()), 0 as f64);
    }

    #[test]
    fn encrypt_decrypt_lossy() {
        let mut henon = HenonMap{parameters: HenonMapParametersBuilder::default()
                .build()
                .unwrap()};
        henon.transform(&"assets/lenna.jpg".to_string(), &"assets/lenna_encrypted_conv.png".to_string());
        henon.transform(&"assets/lenna_encrypted_conv.png".to_string(), &"assets/lenna_decrypted.jpg".to_string());
        assert!(image_diff(&"assets/lenna.jpg".to_string(), &"assets/lenna_decrypted.jpg".to_string()) < 5.0f64);
    }
}
