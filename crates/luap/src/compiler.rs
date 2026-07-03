#[salsa::db]
#[derive(Clone, Default)]
pub struct Compiler {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for Compiler {}

#[salsa::db]
impl crate::Db for Compiler {}
