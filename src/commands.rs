use serenity::prelude::SerenityError;

pub struct Args {
    args: Vec<String>
}

impl Args {
    pub fn parse(string: &str) -> Args {
        Args{args: string.split_whitespace().map(|s| s.to_string()).collect()}
    }
}

pub type CommandResult = Result<(), SerenityError>;
