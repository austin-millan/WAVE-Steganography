// Copyright © 2018 Austin Millan
// [This software is released under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

extern crate steganography;

#[cfg(test)]
mod test_set_bit {
    use steganography::set_bit_at;

    #[test]
    fn test_set_bit(){
        assert_eq!(set_bit_at(8, 0, 1), 9);
        assert_eq!(set_bit_at(0, 1, 1), 2);
        assert_eq!(set_bit_at(-8, 1, 1), -6);
    }
}

#[cfg(test)]
mod test_get_bit {
    use steganography::get_bit_at;

    #[test]
    fn test_get_bit(){
        assert_eq!(get_bit_at(8, 255), false);
        assert_eq!(get_bit_at(8, 8), false);
        assert_eq!(get_bit_at(8, 3), true);
        assert_eq!(get_bit_at(8, 0), false);
        assert_eq!(get_bit_at(-1, 0), true);
    }
}

#[cfg(test)]
mod test_bin_to_dec {
    use steganography::bin_to_dec;

    #[test]
    fn test_bin_to_dec(){
        assert_eq!(bin_to_dec(&[1,0,1]), 5);
        assert_eq!(bin_to_dec(&[0,0,0]), 0);
        assert_eq!(bin_to_dec(&[5,0,0]), 0);
        assert_eq!(bin_to_dec(&[0,0,0]), 0);
    }
}