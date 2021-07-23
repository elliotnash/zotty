use std::usize;

use std::error::Error;

#[derive(Clone, Debug)]
pub struct Args {
    pub command: String,
    args: Vec<String>,
    current: usize
}

impl Args {
    pub fn parse(string: &str) -> Args {
        let mut arg_iter = string.splitn(2, " ");
        let command = arg_iter.next().unwrap_or("").to_lowercase();
        let str_args = arg_iter.next().unwrap_or("");
        Args{
            command,
            args: str_args.split_whitespace().map(|s| s.to_string()).collect(), 
            current: 0
        }
    }
    pub fn current(&self) -> Option<&str> {
        self.args.get(self.current).map(|s|s.as_str())
    }
    pub fn is_empty(&self) -> bool {
        self.args.is_empty()
    }
    pub fn len(&self) -> usize {
        self.args.len()
    }
}

pub type CommandResult = Result<(), Box<dyn Error>>;
