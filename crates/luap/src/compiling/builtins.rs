/// Built-in callable known to the compiler and runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum BuiltinId {
    Print = 0,
}

impl BuiltinId {
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "print" => Some(Self::Print),
            _ => None,
        }
    }
}
