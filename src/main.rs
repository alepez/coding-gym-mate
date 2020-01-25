use std::fs::canonicalize;
use std::path::PathBuf;

use log::trace;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "coding-gym-mate")]
struct Opt {
    /// Watch files for changes and test automatically
    #[structopt(short, long)]
    watch: bool,

    /// Source code
    #[structopt(long, parse(from_os_str))]
    source: PathBuf,

    /// Test input
    #[structopt(long, parse(from_os_str))]
    test_input: PathBuf,

    /// Test expected output
    #[structopt(long, parse(from_os_str))]
    test_output: PathBuf,

    /// Source code language
    #[structopt(long)]
    language: Option<String>,
}

fn main() {
    env_logger::init();

    let opt = Opt::from_args();

    log::trace!("{:?}", opt);
}
