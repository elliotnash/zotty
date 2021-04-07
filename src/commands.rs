use std::usize;

use serenity::prelude::SerenityError;
use std::error::Error;

#[derive(Clone, Debug)]
pub struct Args {
    args: Vec<String>,
    current: usize
}

impl Args {
    pub fn parse(string: &str) -> Args {
        Args{
            args: string.split_whitespace().map(|s| s.to_string()).collect(), 
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
