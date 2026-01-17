struct Stack {
    data: Vec<u32>,
}

impl Stack {
    fn new() -> Self {
        Stack { data: Vec::new() }
    }
}
