
#![feature(phase)]

pub fn xor(source: &[u8], key: &[u8]) -> Vec<u8> {
    match key.len() {
        0 => source.to_vec(),
        1 => xor_with_byte(source, key[0]),
        _ => {
            let key_iter = InfiniteByteIterator::new(key);
            source.iter().zip(key_iter).map(|(&a, b)| a ^ b).collect()
        },
    }
}

pub fn xor_with_byte(source: &[u8], byte: u8) -> Vec<u8> {
    source.iter().map(|&a| a ^ byte).collect()
}

struct InfiniteByteIterator<'a> {
    bytes: &'a [u8],
    index: uint
}

impl<'a> InfiniteByteIterator<'a> {
    pub fn new(bytes: &'a [u8]) -> InfiniteByteIterator<'a> {
        InfiniteByteIterator {
            bytes: bytes,
            index: 0
        }
    }
}

impl<'a> Iterator<u8> for InfiniteByteIterator<'a> {
    fn next(&mut self) -> Option<u8> {
        let byte = self.bytes[self.index];
        self.index = next_index(self.index, self.bytes.len());
        Some(byte)
    }
}

fn next_index(index: uint, count: uint) -> uint {
    if index + 1 < count { index + 1 } else { 0 }
}

#[cfg(test)]
mod test {
    #[phase(plugin)] extern crate stainless;

    pub use super::{
        xor,
        xor_with_byte,
    };

    describe! xor {
        it "should return valid result" {
            let source = &[0, 1, 2, 3];
            let key = &[34, 52];
            assert!(xor(source, key) == [34, 53, 32, 55]);
        }

        it "should return valid result with one byte xor" {
            let source = &[0, 1, 2, 3];
            let key = &[47];
            assert!(xor(source, key) == [47, 46, 45, 44]);
        }

        it "should return source if key is empty" {
            let source = &[0, 1, 2, 3];
            let key = &[];
            assert!(xor(source, key) == [0, 1, 2, 3]);
        }

        it "should return empty result if source is empty" {
            let source = &[];
            let key = &[45, 32, 56];
            assert!(xor(source, key) == []);
        }
    }

    describe! xor_with_byte {
        it "should return valid result" {
            let source = &[0, 1, 2, 3];
            assert!(xor_with_byte(source, 23) == [23, 22, 21, 20]);
        }
    }
}
