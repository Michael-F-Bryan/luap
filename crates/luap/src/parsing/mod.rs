mod tree;

use type_sitter::Node;

pub use self::tree::Tree;

use crate::types::SourceFile;

/// Parse a source file into a concrete syntax tree.
#[salsa::tracked]
pub fn parse(db: &dyn crate::Db, source_file: SourceFile) -> Tree {
    let lang = tree_sitter::Language::from(tree_sitter_lua::LANGUAGE);
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(&lang).unwrap();
    let tree = parser.parse(source_file.contents(db), None).unwrap();

    if tree.root_node().is_missing() {
        todo!();
    }

    tree.into()
}

#[salsa::tracked]
pub struct ParsedOutput<'db> {
    pub source_file: SourceFile,
    #[returns(ref)]
    pub tree: Tree,
}

#[salsa::tracked]
impl<'db> ParsedOutput<'db> {
    pub fn root(&self, db: &'db dyn crate::Db) -> crate::syntax::Chunk<'db> {
        let root = self.tree(db).root_node();
        crate::syntax::Chunk::try_from_raw(root).unwrap()
    }
}
