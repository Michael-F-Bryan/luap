use salsa::Accumulator;
use type_sitter::{HasChildren, Node as _, TreeCursor};

use super::{
    hir::{self, CallStmt, Expr, Stmt},
    string,
};
use crate::{
    diagnostics::{Diagnostic, DiagnosticKind, Unsupported},
    pointer::{ChunkPtr, IdentifierPtr, StatementPtr, StringPtr},
    syntax::{self, anon_unions},
    SourceFile,
};

pub(crate) struct LowerCtx<'db> {
    pub db: &'db dyn crate::Db,
    pub source_file: SourceFile,
}

impl<'db> LowerCtx<'db> {
    pub(crate) fn lower_chunk(&self, chunk: syntax::Chunk<'_>) -> hir::File<'db> {
        let mut statements = Vec::new();
        let mut cursor = TreeCursor(chunk.raw().walk());

        for child in chunk.children(&mut cursor) {
            let child = match child {
                Ok(child) => child,
                Err(_) => continue,
            };
            match child {
                anon_unions::HashBangLine_ReturnStatement_Statement::HashBangLine(node) => {
                    self.unsupported(node.raw(), "hash bang line");
                }
                anon_unions::HashBangLine_ReturnStatement_Statement::ReturnStatement(node) => {
                    self.unsupported(node.raw(), "return statements");
                }
                anon_unions::HashBangLine_ReturnStatement_Statement::Statement(stmt) => {
                    if let Some(stmt) = self.lower_statement(stmt) {
                        statements.push(stmt);
                    }
                }
            }
        }

        hir::File::new(
            self.db,
            statements,
            ChunkPtr::from_node(self.source_file, chunk),
        )
    }

    fn lower_statement(&self, stmt: syntax::Statement<'_>) -> Option<Stmt> {
        match stmt {
            syntax::Statement::FunctionCall(call) => self.lower_function_call(call),
            syntax::Statement::AssignmentStatement(node) => {
                self.unsupported(node.raw(), "assignment statements");
                None
            }
            syntax::Statement::BreakStatement(node) => {
                self.unsupported(node.raw(), "break statements");
                None
            }
            syntax::Statement::Declaration(node) => {
                self.unsupported(node.raw(), "declarations");
                None
            }
            syntax::Statement::DoStatement(node) => {
                self.unsupported(node.raw(), "do statements");
                None
            }
            syntax::Statement::EmptyStatement(node) => {
                self.unsupported(node.raw(), "empty statements");
                None
            }
            syntax::Statement::ForStatement(node) => {
                self.unsupported(node.raw(), "for statements");
                None
            }
            syntax::Statement::GotoStatement(node) => {
                self.unsupported(node.raw(), "goto statements");
                None
            }
            syntax::Statement::IfStatement(node) => {
                self.unsupported(node.raw(), "if statements");
                None
            }
            syntax::Statement::LabelStatement(node) => {
                self.unsupported(node.raw(), "label statements");
                None
            }
            syntax::Statement::RepeatStatement(node) => {
                self.unsupported(node.raw(), "repeat statements");
                None
            }
            syntax::Statement::WhileStatement(node) => {
                self.unsupported(node.raw(), "while statements");
                None
            }
        }
    }

    fn lower_function_call(&self, call: syntax::FunctionCall<'_>) -> Option<Stmt> {
        let callee = match call.name().expect("function call name") {
            anon_unions::FunctionCall_MethodIndexExpression_ParenthesizedExpression_Variable::Variable(
                variable,
            ) => self.lower_variable(variable)?,
            anon_unions::FunctionCall_MethodIndexExpression_ParenthesizedExpression_Variable::FunctionCall(
                node,
            ) => {
                self.unsupported(node.raw(), "indirect calls");
                return None;
            }
            anon_unions::FunctionCall_MethodIndexExpression_ParenthesizedExpression_Variable::MethodIndexExpression(
                node,
            ) => {
                self.unsupported(node.raw(), "method calls");
                return None;
            }
            anon_unions::FunctionCall_MethodIndexExpression_ParenthesizedExpression_Variable::ParenthesizedExpression(
                node,
            ) => {
                self.unsupported(node.raw(), "parenthesized call targets");
                return None;
            }
        };

        let arguments = call.arguments().expect("function call arguments");
        let mut cursor = TreeCursor(arguments.raw().walk());
        let args = arguments
            .expressions(&mut cursor)
            .filter_map(|expr| expr.ok())
            .filter_map(|expr| self.lower_expression(expr))
            .collect();

        Some(Stmt::Call(CallStmt {
            callee,
            args,
            ptr: StatementPtr::from_node(self.source_file, syntax::Statement::FunctionCall(call)),
        }))
    }

    fn lower_variable(&self, variable: syntax::Variable<'_>) -> Option<Expr> {
        let source = &*self.source_file.contents(self.db);
        match variable {
            syntax::Variable::Identifier(identifier) => {
                let name = identifier
                    .raw()
                    .utf8_text(source.as_bytes())
                    .expect("valid utf8")
                    .to_string();
                Some(Expr::Name {
                    name,
                    ptr: IdentifierPtr::from_node(self.source_file, identifier),
                })
            }
            syntax::Variable::BracketIndexExpression(node) => {
                self.unsupported(node.raw(), "bracket index expressions");
                None
            }
            syntax::Variable::DotIndexExpression(node) => {
                self.unsupported(node.raw(), "dot index expressions");
                None
            }
            syntax::Variable::Global(node) => {
                self.unsupported(node.raw(), "global variables");
                None
            }
        }
    }

    fn lower_expression(&self, expr: syntax::Expression<'_>) -> Option<Expr> {
        let source = &*self.source_file.contents(self.db);
        match expr {
            syntax::Expression::String(string) => Some(Expr::String {
                value: string::decode(&string, source),
                ptr: StringPtr::from_node(self.source_file, string),
            }),
            syntax::Expression::BinaryExpression(node) => {
                self.unsupported(node.raw(), "binary expressions");
                None
            }
            syntax::Expression::False(node) => {
                self.unsupported(node.raw(), "boolean literals");
                None
            }
            syntax::Expression::FunctionCall(node) => {
                self.unsupported(node.raw(), "call expressions");
                None
            }
            syntax::Expression::FunctionDefinition(node) => {
                self.unsupported(node.raw(), "function definitions");
                None
            }
            syntax::Expression::Nil(node) => {
                self.unsupported(node.raw(), "nil literals");
                None
            }
            syntax::Expression::Number(node) => {
                self.unsupported(node.raw(), "number literals");
                None
            }
            syntax::Expression::ParenthesizedExpression(node) => {
                self.unsupported(node.raw(), "parenthesized expressions");
                None
            }
            syntax::Expression::TableConstructor(node) => {
                self.unsupported(node.raw(), "table constructors");
                None
            }
            syntax::Expression::True(node) => {
                self.unsupported(node.raw(), "boolean literals");
                None
            }
            syntax::Expression::UnaryExpression(node) => {
                self.unsupported(node.raw(), "unary expressions");
                None
            }
            syntax::Expression::VarargExpression(node) => {
                self.unsupported(node.raw(), "vararg expressions");
                None
            }
            syntax::Expression::Variable(node) => {
                self.unsupported(node.raw(), "variables in expression position");
                None
            }
        }
    }

    fn unsupported(&self, node: &type_sitter::raw::Node<'_>, feature: &str) {
        let path = self.source_file.path(self.db);
        let source = &*self.source_file.contents(self.db);
        Diagnostic(DiagnosticKind::Unsupported(Unsupported::at(
            path.as_str(),
            source,
            feature,
            node.range(),
        )))
        .accumulate(self.db);
    }
}
