use mini_evm::memory;
use mini_evm::opcodes::math;
use mini_evm::stack::Stack;

fn main() {
    let mut stack = Stack::new();

    //push items to stack and print
    // stack.push(3);
    // stack.push(7);
    // stack.push(5);
    // println!("{:?}", stack);

    // // pop one item
    // println!("Popped: {:?}", stack.pop());

    // //print stack
    // println!("{:?}", stack);

    stack.push(10);
    stack.push(20);
    println!("{:?}", stack);

    math::add(&mut stack);
    println!("{:?}", stack);
}
