use std::path::Path;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();

    let src = type_sitter_gen::generate_nodes(tree_sitter_lua::NODE_TYPES)
        .unwrap()
        .into_string();
    let out_path = Path::new(&out_dir).join("ast.rs");
    std::fs::write(out_path, src).unwrap();
}
