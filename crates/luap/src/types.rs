use std::{path::Path, sync::Arc};

use bytestring::ByteString;
use camino::Utf8Path;

#[salsa::input]
#[derive(Debug)]
pub struct SourceFile {
    pub path: Arc<Utf8Path>,
    #[returns(ref)]
    pub contents: ByteString,
}

impl SourceFile {
    pub fn from_path(db: &dyn crate::Db, path: impl AsRef<Path>) -> Result<Self, std::io::Error> {
        let path = path.as_ref();
        let contents = std::fs::read_to_string(path)?;
        Ok(SourceFile::new(
            db,
            Utf8Path::from_path(path).unwrap().into(),
            contents.into(),
        ))
    }
}

#[salsa::tracked]
impl SourceFile {
    pub fn parse(self, db: &dyn crate::Db) -> crate::parsing::Tree {
        crate::parse(db, self)
    }
}
