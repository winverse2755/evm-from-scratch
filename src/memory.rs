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
}

struct Memory {
    simple_memory: simple_memory,
}
