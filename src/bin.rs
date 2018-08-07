// Copyright © 2018 Austin Millan
// [This software is released under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

extern crate chaos_image_encryption;
extern crate steganography;
extern crate clap;

//#[cfg(not(feature = "yaml"))]
fn main() {
    use clap::App;
    use clap::Arg;
    use clap::SubCommand;
    use chaos_image_encryption::image_obfuscation::*;
    use steganography::stego;

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
                                      .arg(Arg::with_name("stego_outfile")
                                           .index(3)
                                           .short("o")
                                           .long("stego_outfile")
                                           .help("Destination path for stego audio file.")
                                           .takes_value(true)
                                           .value_name("STEGO_OUTFILE_PATH")
                                           .required(true))
                                      .arg(Arg::with_name("verbose")
                                           .short("v")
                                           .long("verbose")
                                           .help("Sets the level of verbosity")
                                           .multiple(true))
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
                                                       .required(true))
                                                  .arg(Arg::with_name("a")
                                                       .short("a")
                                                       .long("a")
                                                       .help("Parameters for the henon sequence. Default values are the classical Henon map values.")
                                                       .default_value("1.4")
                                                       .takes_value(true)
                                                       .value_name("HENON_PARAMETER_A")
                                                       .required(false))
                                                  .arg(Arg::with_name("b")
                                                       .short("b")
                                                       .long("b")
                                                       .help("Parameters for the henon sequence. Default values are the classical Henon map values.")
                                                       .default_value("0.3")
                                                       .takes_value(true)
                                                       .value_name("HENON_PARAMETER_B")
                                                       .required(false))
                                                  .arg(Arg::with_name("x")
                                                       .short("x")
                                                       .long("x")
                                                       .help("Starting point for `x` value.")
                                                       .default_value("0.1")
                                                       .takes_value(true)
                                                       .value_name("HENON_PARAMETER_X")
                                                       .required(false))
                                                  .arg(Arg::with_name("y")
                                                       .short("y")
                                                       .long("y")
                                                       .help("Starting point for `x` value.")
                                                       .default_value("0.1")
                                                       .takes_value(true)
                                                       .value_name("HENON_PARAMETER_Y")
                                                       .required(false)))
                          )
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
                                      .arg(Arg::with_name("verbose")
                                           .short("v")
                                           .long("verbose")
                                           .help("Sets the level of verbosity")
                                           .multiple(true))
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
                                                       .required(true))
                                                  .arg(Arg::with_name("a")
                                                       .short("a")
                                                       .long("a")
                                                       .help("Parameters for the henon sequence. Default values are the classical Henon map values.")
                                                       .default_value("1.4")
                                                       .takes_value(true)
                                                       .value_name("HENON_PARAMETER_A")
                                                       .required(false))
                                                  .arg(Arg::with_name("b")
                                                       .short("b")
                                                       .long("b")
                                                       .help("Parameters for the henon sequence. Default values are the classical Henon map values.")
                                                       .default_value("0.3")
                                                       .takes_value(true)
                                                       .value_name("HENON_PARAMETER_B")
                                                       .required(false))
                                                  .arg(Arg::with_name("x")
                                                       .short("x")
                                                       .long("x")
                                                       .help("Starting point for `x` value.")
                                                       .default_value("0.1")
                                                       .takes_value(true)
                                                       .value_name("HENON_PARAMETER_X")
                                                       .required(false))
                                                  .arg(Arg::with_name("y")
                                                       .short("y")
                                                       .long("y")
                                                       .help("Starting point for `x` value.")
                                                       .default_value("0.1")
                                                       .takes_value(true)
                                                       .value_name("HENON_PARAMETER_Y")
                                                       .required(false))))
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
                                           .required(true))
                                      .arg(Arg::with_name("a")
                                           .short("a")
                                           .long("a")
                                           .help("Parameters for the henon sequence. Default values are the classical Henon map values.")
                                           .default_value("1.4")
                                           .takes_value(true)
                                           .value_name("HENON_PARAMETER_A")
                                           .required(false))
                                      .arg(Arg::with_name("b")
                                           .short("b")
                                           .long("b")
                                           .help("Parameters for the henon sequence. Default values are the classical Henon map values.")
                                           .default_value("0.3")
                                           .takes_value(true)
                                           .value_name("HENON_PARAMETER_B")
                                           .required(false))
                                      .arg(Arg::with_name("x")
                                           .short("x")
                                           .long("x")
                                           .help("Starting point for `x` value.")
                                           .default_value("0.1")
                                           .takes_value(true)
                                           .value_name("HENON_PARAMETER_X")
                                           .required(false))
                                      .arg(Arg::with_name("y")
                                           .short("y")
                                           .long("y")
                                           .help("Starting point for `x` value.")
                                           .default_value("0.1")
                                           .takes_value(true)
                                           .value_name("HENON_PARAMETER_Y")
                                           .required(false)))
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
                                           .required(true))
                                      .arg(Arg::with_name("a")
                                           .short("a")
                                           .long("a")
                                           .help("Parameters for the henon sequence. Default values are the classical Henon map values.")
                                           .default_value("1.4")
                                           .takes_value(true)
                                           .value_name("HENON_PARAMETER_A")
                                           .required(false))
                                      .arg(Arg::with_name("b")
                                           .short("b")
                                           .long("b")
                                           .help("Parameters for the henon sequence. Default values are the classical Henon map values.")
                                           .default_value("0.3")
                                           .takes_value(true)
                                           .value_name("HENON_PARAMETER_B")
                                           .required(false))
                                      .arg(Arg::with_name("x")
                                           .short("x")
                                           .long("x")
                                           .help("Starting point for `x` value.")
                                           .default_value("0.1")
                                           .takes_value(true)
                                           .value_name("HENON_PARAMETER_X")
                                           .required(false))
                                      .arg(Arg::with_name("y")
                                           .short("y")
                                           .long("y")
                                           .help("Starting point for `x` value.")
                                           .default_value("0.1")
                                           .takes_value(true)
                                           .value_name("HENON_PARAMETER_Y")
                                           .required(false)))
                          .get_matches();
    // STEG
    if let Some(matches) = matches.subcommand_matches("steg") {
        if matches.is_present("encrypt") {
            if let Some(secret_path) = matches.value_of("secret") {
                let henon_map = HenonMapParameters{
                    a: matches.value_of("a").unwrap().parse::<f64>().unwrap(),
                    b: matches.value_of("b").unwrap().parse::<f64>().unwrap(),
                    x: matches.value_of("x").unwrap().parse::<f64>().unwrap(),
                    y: matches.value_of("y").unwrap().parse::<f64>().unwrap()
                };
                let mut henon = HenonMap{parameters: henon_map};
                if let Err(e) = henon.transform(&secret_path.to_string(), &matches.value_of("encrypt").unwrap().to_string()) {
                        println!("Error encrypting file: {:?}", e);
                };
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
            let henon_map = HenonMapParameters{
                a: matches.value_of("a").unwrap().parse::<f64>().unwrap(),
                b: matches.value_of("b").unwrap().parse::<f64>().unwrap(),
                x: matches.value_of("x").unwrap().parse::<f64>().unwrap(),
                y: matches.value_of("y").unwrap().parse::<f64>().unwrap()
            };
            let mut henon = HenonMap{parameters: henon_map};
            if let Err(e) = henon.transform(&matches.value_of("decrypt").unwrap().to_string(), &matches.value_of("decrypt").unwrap().to_string()) {
                    println!("Error encrypting file: {:?}", e);
            };
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
            let henon_map = HenonMapParameters{
                a: matches.value_of("a").unwrap().parse::<f64>().unwrap(),
                b: matches.value_of("b").unwrap().parse::<f64>().unwrap(),
                x: matches.value_of("x").unwrap().parse::<f64>().unwrap(),
                y: matches.value_of("y").unwrap().parse::<f64>().unwrap()
            };
            let mut henon = HenonMap{parameters: henon_map};
            if let Err(e) = henon.transform(&secret_path.to_string(), &outfile_path.to_string()) {
                    println!("Error encrypting file: {:?}", e);
            };
        };
    }
    // DECRYPT ONLY
    else if let Some(matches) = matches.subcommand_matches("decrypt") {
        let outfile_path = matches.value_of("outfile").unwrap().to_string();
        if let Some(secret_path) = matches.value_of("secret") {
            let henon_map = HenonMapParameters{
                a: matches.value_of("a").unwrap().parse::<f64>().unwrap(),
                b: matches.value_of("b").unwrap().parse::<f64>().unwrap(),
                x: matches.value_of("x").unwrap().parse::<f64>().unwrap(),
                y: matches.value_of("y").unwrap().parse::<f64>().unwrap()
            };
            let mut henon = HenonMap{parameters: henon_map};
            if let Err(e) = henon.transform(&secret_path.to_string(), &outfile_path.to_string()) {
                    println!("Error encrypting file: {:?}", e);
            };
        }
    }
}