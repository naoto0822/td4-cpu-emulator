pub struct Rom {
    pub memories: Vec<u8>,
}

impl Rom {
    pub fn new(memories: Vec<u8>) -> Self {
        Self{
            memories,
        }
    }

    pub fn fetch(&self, target: usize) -> u8 {
        self.memories[target]
    }
}

#[cfg(test)]
mod rom_tests {
    use crate::rom::Rom;

    #[test]
    fn fetch_memory() {
        let mut memories = Vec::new();
        memories.push(0b0000);
        let rom = Rom::new(memories);
        let fetched = rom.fetch(0 as usize);

        assert_eq!(fetched, 0b0000);
    }
}
