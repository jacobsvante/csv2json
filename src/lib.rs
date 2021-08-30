use std::{collections::HashMap, convert::TryFrom, io};
pub mod cli;
use serde_json::Serializer;
use serde::ser::*;

type Record = HashMap<String, String>;


pub fn load<R: io::Read>(reader: &mut csv::Reader<R>) -> anyhow::Result<Vec<Record>> {
    Ok(reader.deserialize().collect::<Result<_, csv::Error>>()?)
}


pub fn dump<W: io::Write>(records: Vec<Record>, output_file: W, indent: Option<String>) -> anyhow::Result<()> {
    match indent {
        Some(indent) => {
            let formatter = serde_json::ser::PrettyFormatter::with_indent(indent.as_bytes());
            let mut ser = Serializer::with_formatter(output_file, formatter);
            records.serialize(&mut ser)?;
        },
        None => {
            let mut ser = Serializer::new(output_file);
            records.serialize(&mut ser)?;
        },
    };
    Ok(())
}


pub fn parse_delimiter(d: char) -> anyhow::Result<u8> {
    u8::try_from(u32::from(d)).map_err(anyhow::Error::from)
}
