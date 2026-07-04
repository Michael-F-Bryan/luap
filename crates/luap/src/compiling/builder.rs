use super::bytecode::{Bytecode, Constant, Instruction, Reg};

pub(crate) struct CodeBuilder {
    instructions: Vec<Instruction>,
    constants: Vec<Constant>,
    next_reg: u16,
}

impl CodeBuilder {
    pub(crate) fn new() -> Self {
        Self {
            instructions: Vec::new(),
            constants: Vec::new(),
            next_reg: 0,
        }
    }

    pub(crate) fn alloc_reg(&mut self) -> Reg {
        let reg = Reg(self.next_reg);
        self.next_reg += 1;
        reg
    }

    pub(crate) fn reserve_regs(&mut self, count: usize) -> Reg {
        let base = Reg(self.next_reg);
        self.next_reg += count as u16;
        base
    }

    pub(crate) fn intern_constant(&mut self, constant: Constant) -> u16 {
        if let Some(idx) = self
            .constants
            .iter()
            .position(|existing| existing == &constant)
        {
            return idx as u16;
        }

        let idx = self.constants.len();
        self.constants.push(constant);
        idx as u16
    }

    pub(crate) fn emit(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }

    pub(crate) fn finish(self) -> Bytecode {
        let mut instructions = self.instructions;
        instructions.push(Instruction::Halt);

        Bytecode {
            instructions,
            constants: self.constants,
            num_regs: self.next_reg,
        }
    }
}
