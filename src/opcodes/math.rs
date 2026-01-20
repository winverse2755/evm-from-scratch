use crate::stack::Stack;

fn add(stack: &mut Stack) {
    let a = stack.pop().unwrap();
    let b = stack.pop().unwrap();
    let result = a + b;
    stack.push(result);

    //include program counter and gas cost handling later
}
