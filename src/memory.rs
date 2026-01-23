struct simple_memory {
    memory: Vec<u32>,
}

impl simple_memory {
    fn new() -> Self {
        simple_memory { memory: Vec::new() }
    }

    fn access(&mut self, offset: usize, size: usize) -> &[u32] {
        let end = offset + size;
        if end > self.memory.len() {
            self.memory.resize(end, 0);
        }
        &self.memory[offset..end]
    }

    fn load(&mut self, offset: usize) -> u32 {
        if offset >= self.memory.len() {
            panic!("Memory load out of bounds");
        }
        self.access(offset, 32)[0]
    }

    fn store(&mut self, offset: usize, value: u32) {
        if offset >= self.memory.len() {
            self.memory.resize(offset + 1, 0);
        }
        self.memory[offset] = value;
    }
}

struct Memory {
    simple_memory: simple_memory,
}

impl Memory {
    fn new() -> Self {
        Memory {
            simple_memory: simple_memory::new(),
        }
    }

    fn access(&mut self, offset: usize, size: usize) -> &[u32] {
        self.simple_memory.access(offset, size)
    }

    fn load(&mut self, offset: usize) -> u32 {
        self.simple_memory.load(offset)
    }

    fn store(&mut self, offset: usize, value: u32) {
        self.simple_memory.store(offset, value);
    }
}
