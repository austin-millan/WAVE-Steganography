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

To download the source code from Git:

`$ git clone https://github.com/austin-millan/WAVE-Steganography.git`

To download dependencies, run the following from the project root directory:

`$ cargo build`


## Test

(not done)

To run all available tests, run the following from the project root directory:

`$ cargo test`

## Usage

**Examples**

(not done)

## Contributing
Please see
[CONTRIBUTING.md](https://github.com/austin-millan/WAVE-Steganography/blob/master/CONTRIBUTING.md) for details on our code of conduct, and the process for submitting pull requests to us.

## License

Provided under MIT License by Austin Millan.

## References

* [AUDIO STEGANOGRAPHY TECHNIQUES](http://shodhganga.inflibnet.ac.in/bitstream/10603/147552/14/14_chapter%205.pdf)