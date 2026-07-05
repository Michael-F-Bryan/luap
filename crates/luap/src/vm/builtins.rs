use crate::vm::{Environment, NativeFuncValue, RuntimeError, Value};

#[derive(Debug, Clone)]
pub struct Builtins {
    pub print: NativeFuncValue,
}

impl Default for Builtins {
    fn default() -> Self {
        Self {
            print: NativeFuncValue::new(
                print as fn(&mut Environment, &[Value]) -> Result<Value, RuntimeError>,
            ),
        }
    }
}

fn print(env: &mut Environment, args: &[Value]) -> Result<Value, RuntimeError> {
    for (i, arg) in args.iter().enumerate() {
        if i > 0 {
            write!(env.stdout, "\t")?;
        }
        write!(env.stdout, "{arg}")?;
    }
    writeln!(env.stdout)?;

    Ok(Value::Nil)
}
