use std::path::{PathBuf};

use structopt::StructOpt;
use coding_gym_mate::*;
use std::convert::TryInto;

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
    test_input: Option<PathBuf>,

    /// Test expected output
    #[structopt(long, parse(from_os_str))]
    test_output: Option<PathBuf>,

    /// Source code language
    #[structopt(long)]
    language: Option<String>,
}

fn main() {
    env_logger::init();

    let opt = Opt::from_args();
    let path = opt.source.as_path();
    let lang_str: Option<&str> = opt.language.as_ref().map(|s| &**s);
    let lang: Option<Language> = (lang_str, path).try_into().ok();

    if let Some(lang) = lang {
        use Language::*;

        let Opt { source, test_input, test_output, .. } = opt;

        match lang {
            Rust => test_rust(source.as_path(), test_input, test_output),
            _ => todo!(),
        }
    }
}
