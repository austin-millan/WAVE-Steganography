# WAVE-Steganography
<img src="https://cdn-images-1.medium.com/max/1400/1*dQyfOpFWmSxrmdOcQgW6OQ.jpeg" width="320" height="150">

Steganography is used to hide information. WAVE-Steganography can be used
to hide data (a "secret") inside a WAV file (a "cover"). This
technique is based on algorithms described
[here](http://shodhganga.inflibnet.ac.in/bitstream/10603/147552/14/14_chapter%205.pdf)
(5.1,5.2).


## Algorithms for Embedding
#### Hide text in audio ([5.1](http://shodhganga.inflibnet.ac.in/bitstream/10603/147552/14/14_chapter%205.pdf))
```
1. Read the text and cover audio file to get the audio samples and the
frequency.
2. Encrypt text file using [specify later].
3. Convert the encrypted text file to binary.
4. Use discrete wavelet transformation function (Haar IWT) to tranform
the cover, returning detailed coefficients CD and approximation
coefficients CA.
5. Embed the encrypted text.
    5a. Hide the message size (the first CD coefficient is replaced by
    the size).
    5b. Hide the actual message.
6. Reconstruction of stego audio signal by applying inverse DWT to
CA and modified CD.
```


#### Hide image in audio ([5.2](http://shodhganga.inflibnet.ac.in/bitstream/10603/147552/14/14_chapter%205.pdf))

```
1. Read the image and cover audio file to get the audio samples and the
frequency.
2. Encrypt image using modified Arnold transformation.
3. Convert the encrypted image to binary.
4. Use discrete wavelet transformation function (Haar IWT) to tranform
the cover, returning detailed coefficients CD and approximation
coefficients CA.
5. Convert CD to binary.
6. For each coefficient of CD:
    6a) if 2^p <= CD and CD < 2^(p+1) for some integer p
    6b) NBR <- p - OBH (original bits to be held)
7. Remaining replaceable bits of each CD are used to store the encrypted
image. The image bits starting from MSB, are stored in LSB’s of each CD.
8. Reconstruction of stego audio signal by applying inverse DWT to
CA and modified CD.
```


## Algorithms for Extracting

#### Extract text from audio ([5.1](http://shodhganga.inflibnet.ac.in/bitstream/10603/147552/14/14_chapter%205.pdf))

```
1. Read the stego audio file to get the audio samples and the frequency
2. Apply DWT to get detail and approximation coefficients, CD, and CA
respectively.
3. Get the message size from the first CD coefficient.
4. From second CD coefficient, consider as many coefficients as the size,
which represent the encrypted message characters.
5. Decrypt the message and write it into a file.
```

#### Extract image from audio ([5.2](http://shodhganga.inflibnet.ac.in/bitstream/10603/147552/14/14_chapter%205.pdf))

```
1. Read the stego audio file.
2. Apply DWT to get detail and approximation coefficients, CD, and CA
respectively.
3. Convert CD to binary.
4. OBH (original bits to be held) is set to 1.
5. For each coefficient of CD, Calculate number of bits to be replaced (NBR):
    5a) if 2^p <= CD and CD < 2^(p+1) for some integer p
    5b) NBR <- p - OBH (original bits to be held)
6. For range 1 to 16, retrieve image size
(since 16 bits are used to store size in binary):
    6a) Retrieve image size bits from LSBs of CDs, based on NBR of each CD.
7. For range 17 to image size, retrieve the image bits:
    7a) Retrieve image bits from LSB’s of remaining CDs based on NBR of each CD.
8. Convert image bits to decimal.
9. Rearrange converted image bits (in decimal) as a matrix.
10. Obtain the inverse Arnold transformation to get the embedded image
```


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

(not done)

## References

* [AUDIO STEGANOGRAPHY TECHNIQUES](http://shodhganga.inflibnet.ac.in/bitstream/10603/147552/14/14_chapter%205.pdf)
