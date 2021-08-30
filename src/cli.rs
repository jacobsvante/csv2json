use std::path::PathBuf;
use clap::Clap;

pub fn parse() -> Opts {
    Opts::parse()
}

/// Convert CSV data to JSON
#[derive(Clap, Debug)]
pub struct Opts {
    #[clap(short, long, parse(from_os_str), default_value = "/dev/stdin")]
    pub input_file: PathBuf,
    /// Where to write output. Defaults to standard output.
    #[clap(short, long, parse(from_os_str))]
    pub output_file: Option<PathBuf>,
    #[clap(long, parse(try_from_str = unescape_chars))]
    pub indent: Option<String>,
    #[clap(short, long, default_value = ",", parse(try_from_str = unescape_char))]
    pub delimiter: char,
}

fn unescape_chars(src: &str) -> anyhow::Result<String> {
    let collected: String = unescape::unescape(src)
        .ok_or_else(|| anyhow::anyhow!("Failed to unescape delimiter"))?
        .chars()
        .collect();
    Ok(collected)
}

fn unescape_char(src: &str) -> anyhow::Result<char> {
    let chars: Vec<char> = unescape_chars(src)?.chars().collect();
    match &chars[..] {
        [c] => Ok(c.to_owned()),
        vec => anyhow::bail!("Needs to be exactly 1 character, not {}", vec.len()),
    }
}
