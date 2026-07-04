//! The bytecode representation of a Lua program.

use std::sync::Arc;

use ordered_float::OrderedFloat;

use super::builtins::BuiltinId;

/// A virtual register index in the register file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Reg(pub u16);

#[salsa::tracked]
#[derive(Debug)]
pub struct Bytecode<'db> {
    #[returns(ref)]
    pub instructions: Vec<Instruction>,
    #[returns(ref)]
    pub constants: Vec<Constant>,
    pub num_regs: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Instruction {
    LoadConst { dst: Reg, idx: u16 },
    GetBuiltin { dst: Reg, id: BuiltinId },
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
    Integer(i64),
    Boolean(bool),
    Nil,
}

pub type Number = OrderedFloat<f64>;
