use std::marker::PhantomData;

use tree_sitter::Range;
use type_sitter::Node as _;

use crate::SourceFile;

/// A reference to an untyped node in a concrete syntax tree.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Pointer {
    pub source_file: SourceFile,
    pub span: Range,
    pub kind: &'static str,
}

impl Pointer {
    pub fn from_node(source_file: SourceFile, node: tree_sitter::Node<'_>) -> Self {
        Pointer {
            source_file,
            span: node.range(),
            kind: node.kind(),
        }
    }

    pub fn node<'tree>(self, tree: &'tree tree_sitter::Tree) -> tree_sitter::Node<'tree> {
        let mut cursor = tree.walk();
        find_node(&mut cursor, self.kind, self.span)
            .expect("pointer does not resolve to a node in this tree")
    }
}

fn find_node<'tree>(
    cursor: &mut tree_sitter::TreeCursor<'tree>,
    kind: &str,
    span: Range,
) -> Option<tree_sitter::Node<'tree>> {
    loop {
        let node = cursor.node();
        if node.kind() == kind && node.range() == span {
            return Some(node);
        }

        if cursor.goto_first_child() {
            if let Some(node) = find_node(cursor, kind, span) {
                return Some(node);
            }
            cursor.goto_parent();
        }

        if cursor.goto_next_sibling() {
            continue;
        }
        break;
    }

    None
}

/// A reference to a typed node in a concrete syntax tree.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Ptr<T> {
    pointer: Pointer,
    _marker: PhantomData<T>,
}

impl<T: type_sitter::Node<'static>> Ptr<T> {
    pub fn from_node<'db>(
        source_file: SourceFile,
        node: <T as type_sitter::Node<'static>>::WithLifetime<'db>,
    ) -> Self {
        let pointer = Pointer::from_node(source_file, *node.raw());
        Ptr {
            pointer,
            _marker: PhantomData,
        }
    }

    pub fn untyped(self) -> Pointer {
        self.pointer
    }

    pub fn node<'tree>(self, tree: &'tree tree_sitter::Tree) -> T::WithLifetime<'tree> {
        let node = self.pointer.node(tree);
        T::WithLifetime::try_from_raw(node).unwrap()
    }
}

macro_rules! pointer_types {
    (
        $(
            $name:ident = $node:ident,
        )*
    ) => {
        $(
            #[doc = concat!("A reference to a [`crate::syntax::", stringify!($node), "`] node in a concrete syntax tree.")]
            pub type $name = Ptr<$crate::syntax::$node<'static>>;
        )*
    };
}

pointer_types! {
    ChunkPtr = Chunk,
    BlockPtr = Block,
    StatementPtr = Statement,
    ExpressionPtr = Expression,
    NumberPtr = Number,
    StringPtr = String,
    NilPtr = Nil,
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use camino::Utf8Path;

    use super::*;
    use crate::{parse, Compiler, SourceFile};

    fn parse_tree(source: &str) -> tree_sitter::Tree {
        let mut parser = tree_sitter::Parser::new();
        let lang = tree_sitter::Language::from(tree_sitter_lua::LANGUAGE);
        parser.set_language(&lang).unwrap();
        parser.parse(source, None).unwrap()
    }

    fn nodes_equal(left: tree_sitter::Node<'_>, right: tree_sitter::Node<'_>) -> bool {
        left.kind() == right.kind()
            && left.byte_range() == right.byte_range()
            && left.is_missing() == right.is_missing()
            && left.is_error() == right.is_error()
    }

    fn source_file(db: &Compiler, path: &str, source: &str) -> SourceFile {
        SourceFile::new(db, Arc::from(Utf8Path::new(path)), source.into())
    }

    fn collect_nodes<'tree>(node: tree_sitter::Node<'tree>) -> Vec<tree_sitter::Node<'tree>> {
        let mut nodes = vec![node];
        let mut cursor = node.walk();
        if cursor.goto_first_child() {
            loop {
                nodes.extend(collect_nodes(cursor.node()));
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
        }
        nodes
    }

    #[test]
    fn node_resolves_pointer_for_every_node_in_tree() {
        let source = "print(42)\nlocal x = \"hi\"";
        let tree = parse_tree(source);
        let db = Compiler::default();
        let source_file = source_file(&db, "test.lua", source);

        for node in collect_nodes(tree.root_node()) {
            let pointer = Pointer::from_node(source_file, node);
            let resolved = pointer.node(&tree);
            assert!(nodes_equal(node, resolved), "kind = {}", node.kind());
        }
    }

    #[test]
    fn node_resolves_through_salsa_parse() {
        let source = "return 1 + 2";
        let db = Compiler::default();
        let source_file = source_file(&db, "test.lua", source);
        let tree = parse(&db, source_file);
        let root = tree.root_node();

        let pointer = Pointer::from_node(source_file, root);
        let resolved = pointer.node(&tree);
        assert!(nodes_equal(root, resolved));
    }

    #[test]
    #[should_panic(expected = "pointer does not resolve to a node in this tree")]
    fn node_panics_when_pointer_does_not_match_tree() {
        let tree = parse_tree("print(1)");
        let db = Compiler::default();
        let source_file = source_file(&db, "test.lua", "print(1)");

        let pointer = Pointer {
            source_file,
            span: tree_sitter::Range {
                start_byte: 0,
                end_byte: 1,
                start_point: tree_sitter::Point { row: 0, column: 0 },
                end_point: tree_sitter::Point { row: 0, column: 1 },
            },
            kind: "number",
        };

        pointer.node(&tree);
    }
}
