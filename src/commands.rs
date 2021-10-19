use crate::args::{self, ArgsKind, DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::chunk::Chunk;
use crate::png::Png;
use crate::Result;
use std::convert::TryFrom;
use std::fs::File;
use std::io::Read;
use std::io::Write;
pub struct Command {}
impl Command {
    pub fn run(args: ArgsKind) -> Result<()> {
        match args {
            ArgsKind::Encode(EncodeArgs {
                file_path,
                chunk_type,
                message,
                output_path,
            }) => {
                let mut png = Png::from_file(&file_path)?;
                png.append_chunk(Chunk::new(chunk_type, message.into_bytes()));
                png.to_file(&output_path)
            }
            ArgsKind::Decode(DecodeArgs {
                file_path,
                chunk_type,
            }) => {
                let png = Png::from_file(&file_path)?;
                let msg = png.chunk_by_type(&chunk_type.to_string()).unwrap(); //TODO maybe error here
                println!("{}", msg);
                Ok(())
            }
            ArgsKind::Remove(RemoveArgs {
                file_path,
                chunk_type,
            }) => {
                let mut png = Png::from_file(&file_path)?;
                let chunk_removed = png.remove_chunk(&chunk_type.to_string())?;
                println!(
                    "Removed chunk: {} from file {}",
                    chunk_removed,
                    file_path.to_str().unwrap() // TODO maybe error here
                );
                png.to_file(&file_path)?;
                Ok(())
            }
            ArgsKind::Print(PrintArgs { file_path }) => {
                let png = Png::from_file(&file_path)?;
                println!("{}", png);
                Ok(())
            }
        }
    }
}
