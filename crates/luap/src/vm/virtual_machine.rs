use crate::{
    bytecode::Instruction,
    compiling::bytecode::Bytecode,
    vm::{Builtins, Environment, Frame, Value},
};

#[derive(Default, Debug)]
pub struct VirtualMachine {
    frame: Frame,
    env: Environment,
    builtins: Builtins,
    pc: usize,
}

impl VirtualMachine {
    pub fn with_env(env: Environment) -> Self {
        Self {
            env,
            ..Default::default()
        }
    }

    pub fn env(&self) -> &Environment {
        &self.env
    }

    pub fn env_mut(&mut self) -> &mut Environment {
        &mut self.env
    }

    pub fn run_program(&mut self, bytecode: &Bytecode) -> Result<u8, RuntimeError> {
        self.frame
            .registers
            .resize(bytecode.num_regs as usize, Value::Nil);

        while self.pc < bytecode.instructions.len() {
            match bytecode.instructions[self.pc] {
                Instruction::LoadConst { dst, idx } => {
                    self.frame[dst] = Value::from(bytecode.constants[idx as usize].clone());
                }
                Instruction::GetBuiltin { dst, id } => {
                    self.frame[dst] = match id {
                        crate::compiling::builtins::BuiltinId::Print => {
                            self.builtins.print.clone().into()
                        }
                    };
                }
                Instruction::Call {
                    callee,
                    args_base,
                    argc,
                } => {
                    let args = &self.frame[args_base..args_base + argc];
                    match &self.frame[callee] {
                        Value::NativeFunc(func) => {
                            func.call(&mut self.env, args)?;
                        }
                        other => {
                            return Err(RuntimeError::NotCallable {
                                value: other.clone(),
                            });
                        }
                    }
                }
                Instruction::Halt => return Ok(0),
            }
            self.pc += 1;
        }

        Ok(0)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RuntimeError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Mismatched argument count: expected {expected}, got {got}")]
    MismatchedArgumentCount { expected: usize, got: usize },
    #[error("attempt to call a non-function value ({value})")]
    NotCallable { value: Value },
}

#[cfg(test)]
mod tests {
    use std::{io::Write, sync::Arc};

    use super::{Environment, RuntimeError, Value, VirtualMachine};
    use crate::compiling::{
        builtins::BuiltinId,
        bytecode::{Bytecode, Constant, Instruction, Number, Reg},
    };

    #[derive(Clone, Default)]
    struct SharedBuffer(std::sync::Arc<std::sync::Mutex<Vec<u8>>>);

    impl Write for SharedBuffer {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.0.lock().unwrap().write(buf)
        }

        fn flush(&mut self) -> std::io::Result<()> {
            self.0.lock().unwrap().flush()
        }
    }

    fn run_with_stdout(bytecode: &Bytecode) -> Result<(u8, String), RuntimeError> {
        let buffer = SharedBuffer::default();
        let shared = buffer.clone();
        let mut vm = VirtualMachine::with_env(Environment {
            stdout: Box::new(buffer),
        });
        let code = vm.run_program(bytecode)?;
        let output = String::from_utf8(shared.0.lock().unwrap().clone()).unwrap();
        Ok((code, output))
    }

    fn stdout_after_error(bytecode: &Bytecode) -> (RuntimeError, String) {
        let buffer = SharedBuffer::default();
        let shared = buffer.clone();
        let mut vm = VirtualMachine::with_env(Environment {
            stdout: Box::new(buffer),
        });
        let err = vm.run_program(bytecode).unwrap_err();
        let output = String::from_utf8(shared.0.lock().unwrap().clone()).unwrap();
        (err, output)
    }

    #[test]
    fn prints_hello_world() {
        let bytecode = Bytecode {
            instructions: vec![
                Instruction::GetBuiltin {
                    dst: Reg(0),
                    id: BuiltinId::Print,
                },
                Instruction::LoadConst {
                    dst: Reg(1),
                    idx: 0,
                },
                Instruction::Call {
                    callee: Reg(0),
                    args_base: Reg(1),
                    argc: 1,
                },
                Instruction::Halt,
            ],
            constants: vec![Constant::String(Arc::from("Hello, world!"))],
            num_regs: 2,
        };

        let (code, output) = run_with_stdout(&bytecode).unwrap();
        assert_eq!(code, 0);
        assert_eq!(output, "Hello, world!\n");
    }

    #[test]
    fn print_separates_arguments_with_tabs() {
        let bytecode = Bytecode {
            instructions: vec![
                Instruction::GetBuiltin {
                    dst: Reg(0),
                    id: BuiltinId::Print,
                },
                Instruction::LoadConst {
                    dst: Reg(1),
                    idx: 0,
                },
                Instruction::LoadConst {
                    dst: Reg(2),
                    idx: 1,
                },
                Instruction::LoadConst {
                    dst: Reg(3),
                    idx: 2,
                },
                Instruction::Call {
                    callee: Reg(0),
                    args_base: Reg(1),
                    argc: 3,
                },
                Instruction::Halt,
            ],
            constants: vec![
                Constant::String(Arc::from("hello")),
                Constant::Number(Number::from(42.0)),
                Constant::Boolean(true),
            ],
            num_regs: 4,
        };

        let (_, output) = run_with_stdout(&bytecode).unwrap();
        assert_eq!(output, "hello\t42\ttrue\n");
    }

    #[test]
    fn print_with_no_arguments_emits_bare_newline() {
        let bytecode = Bytecode {
            instructions: vec![
                Instruction::GetBuiltin {
                    dst: Reg(0),
                    id: BuiltinId::Print,
                },
                Instruction::Call {
                    callee: Reg(0),
                    args_base: Reg(0),
                    argc: 0,
                },
                Instruction::Halt,
            ],
            constants: vec![],
            num_regs: 1,
        };

        let (_, output) = run_with_stdout(&bytecode).unwrap();
        assert_eq!(output, "\n");
    }

    #[test]
    fn calling_a_non_function_value_fails() {
        let bytecode = Bytecode {
            instructions: vec![
                Instruction::LoadConst {
                    dst: Reg(0),
                    idx: 0,
                },
                Instruction::Call {
                    callee: Reg(0),
                    args_base: Reg(0),
                    argc: 0,
                },
                Instruction::Halt,
            ],
            constants: vec![Constant::String(Arc::from("not a function"))],
            num_regs: 1,
        };

        let (err, output) = stdout_after_error(&bytecode);
        assert!(output.is_empty());
        match err {
            RuntimeError::NotCallable { value } => {
                assert_eq!(value, Value::String(Arc::from("not a function")));
            }
            other => panic!("expected NotCallable, got {other:?}"),
        }
    }

    #[test]
    fn registers_default_to_nil() {
        let bytecode = Bytecode {
            instructions: vec![
                Instruction::GetBuiltin {
                    dst: Reg(0),
                    id: BuiltinId::Print,
                },
                Instruction::Call {
                    callee: Reg(0),
                    args_base: Reg(1),
                    argc: 1,
                },
                Instruction::Halt,
            ],
            constants: vec![],
            num_regs: 2,
        };

        let (_, output) = run_with_stdout(&bytecode).unwrap();
        assert_eq!(output, "nil\n");
    }
}
