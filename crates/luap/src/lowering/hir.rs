#[salsa::tracked]
#[derive(Debug)]
pub struct File<'db> {
    statements: Vec<Statement<'db>>,
}

#[salsa::tracked]
#[derive(Debug)]
pub struct Statement<'db> {}
