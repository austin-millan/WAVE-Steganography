
# WAVE-Steganography

[![Build Status](https://travis-ci.org/austin-millan/WAVE-Steganography.svg?branch=master)](https://travis-ci.org/austin-millan/WAVE-Steganography)

<img src="https://cdn-images-1.medium.com/max/1400/1*dQyfOpFWmSxrmdOcQgW6OQ.jpeg" width="480" height="225">


This tool can embed files inside 16-bit WAV files, writing the contents
of a file into an audio file in a way that it can be extracted later, using
this tool.

There's no graphical interface (for now), so you'll need to compile
from source and use its command line interface.

It's not entirely stable yet, a beta release should include steganography for
16-bit _and_ 24-bit WAV files, along with a completed image-encryption crate.
(More in the `Known Issues` section at the bottom).

## Steganography Technique Used

- [Hiding Data (Wiki)](https://github.com/austin-millan/WAVE-Steganography/wiki/Embedding)
- [Extracting Data (Wiki)](https://github.com/austin-millan/WAVE-Steganography/wiki/Extracting)

## Setup
### Prerequisites
The following are required:
* [Rust (>1.0)](https://www.rust-lang.org/en-US/install.html)
* [Cargo](https://doc.rust-lang.org/cargo/)
* [Git](https://git-scm.com/downloads)


### Dependencies
Dependencies are managed using Cargo. See manifest files here:
- [Cargo.toml](https://github.com/austin-millan/WAVE-Steganography/blob/master/Cargo.toml) (wave-stegranography)
- [Cargo.toml](https://github.com/austin-millan/WAVE-Steganography/blob/master/src/chaos_image_encryption/Cargo.toml) (chaos-image-encryption)

## Download / Build

Download using Git:

`$ git clone https://github.com/austin-millan/WAVE-Steganography.git; cd WAVE-Steganography/`

Build (download dependencies):

`$ cargo build`

## Test

Run all tests:

`$ cargo test --all`


## CLI Usage

**Steganography**

```
Embed payload in WAV file.
 main steg [FLAGS] <COVER_PATH> <SECRET_PATH> <STEGO_OUTFILE_PATH> [SUBCOMMAND]
Extract payload in WAV file.
 main unsteg [FLAGS] <STEGO_PATH> <OUTFILE_PATH> [SUBCOMMAND]
```

<img src="https://i.giphy.com/media/5ZZEwBseTuDOyk8Jno/giphy.webp">

**Image Encryption**
```
The pixels of secret image are encrypted using chaotic henon sequences as a keystream.
 main encrypt <SECRET_PATH> <OUTFILE_PATH>
Pixels of the secret image are decrypted using chaotic henon sequences as a keystream.
 main decrypt <SECRET_PATH> <OUTFILE_PATH>
```

<img src="https://i.giphy.com/media/551QEfda5w9rr0uYYS/giphy.webp">

## Contributing
Please see
[CONTRIBUTING.md](https://github.com/austin-millan/WAVE-Steganography/blob/master/CONTRIBUTING.md) for details on our code of conduct, and the process for submitting pull requests to us.

### Known Issues

- Image Encryption by Henon Chaotic System
    - [ ] "Confusion" isn't reached with current image encryption system.
    Only "diffusion" based on pixel values. (Which is why the module is
    called image_obfuscation).
    - [ ] The size of decrypted image does not match exactly with original image, even with lossless formats like PNG.
    - [ ] Issue with lossy format: if encrypting lossy formats (JPG),
    the output should be in a lossless format to avoid issues with decrypting an image that has lost data.

- Wave-Steganography
    - [ ] Only compatible with 16-bit WAV files.

## License

Provided under MIT License by Austin Millan.

## References

* [AUDIO STEGANOGRAPHY TECHNIQUES](http://shodhganga.inflibnet.ac.in/bitstream/10603/147552/14/14_chapter%205.pdf)

## Repository Views

[![HitCount](http://hits.dwyl.com/austin-millan/WAVE-Steganography.svg)](http://hits.dwyl.com/austin-millan/WAVE-Steganography)
