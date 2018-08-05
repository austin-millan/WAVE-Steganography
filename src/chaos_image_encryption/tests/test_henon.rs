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
//    use image::open;
//    use chaos_image_encryption::image_diff;
//    use chaos_image_encryption::image_obfuscation::HenonMap;
//    use chaos_image_encryption::image_obfuscation::HenonMapParametersBuilder;
//
//    #[test]
//    fn encrypt_decrypt_lossless() {
//        let mut henon = HenonMap{parameters: HenonMapParametersBuilder::default()
//                .build()
//                .unwrap()};
//
//        println!("Encrypting lossless PNG...");
//        henon.transform_image(open("examples/lenna.png").unwrap().clone()).save("examples/lenna_encrypted.png".to_string());
//        println!("Decrypting lossless PNG...");
//        henon.transform_image(open(&"examples/lenna_encrypted.png").unwrap().clone()).save(&"examples/lenna_decrypted.png".to_string());
//        assert_eq!(image_diff(&"examples/lenna.png".to_string(), &"examples/lenna_decrypted.png".to_string()), 0 as f64);
//    }
//
//    #[test]
//    fn encrypt_decrypt_lossy() {
//        let mut henon = HenonMap{parameters: HenonMapParametersBuilder::default()
//                .build()
//                .unwrap()};
//        println!("Encrypting lossy JPG... (saving as PNG to avoid loss)");
//        henon.transform_image(open("examples/lenna.jpg").unwrap().clone()).save("examples/lenna_encrypted_conv.png".to_string());
//        println!("Decrypting lossy JPG... (saving as JPG to return to original format)");
//        henon.transform_image(open(&"examples/lenna_encrypted_conv.png").unwrap().clone()).save(&"examples/lenna_decrypted.jpg".to_string());
//        assert!(image_diff(&"examples/lenna.jpg".to_string(), &"examples/lenna_decrypted.jpg".to_string()) < 4.0f64);
//    }
}