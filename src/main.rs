use std::convert::TryInto;
use std::path::{Path, PathBuf};

use inotify::{EventMask, Inotify, WatchMask};
use structopt::StructOpt;

use coding_gym_mate::{launch, Language};

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

fn watch_file_and_launch(source: &Path, test: &dyn Fn() -> ()) {
    let mut inotify = Inotify::init().expect("Failed to initialize inotify");

    inotify
        .add_watch(source, WatchMask::MODIFY)
        .expect("Failed to add inotify watch");

    let mut buffer = [0u8; 4096];

    loop {
        inotify
            .read_events_blocking(&mut buffer)
            .expect("Failed to read inotify events")
            .for_each(|event| {
                if event.mask.contains(EventMask::MODIFY) {
                    test();
                }
            });
    }
}

fn main() {
    env_logger::init();

    let opt = Opt::from_args();
    let path = opt.source.as_path();
    let lang_str: Option<&str> = opt.language.as_ref().map(|s| &**s);
    let lang: Option<Language> = (lang_str, path).try_into().ok();

    let Opt {
        source,
        test_input,
        test_output,
        watch,
        ..
    } = opt;

    let test_input = test_input.as_ref().map(|x| x.as_path());
    let test_output = test_output.as_ref().map(|x| x.as_path());
    let source = source.as_path();

    let test = || {
        let result = launch(lang, source, test_input, test_output);

        if result.is_ok() {
            println!("Test Passed");
        } else if let Err(err) = result {
            println!("{}", err);
        }
    };

    test();

    if watch {
        watch_file_and_launch(source, &test);
    }
}
