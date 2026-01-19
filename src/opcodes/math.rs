use mini_evm::stack::Stack;

fn add(stack: &mut Stack) {
    let a = stack.pop().unwrap();
    let b = stack.pop().unwrap();
    let result = a + b;
    stack.push(result);
}
