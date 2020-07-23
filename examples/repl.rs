use codespan_reporting::files::SimpleFile;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use codespan_reporting::term::{self, Config};
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();

    let mut inputs = Vec::new();

    loop {
        write!(stdout, "> ")?;
        stdout.flush()?;

        let input = {
            let mut input = String::new();
            stdin.read_line(&mut input)?;

            input
        };

        let parse = expr_parser::Parser::new(&input).parse();

        display_diagnostics(&input, &parse)?;

        if let Some(result) = parse.eval() {
            writeln!(stdout, "{}", result)?;
        } else {
            writeln!(stderr, "failed to evaluate")?;
        }

        writeln!(stderr, "\n{}", parse.format())?;

        inputs.push(input);
    }
}

fn display_diagnostics(input: &str, parse: &expr_parser::Parse) -> io::Result<()> {
    let file = SimpleFile::new("REPL input", &input);

    let diagnostics = parse.diagnostics(());

    let writer = StandardStream::stderr(ColorChoice::Auto);
    let mut writer = writer.lock();

    let config = Config::default();

    for diagnostic in diagnostics {
        term::emit(&mut writer, &config, &file, &diagnostic)?;
    }

    Ok(())
}
