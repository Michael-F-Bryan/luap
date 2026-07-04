use std::sync::Arc;

use super::{
    builder::CodeBuilder,
    bytecode::{Constant, Instruction, Reg},
    resolve::{resolve_name, ResolvedName},
};
use crate::lowering::hir::{CallStmt, Expr, File, Stmt};

impl File<'_> {
    pub(crate) fn compile(&self, db: &dyn crate::Db, builder: &mut CodeBuilder) {
        for stmt in self.statements(db) {
            stmt.compile(db, builder);
        }
    }
}

impl Stmt {
    fn compile(&self, db: &dyn crate::Db, builder: &mut CodeBuilder) {
        match self {
            Stmt::Call(call) => call.compile(db, builder),
        }
    }
}

impl CallStmt {
    fn compile(&self, db: &dyn crate::Db, builder: &mut CodeBuilder) {
        let callee = builder.alloc_reg();
        self.callee.compile(db, builder, callee);

        let argc = self.args.len();
        let args_base = if argc == 0 {
            callee
        } else {
            let base = builder.reserve_regs(argc);
            for (index, arg) in self.args.iter().enumerate() {
                arg.compile(db, builder, Reg(base.0 + index as u16));
            }
            base
        };

        builder.emit(Instruction::Call {
            callee,
            args_base,
            argc: argc as u8,
        });
    }
}

impl Expr {
    fn compile(&self, db: &dyn crate::Db, builder: &mut CodeBuilder, dst: Reg) {
        match self {
            Expr::StringLiteral { value, .. } => {
                let idx = builder.intern_constant(Constant::String(Arc::from(value.as_str())));
                builder.emit(Instruction::LoadConst { dst, idx });
            }
            Expr::Name { name, ptr } => {
                if let Some(ResolvedName::Builtin(id)) = resolve_name(db, *ptr, name.clone()) {
                    builder.emit(Instruction::GetBuiltin { dst, id });
                }
            }
        }
    }
}
