use crate::evm::EVM;

pub fn add(evm: &mut EVM) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();
    evm.stack.push(a + b);
    evm.gas_cost(3);
    evm.step();
}

pub fn mul(evm: &mut EVM) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();
    evm.stack.push(a * b);
    evm.gas_cost(5);
    evm.step();
}

pub fn sub(evm: &mut EVM) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();
    evm.stack.push(a - b);
    evm.gas_cost(3);
    evm.step();
}
