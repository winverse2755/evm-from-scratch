use mini_evm::evm::EVM;
use mini_evm::opcodes::math;
use mini_evm::stack::Stack;

fn main() {
    let mut evm = EVM {
        stack: Stack::new(),
        pc: 0,
        gas: 100,
        value: 0,
        calldata: vec![],
    };

    evm.stack.push(10);
    evm.stack.push(20);

    math::add(&mut evm);

    println!("{:#?}", evm);
}
