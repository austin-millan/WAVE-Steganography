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
                                      .arg(Arg::with_name("outfile")
                                           .index(3)
                                           .short("o")
                                           .long("outfile")
                                           .help("Destination path for stego audio file.")
                                           .takes_value(true)
                                           .value_name("OUTFILE_PATH")
                                           .required(true))
                                      .arg(Arg::with_name("encrypt")
                                           .short("e")
                                           .long("encrypt")
                                           .help("The pixels of secret image are encrypted using chaotic henon sequences as a keystream.")
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

    // STEG
    if let Some(matches) = matches.subcommand_matches("steg") {
        let img_path = "TEMP-IMG-ENC.png".to_string();
        // STEGANOGRAPHY + USE OF IMAGE ENCRYPTION
        if matches.is_present("encrypt") {
            if let Some(secret_path) = matches.value_of("secret") {
                let mut henon = HenonMap{parameters: HenonMapParametersBuilder::default()
                        .build()
                        .unwrap()};
                henon.transform(&secret_path.to_string(), &img_path);
            }
            stego::lsb::enc_payload(&matches.value_of("cover").unwrap().to_string(),
                                    &matches.value_of("outfile").unwrap().to_string(),
                                    &img_path,
                                    2u8);
            if Path::new(&img_path).exists() {
                fs::remove_file(&img_path).unwrap();
            }
        }
        else {
            stego::lsb::enc_payload(&matches.value_of("cover").unwrap().to_string(),
                                    &matches.value_of("outfile").unwrap().to_string(),
                                    &matches.value_of("secret").unwrap().to_string(),
                                    2u8);
        }

    }
    // UNSTEG
    else if let Some(matches) = matches.subcommand_matches("unsteg") {
        let img_path = "TEMP-IMG-DEC.png".to_string();
        if matches.is_present("decrypt") {
            stego::lsb::dec_payload(&matches.value_of("stegofile").unwrap().to_string(),
                                    &img_path.to_string(),
                                    2u8);
            let mut henon = HenonMap{parameters: HenonMapParametersBuilder::default()
                    .build()
                    .unwrap()};
            henon.transform(&img_path, &matches.value_of("outfile").unwrap().to_string());
            if Path::new(&img_path).exists() {
                fs::remove_file(&img_path).unwrap();
            }
        }
        else {
            stego::lsb::dec_payload(&matches.value_of("stegofile").unwrap().to_string(),
                                    &matches.value_of("outfile").unwrap().to_string(),
                                    2u8);
        }
    }
    // ENCRYPT ONLY
    else if let Some(matches) = matches.subcommand_matches("encrypt") {
        let outfile_path = matches.value_of("outfile").unwrap().to_string();
        // STEGANOGRAPHY + IMAGE ENCRYPTION
        if let Some(secret_path) = matches.value_of("secret") {
            let mut henon = HenonMap{parameters: HenonMapParametersBuilder::default()
                    .build()
                    .unwrap()};
            henon.transform(&secret_path.to_string(), &outfile_path.to_string());
        }
    }
    // DECRYPT ONLY
    else if let Some(matches) = matches.subcommand_matches("decrypt") {
        let outfile_path = matches.value_of("outfile").unwrap().to_string();
        // STEGANOGRAPHY + IMAGE ENCRYPTION
        if let Some(secret_path) = matches.value_of("secret") {
            let mut henon = HenonMap{parameters: HenonMapParametersBuilder::default()
                    .build()
                    .unwrap()};
            henon.transform(&secret_path.to_string(), &outfile_path.to_string());
        }
    }
}