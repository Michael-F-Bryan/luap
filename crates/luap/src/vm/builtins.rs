use crate::vm::{Environment, RuntimeError, Value};

pub fn print(env: &mut Environment, args: &[Value]) -> Result<Value, RuntimeError> {
    for (i, arg) in args.iter().enumerate() {
        if i > 0 {
            write!(env.stdout, "\t")?;
        }
        write!(env.stdout, "{arg}")?;
    }
    writeln!(env.stdout)?;

    Ok(Value::Nil)
}
