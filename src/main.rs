use mini_evm::memory;
use mini_evm::stack::Stack;
pub mod opcodes;

fn main() {
    let mut stack = Stack::new();

    //push items to stack and print
    stack.push(3);
    stack.push(7);
    stack.push(5);
    println!("{:?}", stack);

    // pop one item
    println!("Popped: {:?}", stack.pop());

    //print stack
    println!("{:?}", stack);
}
