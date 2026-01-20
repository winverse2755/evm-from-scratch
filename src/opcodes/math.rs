use crate::stack::Stack;

pub fn add(stack: &mut Stack) {
    let a = stack.pop().unwrap();
    let b = stack.pop().unwrap();
    stack.push(a + b);

    //include program counter and gas cost handling later
}
