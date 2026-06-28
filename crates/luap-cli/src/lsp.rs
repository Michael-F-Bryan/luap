use clap::Parser;

#[derive(Parser, Debug)]
pub struct LspCmd {
    #[arg(
        long,
        group = "lsp-mode",
        required = true,
        help = "The TCP address to run the LSP server on"
    )]
    pub(crate) tcp: Option<String>,
    #[arg(
        long,
        group = "lsp-mode",
        required = true,
        help = "Whether to communicate over stdin/stdout"
    )]
    pub(crate) stdio: bool,
}

impl LspCmd {
    pub fn run(self) {
        todo!("Implement the LSP server")
    }
}
