use mini_evm::Stack;

fn main() {
    let mut stack = Stack::new();

    stack.push(10);
    stack.push(20);
    stack.push(30);
    println!("{:?}", stack);

    // println!("Popped: {:?}", stack.pop());
    // println!("Popped: {:?}", stack.pop());
    // println!("Popped: {:?}", stack.pop());
}
