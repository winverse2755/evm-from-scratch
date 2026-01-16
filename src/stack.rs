struct Stack {
    data: Vec<i32>,
}

impl Stack {
    fn new() -> Self {
        Stack { data: Vec::new() }
    }
}
