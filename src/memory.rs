struct simple_memory {
    data: Vec<u32>,
}

impl simple_memory {
    fn new() -> Self {
        simple_memory { data: Vec::new() }
    }

    fn access(&mut self, offset: usize, size: usize) -> &[u32] {
        let end = offset + size;
        if end > self.data.len() {
            self.data.resize(end, 0);
        }
        &self.data[offset..end]
    }
}

struct Memory {
    simple_memory: simple_memory,
}
