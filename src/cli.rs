use std::path::PathBuf;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "tep", about = "A command-line de-autocorrect tool.")]
pub struct Opt {
    /// Input file
    #[structopt(parse(from_os_str))]
    pub input: PathBuf,

    /// Output file, same as input file if not present
    #[structopt(parse(from_os_str))]
    pub output: Option<PathBuf>,
}
