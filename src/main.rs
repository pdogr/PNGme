#![allow(unused_imports)]
#![allow(dead_code)]
mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    commands::Command::run(args::Config::parse_args()?)?;
    Ok(())
}
