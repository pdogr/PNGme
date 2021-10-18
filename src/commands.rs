use crate::args::{ArgsKind, DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::Result;
pub struct Command {}
impl Command {
    pub fn run(args: ArgsKind) -> Result<()> {
        Ok(())
    }
}
