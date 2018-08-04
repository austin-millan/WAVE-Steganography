# WAVE-Steganography
<img src="https://cdn-images-1.medium.com/max/1400/1*dQyfOpFWmSxrmdOcQgW6OQ.jpeg" width="480" height="225">

**Note: The software is still under active development and not
feature complete or ready for consumption by anyone other than
software developers.**

Steganography is used to hide information. WAVE-Steganography can be used
to hide data (a "secret") inside a WAV file (a "cover"). This
technique is based on algorithms described
[here](http://shodhganga.inflibnet.ac.in/bitstream/10603/147552/14/14_chapter%205.pdf)
(5.1,5.2).


## Steganography Technique Used

- [Hiding Data (Wiki)](https://github.com/austin-millan/WAVE-Steganography/wiki/Embedding)
- [Extracting Data (Wiki)](https://github.com/austin-millan/WAVE-Steganography/wiki/Extracting)


## Setup
### Prerequisites
The following are required:
* [Rust (>1.0)](https://www.rust-lang.org/en-US/install.html)
* [Cargo](https://doc.rust-lang.org/cargo/)
* [Git](https://git-scm.com/downloads)


###  Dependencies
Dependencies are managed using Cargo. See manifest files here:
- [Cargo.toml](https://github.com/austin-millan/WAVE-Steganography/blob/master/Cargo.toml) (wave-stegranography)
- [Cargo.toml](https://github.com/austin-millan/WAVE-Steganography/blob/master/src/chaos_image_encryption/Cargo.toml) (chaos-image-encryption)

## Download / Build

Download using Git:

`$ git clone https://github.com/austin-millan/WAVE-Steganography.git; cd WAVE-Steganography/`

Build (download dependencies):

`$ cargo build`

The first build will take a few minutes as it builds dependencies from
[crates.io](https://crates.io/).


## Test

Run all tests:

`$ cargo test --all`


Run `chaos_image_encryption` tests:

`$ cargo test -p chaos_image_encryption`

## Usage

Steganography

```
USAGE:
    main steg [FLAGS] <COVER_PATH> <SECRET_PATH> <OUTFILE_PATH>

    main unsteg [FLAGS] <STEGO_PATH> <OUTFILE_PATH>
```

Image Encryption
```
USAGE:
    main encrypt <SECRET_PATH> <OUTFILE_PATH>

    main decrypt <SECRET_PATH> <OUTFILE_PATH>
```


## Contributing
Please see
[CONTRIBUTING.md](https://github.com/austin-millan/WAVE-Steganography/blob/master/CONTRIBUTING.md) for details on our code of conduct, and the process for submitting pull requests to us.

### Known Issues

- General
    - [ ] Still requires additional testing.

- Image Encryption by Henon Chaotic System
    - [ ] "Confusion" isn't reached with current image encryption system. Only "diffusion" based on pixel values.
    - [ ] The size of decrypted image does not match exactly with original image, even with lossless formats like PNG.

## License

Provided under MIT License by Austin Millan.

## References

* [AUDIO STEGANOGRAPHY TECHNIQUES](http://shodhganga.inflibnet.ac.in/bitstream/10603/147552/14/14_chapter%205.pdf)