pub trait MemoryBus {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, value: u8);
}

pub struct FlatMemory {
    data: Vec<u8>,
}

impl FlatMemory {
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0; size],
        }
    }

    pub fn with_capacity(size: usize) -> Self {
        let mut data = Vec::with_capacity(size);
        data.resize(size, 0);
        Self { data }
    }

    pub fn load(&mut self, address: u16, bytes: &[u8]) {
        for (i, &b) in bytes.iter().enumerate() {
            let addr = (address as usize).wrapping_add(i);
            if addr < self.data.len() {
                self.data[addr] = b;
            }
        }
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }
}

impl MemoryBus for FlatMemory {
    fn read(&self, address: u16) -> u8 {
        self.data.get(address as usize).copied().unwrap_or(0)
    }

    fn write(&mut self, address: u16, value: u8) {
        if let Some(byte) = self.data.get_mut(address as usize) {
            *byte = value;
        }
    }
}
