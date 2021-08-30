use std::fs::File;
use std::io::Write;

use csv2json::{cli, parse_delimiter, load, dump};

fn main() -> anyhow::Result<()> {
    let opts = cli::parse();
    let mut builder = csv::ReaderBuilder::new();
    let delimiter = parse_delimiter(opts.delimiter)?;
    builder.delimiter(delimiter);

    let input_file = File::open(opts.input_file)?;
    let output_file: Box<dyn Write> = match opts.output_file {
        Some(path) => Box::new(File::open(path)?),
        None => Box::new(std::io::stdout()),
    };

    let mut reader = builder.from_reader(input_file);
    // TODO: Write records in batches so we don't have to load everything into memory
    let records = load(&mut reader)?;

    dump(records, output_file, opts.indent)
}
