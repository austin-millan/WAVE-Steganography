// For dev
#![allow(dead_code)]
// #![allow(unused_imports)]
#![allow(unused_variables)]
//#![allow(unused_must_use)]
//#![allow(unused_mut)]


extern crate chaos_image_encryption;
extern crate steganography;
extern crate clap;



//    // Cover paths
//    let wav_path = String::from("examples/cover_audio.wav");
//    // Payload paths
//    let payload_text_in = String::from("examples/secret_text.txt");
//    let payload_image_in = String::from("examples/lenna.png");
//    // Output paths
//    let stego_out = String::from("examples/stego_audio.wav");
//    let payload_image_out = String::from("examples/extracted_image.png");
//    let payload_text_out = String::from("examples/extracted_text.txt");

//    println!("Payload (text) filename length: {}", payload_text_in.chars().count());
//    println!("Payload (image) filename length: {}", payload_text_in.chars().count());
//    println!("Cover filename length: {}", wav_path.chars().count());
//    println!("Stego filename length: {}", stego_out.chars().count());
//    println!("Extracted (text) filename length: {}", payload_text_out.chars().count());


//    // Remove file if it exists to avoid writing over old stego file
//    if Path::new(&stego_out).exists() {
//        println!("Removing file.");
//        fs::remove_file(&stego_out).unwrap();
//    }
//
//    println!("Encoding text...");
//    stego::lsb::enc_payload(&wav_path, &stego_out, &payload_text_in, 2u8);
//    //println!("Decoding text...");
//    stego::lsb::dec_payload(&stego_out, &payload_text_out, 2u8);
//
//    // Remove file
//    if Path::new(&stego_out).exists() {
//        println!("Removing stego file.");
//        fs::remove_file(&stego_out).unwrap();
//    }
//
//    println!("Encoding image...");
//    stego::lsb::enc_payload(&wav_path, &stego_out, &payload_image_in, 1u8);
//
//    println!("Decoding image...");
//    stego::lsb::dec_payload(&stego_out, &payload_image_out, 1u8);
//
//
//    // Remove file
//    if Path::new(&stego_out).exists() {
//        println!("Removing stego file.");
//        fs::remove_file(&stego_out).unwrap();
//    }

//#[cfg(not(feature = "yaml"))]
fn main() {
    use clap::App;
    use clap::Arg;
    use clap::SubCommand;
    use chaos_image_encryption::image_obfuscation::*;
    use chaos_image_encryption::image_obfuscation::HenonMapParametersBuilder;
    // As stated above, if clap is not compiled with the YAML feature, it is disabled.
    println!("YAML feature is disabled.");
    println!("Pass --features yaml to cargo when trying this example.");

    let matches = App::new("Wave-Steganography")
                          .version("0.1")
                          .author("Austin M. <austin.millan@gmail.com>")
                          .about("LSB Steganography")
                          .subcommand(SubCommand::with_name("steg")
                                      .about("Embed payload in WAV file.")
                                      .version("0.1")
                                      .arg(Arg::with_name("cover")
                                           .short("c")
                                           .long("cover")
                                           .help("Path of Wave audio file to be used in embedding payload.")
                                           .takes_value(true)
                                           .value_name("COVER_PATH")
                                           .required(true))
                                      .arg(Arg::with_name("payload")
                                           .short("p")
                                           .long("payload")
                                           .help("Path of file to embed in cover Wave audio file.")
                                           .takes_value(true)
                                           .value_name("SECRET_PATH")
                                           .required(true))
                                      .arg(Arg::with_name("outfile")
                                           .short("o")
                                           .long("outfile")
                                           .help("Destination path for stego audio file.")
                                           .takes_value(true)
                                           .value_name("DEST_PATH")
                                           .required(false))
                                      .arg(Arg::with_name("obfuscation")
                                           .short("f")
                                           .long("obfuscate")
                                           .help("The pixels of payload image are masked using chaotic henon map.")
                                           .takes_value(true)
                                           .value_name("OBFUSCATE")
                                           .required(false))
                                      .arg(Arg::with_name("encryption")
                                           .short("e")
                                           .long("encrypt")
                                           .help("<NotImplemented>Payload file is encrypted using RSA.<NotImplemented>")
                                           .takes_value(true)
                                           .value_name("ENCRYPT")
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
                                           .short("s")
                                           .long("stego")
                                           .help("Path of Wave audio file to extract payload from.")
                                           .takes_value(true)
                                           .value_name("STEGO")
                                           .required(true))
                                      .arg(Arg::with_name("outfile")
                                           .short("o")
                                           .long("outfile")
                                           .help("Destination path for extracted payload.")
                                           .takes_value(true)
                                           .value_name("OUTFILE")
                                           .required(false))
                                      .arg(Arg::with_name("obfuscation")
                                           .short("f")
                                           .long("obfuscate")
                                           .help("The pixels of payload image are reverted to their original values using chaotic henon map.")
                                           .takes_value(true)
                                           .value_name("OBFUSCATE")
                                           .required(false))
                                      .arg(Arg::with_name("encryption")
                                           .short("e")
                                           .long("encrypt")
                                           .help("<NotImplemented>Payload file is decrypted using RSA.<NotImplemented>")
                                           .takes_value(true)
                                           .value_name("ENCRYPT")
                                           .required(false))
                                      .arg(Arg::with_name("verbose")
                                           .short("v")
                                           .long("verbose")
                                           .help("Sets the level of verbosity")
                                           .multiple(true)))
                          .get_matches();

    if let Some(matches) = matches.subcommand_matches("steg") {
        if matches.is_present("obfuscation") {
            println!("Printing obfuscation info...");
            let mut henon = HenonMap{parameters: HenonMapParametersBuilder::default()
                    .build()
                    .unwrap()};
        }
         else { }
        if matches.is_present("payload") {
            println!("Printing payload info...");
        }
         else { }
        if matches.is_present("outfile") {
            println!("Printing outfile info...");
        }
         else { }
    }
    else if let Some(matches) = matches.subcommand_matches("unsteg") {
        if matches.is_present("cover") {
            println!("Printing cover info...");
        }
         else { }
        if matches.is_present("outfile") {
            println!("Printing outfile info...");
        }
         else { }
    }
}