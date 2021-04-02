use std::backtrace::Backtrace;

use thiserror::Error;
use crate::morse::{ascii_to_morse, morse_to_ascii};

#[derive(Error, Debug)]
pub enum CheckboxClientError {
    #[error("I/O Error occurred: {0:?}\n{1}")]
    IoError(#[from] std::io::Error, Backtrace),
    #[error("attohttpc Error occurred: {0:?}\n{1}")]
    AttoHttpCError(#[from] attohttpc::Error, Backtrace),
}

type Result<T> = std::result::Result<T, CheckboxClientError>;

#[derive(Debug, Default)]
pub struct CheckboxClient {
    pub state_id: Option<String>,
}

const BASE_URL: &str = "https://xkcd.com/2445/morse/...";

impl CheckboxClient {
    pub fn new() -> CheckboxClient {
        Default::default()
    }

    pub fn communicate(&mut self, input: &str) -> Result<String> {
        if input.starts_with("%") {
            return self.handle_command(&input[1..]);
        }

        let input_as_url_morse = url_safe_morse(input);
        let url = match &self.state_id {
            Some(u) => [BASE_URL, &*url_safe_morse(u), &*input_as_url_morse].join("/"),
            None => [BASE_URL, &*input_as_url_morse].join("/"),
        };
        eprintln!("! GET {}", url);
        let response = attohttpc::get(url)
            .send()?
            .error_for_status()?;
        let decoded: Vec<String> = response.text()?
            .split('/')
            .map(|s| morse_to_ascii(&*s.trim()))
            .collect();
        // first is always UUID?
        let (first, rest) = decoded.split_first().expect("XKCD didn't give us 2+ fields");
        let previous = self.state_id.replace(first.clone());
        if previous.is_none() || &previous.unwrap() != first {
            eprintln!("! Assigned State ID {}", first);
        }

        Ok(rest.join(" "))
    }

    fn handle_command(&mut self, cmd: &str) -> Result<String> {
        let split: Vec<_> = cmd.split(" ").collect();
        let (name, args) = split.split_first().unwrap();
        match name {
            &"state" => {
                match args {
                    &[] => {
                        self.state_id = None;
                        Ok(String::from("! Cleared state"))
                    }
                    &[state_id] => {
                        self.state_id = Some(String::from(state_id));
                        Ok(format!("! Set state to {}", state_id))
                    }
                    _ => Ok(String::from("! Invalid arguments.")),
                }
            }
            _ => Ok(String::from("! Invalid command.")),
        }
    }
}

fn url_safe_morse(input: &str) -> String {
    let r = ascii_to_morse(input).replace(' ', "_");
    match &*r {
        "." => String::from("_."),
        ".." => String::from("_.."),
        _ => r,
    }
}
