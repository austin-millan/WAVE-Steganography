extern crate chaos_image_encryption;
extern crate steganography;
extern crate clap;


//#[cfg(not(feature = "yaml"))]
fn main() {
    use clap::App;
    use clap::Arg;
    use clap::SubCommand;
    use chaos_image_encryption::image_obfuscation::*;
    use chaos_image_encryption::image_obfuscation::HenonMapParametersBuilder;
    use steganography::stego;
    use std::path::Path;
    use std::fs;

    let matches = App::new("Wave-Steganography")
                          .version("0.1")
                          .author("Austin M. <austin.millan@gmail.com>")
                          .about("LSB Steganography on WAV Files")
                          .subcommand(SubCommand::with_name("try_me")
                                      .about("Encrypt/decrypt example image, embed/extract secret data using steganography")
                                      .version("0.1")
                                      .arg(Arg::with_name("secret")))
                          .subcommand(SubCommand::with_name("steg")
                                      .about("Embed payload in WAV file.")
                                      .version("0.1")
                                      .arg(Arg::with_name("cover")
                                           .index(1)
                                           .short("c")
                                           .long("cover")
                                           .help("Path of Wave audio file to be used in embedding payload.")
                                           .takes_value(true)
                                           .value_name("COVER_PATH")
                                           .required(true))
                                      .arg(Arg::with_name("secret")
                                           .index(2)
                                           .long("secret")
                                           .help("Path of file to encrypt and/or embed in cover Wave audio file.")
                                           .takes_value(true)
                                           .value_name("SECRET_PATH")
                                           .required(true))
                                      .arg(Arg::with_name("stego_outfile")
                                           .index(3)
                                           .short("o")
                                           .long("stego_outfile")
                                           .help("Destination path for stego audio file.")
                                           .takes_value(true)
                                           .value_name("STEGO_OUTFILE_PATH")
                                           .required(true))
                                      .arg(Arg::with_name("encrypt")
                                           .short("e")
                                           .long("encrypt")
                                           .help("The pixels of secret image are encrypted using chaotic henon sequences as a keystream.")
                                           .takes_value(true)
                                           .value_name("ENCRYPTED_OUTFILE_PATH")
                                           .required(false))
                                      .arg(Arg::with_name("verbose")
                                           .short("v")
                                           .long("verbose")
                                           .help("Sets the level of verbosity")
                                           .multiple(true)))
                          .subcommand(SubCommand::with_name("unsteg")
                                      .about("Extract payload in WAV file.")
                                      .version("0.1")
                                      .arg(Arg::with_name("stegofile")
                                           .index(1)
                                           .long("stego")
                                           .help("Path of Wave audio file to extract secret from.")
                                           .takes_value(true)
                                           .value_name("STEGO_PATH")
                                           .required(true))
                                      .arg(Arg::with_name("outfile")
                                           .index(2)
                                           .short("o")
                                           .long("outfile")
                                           .help("Destination path for extracted secret.")
                                           .takes_value(true)
                                           .value_name("OUTFILE_PATH")
                                           .required(true))
                                      .arg(Arg::with_name("decrypt")
                                           .short("d")
                                           .long("decrypt")
                                           .help("The pixels of secret image are decrypted using chaotic henon sequences as a keystream.")
                                           .takes_value(true)
                                           .value_name("DECRYPTED_OUTFILE_PATH")
                                           .required(false))
                                      .arg(Arg::with_name("verbose")
                                           .short("v")
                                           .long("verbose")
                                           .help("Sets the level of verbosity")
                                           .multiple(true)))
                          .subcommand(SubCommand::with_name("encrypt")
                                      .about("The pixels of secret image are encrypted using chaotic henon sequences as a keystream.")
                                      .version("0.1")
                                      .arg(Arg::with_name("secret")
                                           .index(1)
                                           .short("s")
                                           .long("secret")
                                           .help("Path of image file to encrypt.")
                                           .takes_value(true)
                                           .value_name("SECRET_PATH")
                                           .required(true))
                                      .arg(Arg::with_name("outfile")
                                           .index(2)
                                           .short("o")
                                           .long("outfile")
                                           .help("Destination path for encrypted image.")
                                           .takes_value(true)
                                           .value_name("OUTFILE_PATH")
                                           .required(true)))
                          .subcommand(SubCommand::with_name("decrypt")
                                      .about("Pixels of the secret image are decrypted using chaotic henon sequences as a keystream.")
                                      .version("0.1")
                                      .arg(Arg::with_name("secret")
                                           .index(1)
                                           .short("s")
                                           .long("Path of image file to decrypt.")
                                           .help("The ")
                                           .takes_value(true)
                                           .value_name("SECRET_PATH")
                                           .required(true))
                                      .arg(Arg::with_name("outfile")
                                           .index(2)
                                           .short("o")
                                           .long("outfile")
                                           .help("Destination path for decrypted image.")
                                           .takes_value(true)
                                           .value_name("OUTFILE_PATH")
                                           .required(true)))
                          .get_matches();
    // try_me example
    if let Some(_matches) = matches.subcommand_matches("try_me") {
        {   // Cleanup examples dir
            if Path::new(&"examples/lenna_encrypted.png").exists() {
                fs::remove_file(&"examples/lenna_encrypted.png").unwrap();
            }
            if Path::new(&"examples/lenna_decrypted.png").exists() {
                fs::remove_file(&"examples/lenna_decrypted.png").unwrap();
            }
            if Path::new(&"examples/stego_audio.wav").exists() {
                fs::remove_file(&"examples/stego_audio.wav").unwrap();
            }
            if Path::new(&"examples/extracted.txt").exists() {
                println!("Removing...");
                fs::remove_file(&"examples/extracted.txt").unwrap();
            }
            if Path::new(&"examples/extracted_image.png").exists() {
                println!("Removing...");
            }
        }

        {   // Image encryption
            let mut henon = HenonMap{parameters: HenonMapParametersBuilder::default()
                    .build()
                    .unwrap()};
            if let Err(e) = henon.transform(&"examples/lenna.png".to_string(), &"examples/lenna_encrypted.png".to_string()) {
                println!("Error encrypting file: {:?}", e);
            };
            if let Err(e) = henon.transform(&"examples/lenna_encrypted.png".to_string(), &"examples/lenna_decrypted.png".to_string()) {
                println!("Error encrypting file: {:?}", e);
            };
        }

        {   // Steganography with .txt file. Then with .png file.
            if let Err(e) =  stego::lsb::enc_payload(&"examples/cover_audio.wav".to_string(),
                                    &"examples/stego_audio1.wav".to_string(),
                                    &"examples/secret_text.txt".to_string(),
                                    2u8) {
                println!("Error encoding WAV file with secret data: {:?}", e);
            }
            if let Err(e) = stego::lsb::dec_payload(
                &"examples/stego_audio1.wav".to_string(),
                &"examples/extracted.txt".to_string(),
                2u8) {
                println!("Error decoding WAV file with secret data: {:?}", e);
            }
            if let Err(e) =  stego::lsb::enc_payload(&"examples/cover_audio.wav".to_string(),
                                    &"examples/stego_audio2.wav".to_string(),
                                    &"examples/lenna.png".to_string(),
                                    2u8) {
                println!("Error encoding WAV file with secret data: {:?}", e);
            }
            if let Err(e) = stego::lsb::dec_payload(
                &"examples/stego_audio2.wav".to_string(),
                &"examples/extracted_image.png".to_string(),
                2u8) {
                println!("Error decoding WAV file with secret data: {:?}", e);
            }
        }
    }

    // STEG
    if let Some(matches) = matches.subcommand_matches("steg") {
        if matches.is_present("encrypt") {
            if let Some(secret_path) = matches.value_of("secret") {
                let mut henon = HenonMap{parameters: HenonMapParametersBuilder::default()
                        .build()
                        .unwrap()};
                if let Err(e) = henon.transform(&secret_path.to_string(),
                                &matches.value_of("encrypt").unwrap().to_string()) {
                    println!("Error encrypting file: {:?}", e);
                }
            }
            if let Err(e) =  stego::lsb::enc_payload(&matches.value_of("cover").unwrap().to_string(),
                                    &matches.value_of("stego_outfile").unwrap().to_string(),
                                    &matches.value_of("encrypt").unwrap().to_string(),
                                    2u8) {
                println!("Error encoding WAV file with secret data: {:?}", e);
            }
        }
        else {
            if let Err(e) = stego::lsb::enc_payload(&matches.value_of("cover").unwrap().to_string(),
                                    &matches.value_of("stego_outfile").unwrap().to_string(),
                                    &matches.value_of("secret").unwrap().to_string(),
                                    2u8) {
                println!("Error encoding WAV file with secret data: {:?}", e);
            }
        }
    }
    // UNSTEG
    else if let Some(matches) = matches.subcommand_matches("unsteg") {
        if matches.is_present("decrypt") {
            if let Err(e) = stego::lsb::dec_payload(&matches.value_of("stegofile").unwrap().to_string(),
                                    &matches.value_of("decrypt").unwrap().to_string(),
                                    2u8) {
                println!("Error decoding WAV file with secret data: {:?}", e);
            }
            let mut henon = HenonMap{parameters: HenonMapParametersBuilder::default()
                    .build()
                    .unwrap()};
            if let Err(e) = henon.transform(&matches.value_of("decrypt").unwrap().to_string(), &matches.value_of("outfile").unwrap().to_string()) {
                println!("Error decrypting file: {:?}", e);
            }
        }
        else {
            if let Err(e) = stego::lsb::dec_payload(
                &matches.value_of("stegofile").unwrap().to_string(),
                &matches.value_of("outfile").unwrap().to_string(),
                2u8) {
                println!("Error decoding WAV file with secret data: {:?}", e);
            }
        }
    }
    // ENCRYPT ONLY
    else if let Some(matches) = matches.subcommand_matches("encrypt") {
        let outfile_path = matches.value_of("outfile").unwrap().to_string();
        if let Some(secret_path) = matches.value_of("secret") {
            let mut henon = HenonMap{parameters: HenonMapParametersBuilder::default()
                    .build()
                    .unwrap()};
            if let Err(e) = henon.transform(&secret_path.to_string(), &outfile_path.to_string()) {
                    println!("Error encrypting file: {:?}", e);
            };
        }
    }
    // DECRYPT ONLY
    else if let Some(matches) = matches.subcommand_matches("decrypt") {
        let outfile_path = matches.value_of("outfile").unwrap().to_string();
        if let Some(secret_path) = matches.value_of("secret") {
            let mut henon = HenonMap{parameters: HenonMapParametersBuilder::default()
                    .build()
                    .unwrap()};
            if let Err(e) = henon.transform(&secret_path.to_string(), &outfile_path.to_string()) {
                    println!("Error decrypting file: {:?}", e);
            }
        }
    }
}