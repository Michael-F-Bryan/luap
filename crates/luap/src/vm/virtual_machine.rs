use crate::{
    bytecode::Instruction,
    compiling::bytecode::Bytecode,
    vm::{builtins, Environment, Frame, NativeFuncValue, Value},
};

#[derive(Default, Debug)]
pub struct VirtualMachine {
    frame: Frame,
    env: Environment,
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
                            Value::NativeFunc(NativeFuncValue::new(
                                builtins::print
                                    as fn(
                                        &mut Environment,
                                        &[Value],
                                    )
                                        -> Result<Value, RuntimeError>,
                            ))
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
                        _ => {
                            return Err(RuntimeError::NotCallable {
                                value: self.frame[callee].clone(),
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
