use std::io::{self, Write};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();

    let mut input = String::new();

    loop {
        write!(stdout, "> ")?;
        stdout.flush()?;

        stdin.read_line(&mut input)?;

        let parse = expr_parser::Parser::new(&input).parse();

        if let Some(result) = parse.eval() {
            writeln!(stdout, "{}", result)?;
        } else {
            writeln!(stderr, "Failed to evaluate.")?;
        }

        writeln!(stderr, "{}", parse.format())?;

        input.clear();
    }
}
