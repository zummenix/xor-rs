
#![feature(phase)]
#[phase(plugin, link)]
extern crate stainless;

pub fn xor(source: &[u8], key: &[u8]) -> Vec<u8> {
    if source.len() == 0 || key.len() == 0 {
        return source.to_vec();
    }

    let key_iter = InfiniteByteIterator::new(key);
    source.iter().zip(key_iter).map(|(&a, b)| a ^ b).collect()
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
    fn next<'a>(&'a mut self) -> Option<u8> {
        let byte = self.bytes[self.index];
        self.index = next_index(self.index, self.bytes.len());
        Some(byte)
    }
}

fn next_index(index: uint, count: uint) -> uint {
    if index + 1 < count { index + 1 } else { 0 }
}

describe! test_xor {
    it "should return right result" {
        let source = [0, 1, 2, 3];
        let key = [34, 52];
        assert!(xor(source, key).as_slice() == [34, 53, 32, 55]);
    }

    it "should return source if key is empty" {
        let source = [0, 1, 2, 3];
        let key = [];
        assert!(xor(source, key).as_slice() == [0, 1, 2, 3]);
    }

    it "should return empty result if source is empty" {
        let source = [];
        let key = [45, 32, 56];
        assert!(xor(source, key).as_slice() == []);
    }
}
