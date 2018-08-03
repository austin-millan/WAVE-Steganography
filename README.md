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
* [Rust](https://www.rust-lang.org/en-US/install.html)
* [Cargo](https://doc.rust-lang.org/cargo/)
* [Git](https://git-scm.com/downloads)


###  Dependencies
Dependencies are managed using Cargo. See manifest files here:
- [Cargo.toml](https://github.com/austin-millan/WAVE-Steganography/blob/master/Cargo.toml) (wave-stegranography)
- [Cargo.toml](https://github.com/austin-millan/WAVE-Steganography/blob/master/src/chaos_image_encryption/Cargo.toml) (chaos-image-encryption)

## Install / Build

This project is currently only available through Github (not [crates.io](https://crates.io/))

To download using Git:

`$ git clone https://github.com/austin-millan/WAVE-Steganography.git`

To build (and download dependencies):

`$ cargo build`

The first build will take a few minutes as it builds dependencies from
[crates.io](https://crates.io/).


## Test

Run all tests:

`$ cargo test --all`


Run `chaos_image_encryption` tests:

`$ cargo test -p chaos_image_encryption`

## Usage

**Run chaos_image_encryption
[`bin.rs`](https://github.com/austin-millan/WAVE-Steganography/blob/master/src/scramble_image/src/main.rs):**

`$ cargo run -p chaos_image_encryption`


## Contributing
Please see
[CONTRIBUTING.md](https://github.com/austin-millan/WAVE-Steganography/blob/master/CONTRIBUTING.md) for details on our code of conduct, and the process for submitting pull requests to us.

### Issues

- Image Encryption by Henon Chaotic System
    - [x] Generate henon sequence.
    - [ ] Shuffle pixels of payload.
    - [x] Mask pixels of payload.
    - [x] Recover pixels of payload.

- Steganography
    - [x] Embed payload's length in first four bytes of cover audio.
    - [x] Extract payload length from first four bytes of cover audio.
    - [x] Embed payload in remaining bytes of cover audio.
    - [x] Extract payload from remaining bytes of cover audio.

## License

Provided under MIT License by Austin Millan.

## References

* [AUDIO STEGANOGRAPHY TECHNIQUES](http://shodhganga.inflibnet.ac.in/bitstream/10603/147552/14/14_chapter%205.pdf)