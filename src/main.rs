use mini_evm::Stack;

fn main() {
    let mut stack = Stack::new();

    stack.push(3);
    stack.push(7);
    stack.push(5);
    println!("{:?}", stack);

    println!("Popped: {:?}", stack.pop());
    println!("Popped: {:?}", stack.pop());
    println!("Popped: {:?}", stack.pop());
    println!("Popped: {:?}", stack.pop());
}
