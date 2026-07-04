use std::sync::Arc;

use crate::vm::{Environment, RuntimeError, Value};

#[derive(Clone)]
pub struct NativeFuncValue {
    name: String,
    func: Arc<dyn NativeFunc>,
}

impl NativeFuncValue {
    pub fn new<F>(func: F) -> Self
    where
        F: NativeFunc + 'static,
    {
        let name = std::any::type_name::<F>();
        Self {
            name: name.to_string(),
            func: Arc::new(func),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn call(&self, env: &mut Environment, args: &[Value]) -> Result<Value, RuntimeError> {
        self.func.call(env, args)
    }
}

impl std::fmt::Debug for NativeFuncValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NativeFuncValue")
            .field("name", &self.name)
            .finish_non_exhaustive()
    }
}

impl PartialEq for NativeFuncValue {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && Arc::ptr_eq(&self.func, &other.func)
    }
}

pub trait NativeFunc {
    fn call(&self, env: &mut Environment, args: &[Value]) -> Result<Value, RuntimeError>;
}

// Varargs functions
impl NativeFunc for fn(&mut Environment, &[Value]) -> Result<Value, RuntimeError> {
    fn call(&self, env: &mut Environment, args: &[Value]) -> Result<Value, RuntimeError> {
        self(env, args)
    }
}

macro_rules! count {
    ($head:ident $($tail:ident)*) => {
        1 + count!($($tail)*)
    };
    () => {
        0
    };
}

macro_rules! native_func_fixed_arity {
    () => {};

    ($first:ident) => {
        #[allow(non_snake_case)]
        impl NativeFunc
            for fn(&mut Environment, $first: Value) -> Result<Value, RuntimeError>
        {
            fn call(
                &self,
                env: &mut Environment,
                args: &[Value],
            ) -> Result<Value, RuntimeError> {
                match args {
                    [$first] => self(env, $first.clone()),
                    _ => Err(RuntimeError::MismatchedArgumentCount {
                        expected: 1,
                        got: args.len(),
                    }),
                }
            }
        }
    };

    ($first:ident $($rest:ident)+) => {
        native_func_fixed_arity!($($rest)+);

        #[allow(non_snake_case)]
        impl NativeFunc
            for fn(
                &mut Environment,
                $first: Value,
                $($rest: Value),+
            ) -> Result<Value, RuntimeError>
        {
            fn call(
                &self,
                env: &mut Environment,
                args: &[Value],
            ) -> Result<Value, RuntimeError> {
                match args {
                    [$first, $($rest),+] => self(env, $first.clone(), $($rest.clone()),+),
                    _ => Err(RuntimeError::MismatchedArgumentCount {
                        expected: count!($first $($rest)+),
                        got: args.len(),
                    }),
                }
            }
        }
    };
}

impl NativeFunc for fn(&mut Environment) -> Result<Value, RuntimeError> {
    fn call(&self, env: &mut Environment, args: &[Value]) -> Result<Value, RuntimeError> {
        if !args.is_empty() {
            return Err(RuntimeError::MismatchedArgumentCount {
                expected: 0,
                got: args.len(),
            });
        }
        self(env)
    }
}

native_func_fixed_arity!(A B C D E F G H I J K L M N O P Q R S T U V W X Y Z);
