use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

mod lexer;
mod tokens;

fn main() -> Result<()> {
    let mut lex: lexer::Lexer;
    let mut rl = DefaultEditor::new()?;
    let history_file = std::env::home_dir().unwrap().join(".lang_history");

    let _ = rl.save_history(&history_file);

    loop {
        let readline = rl.readline(r"λ ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                lex = lexer::Lexer::new(line);
                while let Some(token) = lex.next() {
                    println!("{:?}", token);
                }
            },
            Err(ReadlineError::Interrupted) => {
                break
            },
            Err(ReadlineError::Eof) => {
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }

    let _ = rl.save_history(&history_file);
    Ok(())
}
