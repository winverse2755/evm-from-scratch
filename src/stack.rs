#[derive(Debug)]

struct Stack {
    data: Vec<u32>,
}

impl Stack {
    fn new() -> Self {
        Stack { data: Vec::new() }
    }

    fn push(&mut self, item: u32) {
        self.data.push(item);
    }
}
