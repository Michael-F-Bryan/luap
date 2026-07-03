use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub struct Tree(tree_sitter::Tree);

impl From<tree_sitter::Tree> for Tree {
    fn from(tree: tree_sitter::Tree) -> Self {
        Self(tree)
    }
}

impl PartialEq for Tree {
    fn eq(&self, other: &Self) -> bool {
        let mut left = self.0.root_node().walk();
        let mut right = other.0.root_node().walk();

        cursors_equal(&mut left, &mut right)
    }
}

impl Eq for Tree {}

pub(crate) fn cursors_equal(
    left: &mut tree_sitter::TreeCursor,
    right: &mut tree_sitter::TreeCursor,
) -> bool {
    loop {
        if !nodes_equal(left.node(), right.node()) {
            return false;
        }

        if left.goto_first_child() {
            if !right.goto_first_child() || !cursors_equal(left, right) {
                return false;
            }
            left.goto_parent();
            right.goto_parent();
        } else if right.goto_first_child() {
            return false;
        }

        if left.goto_next_sibling() {
            if !right.goto_next_sibling() {
                return false;
            }
        } else {
            return !right.goto_next_sibling();
        }
    }
}

pub(crate) fn nodes_equal(left: tree_sitter::Node, right: tree_sitter::Node) -> bool {
    left.kind() == right.kind()
        && left.byte_range() == right.byte_range()
        && left.is_missing() == right.is_missing()
        && left.is_error() == right.is_error()
}

impl Hash for Tree {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut cursor = self.0.root_node().walk();
        hash_cursor(&mut cursor, state);
    }
}

fn hash_cursor<H: Hasher>(cursor: &mut tree_sitter::TreeCursor, state: &mut H) {
    loop {
        hash_node(cursor.node(), state);

        if cursor.goto_first_child() {
            hash_cursor(cursor, state);
            cursor.goto_parent();
        }

        if cursor.goto_next_sibling() {
            continue;
        }
        break;
    }
}

fn hash_node<H: Hasher>(node: tree_sitter::Node, state: &mut H) {
    node.kind().hash(state);
    node.byte_range().hash(state);
    node.is_missing().hash(state);
    node.is_error().hash(state);
}
