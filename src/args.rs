use crate::chunk_type::ChunkType;
use clap::{App, AppSettings, Arg, SubCommand};
use std::{fmt::Display, path::Path, str::FromStr};
#[derive(Debug)]
pub enum ArgsParseErr {
    UnknownArgument,
}
impl std::error::Error for ArgsParseErr {}

impl Display for ArgsParseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArgsParseErr::UnknownArgument => write!(f, "Unkown argument found"),
        }
    }
}
pub enum ArgsKind {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}
fn is_chunk_type_valid(chunk_type: String) -> Result<(), String> {
    match ChunkType::from_str(&chunk_type) {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from(
            "Chunk type not valid. All 4 bytes must be from alphabet.",
        )),
    }
}
pub struct EncodeArgs {
    pub file_path: Box<Path>,
    pub chunk_type: ChunkType,
    pub message: String,
    pub output_path: Box<Path>,
}

pub struct DecodeArgs {
    pub file_path: Box<Path>,
    pub chunk_type: ChunkType,
}

pub struct RemoveArgs {
    pub file_path: Box<Path>,
    pub chunk_type: ChunkType,
}

pub struct PrintArgs {
    pub file_path: Box<Path>,
}
impl EncodeArgs {
    pub fn new(
        file_path: &str,
        chunk_type: &str,
        message: &str,
        output_path: &str,
    ) -> crate::Result<Self> {
        Ok(Self {
            file_path: Box::from(Path::new(file_path)),
            chunk_type: ChunkType::from_str(chunk_type)?,
            message: message.to_string(),
            output_path: Box::from(Path::new(output_path)),
        })
    }
}
impl DecodeArgs {
    pub fn new(file_path: &str, chunk_type: &str) -> crate::Result<Self> {
        Ok(Self {
            file_path: Box::from(Path::new(file_path)),
            chunk_type: ChunkType::from_str(chunk_type)?,
        })
    }
}
impl RemoveArgs {
    pub fn new(file_path: &str, chunk_type: &str) -> crate::Result<Self> {
        Ok(Self {
            file_path: Box::from(Path::new(file_path)),
            chunk_type: ChunkType::from_str(chunk_type)?,
        })
    }
}
impl PrintArgs {
    pub fn new(file_path: &str) -> crate::Result<Self> {
        Ok(Self {
            file_path: Box::from(Path::new(file_path)),
        })
    }
}
pub struct Config {}
impl Config {
    pub fn new() -> Self {
        Self {}
    }
    pub fn parse_args() -> crate::Result<ArgsKind> {
        let matches = App::new("PNGme")
            .version("1.0")
            .author("plaxi0s")
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .about("RRCU {Random Rust Coding Urges}")
            .subcommand(
                SubCommand::with_name("encode")
                    .about("Encodes a message into a .png file")
                    .arg(
                        Arg::with_name("file_path")
                            .required(true)
                            .help("Path of .png file")
                            .short("f")
                            .index(1),
                    )
                    .arg(
                        Arg::with_name("chunk_type")
                            .required(true)
                            .help("Chunk type")
                            .short("t")
                            .index(2)
                            .validator(is_chunk_type_valid),
                    )
                    .arg(
                        Arg::with_name("message")
                            .required(true)
                            .help("Message to encode")
                            .short("m")
                            .index(3),
                    )
                    .arg(
                        Arg::with_name("output_path")
                            .required(true)
                            .help("Output path for png")
                            .short("o")
                            .index(4),
                    ),
            )
            .subcommand(
                SubCommand::with_name("decode")
                    .about("Decodes a message from a .png file")
                    .arg(
                        Arg::with_name("file_path")
                            .required(true)
                            .help("Path of .png file")
                            .short("f")
                            .index(1),
                    )
                    .arg(
                        Arg::with_name("chunk_type")
                            .required(true)
                            .help("Chunk type")
                            .short("t")
                            .index(2)
                            .validator(is_chunk_type_valid),
                    ),
            )
            .subcommand(
                SubCommand::with_name("remove")
                    .about("Removes a chunk_type rom a .png file")
                    .arg(
                        Arg::with_name("file_path")
                            .required(true)
                            .help("Path of .png file")
                            .short("f")
                            .index(1),
                    )
                    .arg(
                        Arg::with_name("chunk_type")
                            .required(true)
                            .help("Chunk type")
                            .short("t")
                            .index(2)
                            .validator(is_chunk_type_valid),
                    ),
            )
            .subcommand(
                SubCommand::with_name("print")
                    .about("Print a .png file")
                    .arg(
                        Arg::with_name("file_path")
                            .required(true)
                            .help("Path of .png file")
                            .short("f")
                            .index(1),
                    ),
            )
            .get_matches();
        match matches.subcommand() {
            ("encode", Some(m)) => Ok(ArgsKind::Encode(EncodeArgs::new(
                m.value_of("file_path").unwrap(),
                m.value_of("chunk_type").unwrap(),
                m.value_of("message").unwrap(),
                m.value_of("output_path").unwrap(),
            )?)),
            ("decode", Some(m)) => Ok(ArgsKind::Decode(DecodeArgs::new(
                m.value_of("file_path").unwrap(),
                m.value_of("chunk_type").unwrap(),
            )?)),
            ("remove", Some(m)) => Ok(ArgsKind::Remove(RemoveArgs::new(
                m.value_of("file_path").unwrap(),
                m.value_of("chunk_type").unwrap(),
            )?)),
            ("print", Some(m)) => Ok(ArgsKind::Print(PrintArgs::new(
                m.value_of("file_path").unwrap(),
            )?)),
            _ => Err(Box::new(ArgsParseErr::UnknownArgument)),
        }
    }
}
