use anyhow::{bail, ensure, Context, Result};

use clap::Clap;
use std::fs::File;
use std::io::{ stdin, BufRead, BufReader };
use std::path::PathBuf;
use std::collections::HashMap;

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
}

fn main () -> Result<()> {
    let opts = Opts::parse();

    if let Some(path) = opts.file {
        let f = File::open(path)?;
        let reader = BufReader::new(f);
        run(reader)
    } else {
        bail!("No file detected. Please pass CSV file.")
    }
}

fn run<R: BufRead>(reader: R) -> Result<()> {
    let mut counter: HashMap<String, u32> = HashMap::new();

    for line in reader.lines() {
        let l = line?;
        let word = l.split(",").collect::<Vec<&str>>()[0];

        let count = counter.entry(word.to_string()).or_insert(0);
        *count += 1;
    }

    println!("{:?}", counter);

    Ok(())
}
