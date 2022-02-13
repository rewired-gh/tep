use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read, Write};

use anyhow::Result;
use structopt::StructOpt;

use crate::cli::Opt;
use crate::cook::cook;

mod cli;
mod cook;
mod node;
mod rule;
mod tree;

fn main() -> Result<()> {
    let Opt { input, output } = Opt::from_args();
    let input_file = File::open(&input)?;

    let mut input_content = String::new();
    BufReader::new(&input_file).read_to_string(&mut input_content)?;

    let output_content = cook(&input_content);

    let mut output_file;
    match output {
        Some(file) => {
            output_file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(file)?
        }
        None => output_file = OpenOptions::new().write(true).truncate(true).open(&input)?,
    }
    write!(output_file, "{}", output_content)?;

    Ok(())
}
