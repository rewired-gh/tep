/// Overview: Read, process and write.
mod cli;
mod common;

use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read, Write};

use anyhow::Result;
use structopt::StructOpt;

use cli::Opt;
use common::cook::cook;

fn main() -> Result<()> {
    let Opt {
        input,
        output,
        trim,
    } = Opt::from_args();
    let input_file = File::open(&input)?;

    let mut input_content = String::new();
    BufReader::new(&input_file).read_to_string(&mut input_content)?;

    let output_content = cook(&input_content, trim);

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
