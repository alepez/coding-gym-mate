use std::path::{PathBuf, Path};

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

    let Opt { source, test_input, test_output, .. } = opt;
    let mut runner = runner::make_runner(lang);

    let exe = format!("{}.exe", source.to_str().unwrap());
    let exe = Path::new(&exe);

    if let Some(mut runner) = runner {
        runner.compile(&source, exe).ok().unwrap();

        if let Some(input_file) = test_input {
            let output_file = Path::new("FIXME");
            // FIXME handle error
            runner.execute(&input_file, output_file).ok().unwrap();
        }
    }
}
