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
Project dependencies are managed by Cargo, see
[Cargo.toml](https://github.com/austin-millan/WAVE-Steganography/blob/master/Cargo.toml).

## Install / Build

This project is currently only available through Github (not [crates.io](https://crates.io/))

To download using Git:

`$ git clone https://github.com/austin-millan/WAVE-Steganography.git`

To build (and download dependencies):

`$ cargo build`

The first build will take a few minutes as it fetches dependencies from
[crates.io](https://crates.io/).

```
$ time cargo build
...
real    2m11.613s
user    4m52.722s
sys     0m37.476s
```


## Test

**Note: as of 7/15 no tests exist yet.**

To run all available tests, run the following from the project root directory:

`$ cargo test`

## Usage

**Run scramble_image
[`main.rs`](https://github.com/austin-millan/WAVE-Steganography/blob/master/src/scramble_image/src/main.rs):**

`$ cargo run -p scramble_image`


## Contributing
Please see
[CONTRIBUTING.md](https://github.com/austin-millan/WAVE-Steganography/blob/master/CONTRIBUTING.md) for details on our code of conduct, and the process for submitting pull requests to us.

### Issues

- Image Encryption by Henon Chaotic System
    - [ ] Encrypt image using Henon algorithm.
        - [x] Generate henon sequence.
        - [ ] XOR-op between image and henon sequence.
        - [ ] Stretch: include image shuffling

    - [ ] Decrypt image using Henon algorithm.
        - [ ] Generate (inverse) henon sequence.
        - [ ] XOR-op between image and henon sequence.

- Steganography
    - [ ] Embed text using LSB.
    - [ ] Embed binary files using LSB.
    - [ ] Option to encrypt images using chaotic systems.
    - [ ] Option to encrypt binary/text files using standard encryption
    schemes.

## License

Provided under MIT License by Austin Millan.

## References

* [AUDIO STEGANOGRAPHY TECHNIQUES](http://shodhganga.inflibnet.ac.in/bitstream/10603/147552/14/14_chapter%205.pdf)