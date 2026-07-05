//! The bytecode representation of a Lua program.

use std::{ops::Add, sync::Arc};

use ordered_float::OrderedFloat;

use super::builtins::BuiltinId;

/// A virtual register index in the register file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Reg(pub u16);

impl Add<u8> for Reg {
    type Output = Reg;

    fn add(self, rhs: u8) -> Self::Output {
        Reg(self.0 + rhs as u16)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Bytecode {
    pub instructions: Vec<Instruction>,
    pub constants: Vec<Constant>,
    pub num_regs: u16,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Instruction {
    LoadConst {
        dst: Reg,
        idx: u16,
    },
    GetBuiltin {
        dst: Reg,
        id: BuiltinId,
    },
    Call {
        callee: Reg,
        args_base: Reg,
        argc: u8,
    },
    Halt,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Constant {
    String(Arc<str>),
    Number(Number),
    Boolean(bool),
    Nil,
}

pub type Number = OrderedFloat<f64>;
