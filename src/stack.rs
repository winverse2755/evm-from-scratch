const MAX_STACK_SIZE: u32 = 1024;

#[derive(Debug, PartialEq)]

pub struct Stack {
    data: Vec<u32>,
}

impl Stack {
    pub fn new() -> Self {
        Stack { data: Vec::new() }
    }

    pub fn push(&mut self, item: u32) {
        if item < MAX_STACK_SIZE {
            self.data.push(item);
        } else {
            panic!("Stack overflow: cannot push more items")
        }
    }

    pub fn pop(&mut self) -> Option<u32> {
        if self.data.len() == 0 {
            panic!("Stack underflow: cannot pop from empty stack")
        } else {
            self.data.pop()
        }
    }
}
