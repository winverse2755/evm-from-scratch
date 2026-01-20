use crate::evm::EVM;

pub fn add(evm: &mut EVM) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();
    evm.stack.push(a + b);

    //include program counter and gas cost handling later
}
