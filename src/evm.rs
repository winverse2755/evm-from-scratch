pub struct EVM {
    stack: Stack,
    pc: usize, // program counter
    gas: u64,
    value: u64,
    calldata: Vec<u8>,
}

impl EVM {
    fn gas_cost(&mut self, cost: u64) {
        if self.gas >= cost {
            self.gas -= cost;
        } else {
            panic!("Out of gas");
        }
    }

    fn step(&mut self) {
        // Placeholder for fetching the next opcode
        // let opcode = self.fetch_opcode();
        // match opcode {
        //     Opcode::ADD => {
        //         math::add(&mut self.stack);
        //         self.gas_cost(3); // example gas cost for ADD
        //     }
        //     _ => unimplemented!(),
        // }
        self.pc += 1; // Move to the next instruction
    }
}
