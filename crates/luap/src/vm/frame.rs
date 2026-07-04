use std::ops::{Bound, Index, IndexMut};

use crate::{bytecode::Reg, vm::Value};

#[derive(Debug, Default)]
pub struct Frame {
    pub registers: Vec<Value>,
}

impl Index<Reg> for Frame {
    type Output = Value;

    fn index(&self, index: Reg) -> &Self::Output {
        &self.registers[index.0 as usize]
    }
}

impl IndexMut<Reg> for Frame {
    fn index_mut(&mut self, index: Reg) -> &mut Self::Output {
        &mut self.registers[index.0 as usize]
    }
}

impl<R: std::ops::RangeBounds<Reg>> Index<R> for Frame {
    type Output = [Value];

    fn index(&self, index: R) -> &Self::Output {
        let start = match index.start_bound() {
            Bound::Included(r) => r.0 as usize,
            Bound::Excluded(r) => r.0 as usize + 1,
            Bound::Unbounded => 0,
        };
        let end = match index.end_bound() {
            Bound::Included(r) => r.0 as usize + 1,
            Bound::Excluded(r) => r.0 as usize,
            Bound::Unbounded => self.registers.len(),
        };
        &self.registers[start..end]
    }
}
