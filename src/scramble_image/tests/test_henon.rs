// For dev
#![allow(dead_code)]
// #![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(unused_mut)]

extern crate scramble_image;
extern crate image;

#[cfg(test)]
mod test_to_decimal {
    //use super::vec_to_dec;
    use scramble_image::chaotic_maps::vec_to_dec;

    #[test]
    fn it_works() {
        assert_eq!(vec_to_dec(vec![0, 0, 0, 0, 0, 0, 0, 0]), 0);
        assert_eq!(vec_to_dec(vec![1, 1, 1, 0, 1, 0, 0, 1]), 233);
    }
}


#[cfg(test)]
mod test_encrypt_decrypt {
    use scramble_image::chaotic_maps::*;
    use scramble_image::chaotic_maps::image_diff;
    use image::*;

    #[test]
    fn encrypt() {
        let mut henon = HenonMap{parameters: HenonMapParametersBuilder::default()
            .build()
            .unwrap()};

        println!("Encrypting...");
        henon.encrypt(&open("examples/lena.png").unwrap()).save("examples/lena_encrypted.png".to_string());
        assert_eq!(image_diff(&"examples/lena.png".to_string(), &"examples/lena_encrypted.png".to_string()), 84.6267827918708 as f64);
    }

    #[test]
    fn decrypt() {
        let mut henon = HenonMap{parameters: HenonMapParametersBuilder::default()
                .build()
                .unwrap()};
        henon.decrypt(&open(&"examples/lena_encrypted.png").unwrap()).save(&"examples/lena_decrypted.png".to_string());
        assert_eq!(image_diff(&"examples/lena.png".to_string(), &"examples/lena_decrypted.png".to_string()), 0 as f64);
    }
}