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
    use image::*;
    use scramble_image::chaotic_maps::vec_to_dec;
    use scramble_image::chaotic_maps::image_diff;
    use std::path::Path;

    #[test]
    fn it_works() {
        let henon_params = HenonMapParametersBuilder::default()
            .build()
            .unwrap();

        let mut henon = HenonMap{parameters: henon_params};

        // super::scramble_image::chaotic_maps::
        // Test differences between original and encrypted image.
        let secret_path_png = Path::new(String::from("examples/secret_image_lena.png"));
        let img_png_path_decrypted = Path::new(&String::from("examples/output_henon_decrypted.png"));
        let img_png_path_encrypted = Path::new(&String::from("examples/output_henon_encrypted.png"));
        let mut img = open(Path::new(&String::from("examples/secret_image_lena.png"))).unwrap();
        henon.encrypt(&img, &img_png_path_encrypted)
            .save(img_png_path_encrypted);
        henon.decrypt(&img, &img_png_path_encrypted)
            .save(img_png_path_decrypted);
        //println!("diff <original, decrypted>: {:?} ({:?})", image_diff(&path, &dest_path_decrypted), extension);

        assert_eq!(image_diff(secret_path_png, img_png_path_decrypted), 0 as f64);


    }
}