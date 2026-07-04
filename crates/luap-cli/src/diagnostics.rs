use std::io::{stderr, IsTerminal, Write};

use miette::{GraphicalReportHandler, GraphicalTheme, Report};

pub fn print_report(report: &Report) {
    let color = stderr().is_terminal() && std::env::var_os("NO_COLOR").is_none();
    let theme = if color {
        GraphicalTheme::unicode()
    } else {
        GraphicalTheme::unicode_nocolor()
    };
    let handler = GraphicalReportHandler::new_themed(theme);
    let mut buffer = String::new();
    let _ = handler.render_report(&mut buffer, report.as_ref());
    let _ = stderr().write_all(buffer.as_bytes());
}
