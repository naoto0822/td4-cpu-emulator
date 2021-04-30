pub struct Rom {
    pub memories: Vec<u8>,
}

impl Rom {
    pub fn new(memories: Vec<u8>) -> Self {
        Self { memories }
    }

    pub fn fetch(&self, target: u8) -> u8 {
        self.memories[target as usize]
    }

    pub fn size(&self) -> u8 {
        self.memories.len() as u8
    }
}

#[cfg(test)]
mod rom_tests {
    use crate::rom::Rom;

    #[test]
    fn fetch_memory() {
        let memories: Vec<u8> = vec![0b0000];
        let rom = Rom::new(memories);
        let fetched = rom.fetch(0);

        assert_eq!(fetched, 0b0000);
    }
}
