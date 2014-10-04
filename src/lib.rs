
pub fn xor(source: &[u8], key: &[u8]) -> Vec<u8> {
    if source.len() == 0 || key.len() == 0 {
        return source.to_vec();
    }

    let key_iter = InfiniteByteIterator::new(key);
    source.iter().zip(key_iter).map(|(&a, b)| a ^ b).collect()
}

struct InfiniteByteIterator {
    bytes: Vec<u8>,
    index: uint
}

impl InfiniteByteIterator {
    pub fn new(bytes: &[u8]) -> InfiniteByteIterator {
        InfiniteByteIterator {
            bytes: bytes.to_vec(),
            index: 0
        }
    }
}

impl Iterator<u8> for InfiniteByteIterator {
    fn next(&mut self) -> Option<u8> {
        let byte = self.bytes[self.index];
        self.index = next_index(self.index, self.bytes.len());
        Some(byte)
    }
}

fn next_index(current: uint, count: uint) -> uint {
    let index = current + 1;
    if index < count {
        index
    } else {
        0
    }
}

#[cfg(test)]
mod test {

    use super::xor;

    #[test]
    fn test_result() {
        let source = [0, 1, 2, 3];
        let key = [34, 52];
        assert!(xor(source, key) == vec![34, 53, 32, 55]);
    }

    #[test]
    fn test_with_empty_key() {
        let source = [0, 1, 2, 3];
        let key = [];
        assert!(xor(source, key) == vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_with_empty_source() {
        let source = [];
        let key = [45, 32, 56];
        assert!(xor(source, key) == vec![]);
    }
}
