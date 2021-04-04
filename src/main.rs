mod words_counter;
use anyhow::{bail, Result};

use clap::Clap;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use crate::words_counter::counter;

#[derive(Clap, Debug)]
#[clap(
    name = "words counter",
    version = "1.0.0",
    author = "pipopotamasu",
    about = "Count up words from CSV file."
)]

struct Opts {
    #[clap(name = "FILE")]
    file: Option<PathBuf>,

    #[clap(short = 's', long)]
    skip_header: bool,
}

fn main () -> Result<()> {
    let opts = Opts::parse();

    if let Some(path) = opts.file {
        let f = File::open(path)?;
        let reader = BufReader::new(f);
        counter::run(reader, opts.skip_header)
    } else {
        bail!("No file detected. Please pass CSV file.")
    }
}
