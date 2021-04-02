#![deny(warnings)]
#![feature(backtrace)]

use std::backtrace::Backtrace;
use std::io::Write;
use std::process::exit;

use console::{Term, style};
use thiserror::Error;

use crate::checkbox::{CheckboxClient, CheckboxClientError};

mod checkbox;
mod morse;

#[derive(Error, Debug)]
enum Error {
    #[error("I/O Error occurred: {0:?}\n{1}")]
    IoError(#[from] std::io::Error, Backtrace),
    #[error("Checkbox Client Error occurred: {0}\n{1}")]
    CheckboxClientError(#[from] CheckboxClientError, Backtrace),
}

type Result<T> = std::result::Result<T, Error>;

fn main() {
    if let Err(error) = main_for_result() {
        eprintln!("{}", style(format!("Error: {:?}", error)).for_stderr().red());
        exit(1);
    }
}

fn main_for_result() -> Result<()> {
    let mut stdout = console::Term::stdout();
    stdout.write_line("Welcome to Checkbox - Enter any text")?;

    let mut client = CheckboxClient::new();

    loop {
        let input = ask(&mut stdout, "> ")?;
        let output = client.communicate(&input)?;
        stdout.write_line(&*format!("< {}", output))?;
    }
}

fn ask(term: &mut Term, prompt: &str) -> Result<String> {
    term.write(prompt.as_bytes())?;

    Ok(term.read_line()?)
}
