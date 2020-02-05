use std::convert::TryInto;
use std::env;
use std::path::{Path, PathBuf};

use inotify::{
    EventMask,
    Inotify,
    WatchMask,
};
use structopt::StructOpt;

use coding_gym_mate::{Language, launch};

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

fn init_inotify(path: &Path) -> Inotify {
    let mut inotify = Inotify::init()
        .expect("Failed to initialize inotify");

    inotify
        .add_watch(
            path,
            WatchMask::MODIFY,
        )
        .expect("Failed to add inotify watch");

    inotify
}

// TODO Make paths absolute
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

    let mut inotify = init_inotify(&source);

    let once = || { launch(lang, &source, test_input, test_output) };

    loop {
        let result = once();

        if result.is_ok() {
            println!("Test Passed");
        } else {
            println!("{:?}", result);
        }

        if !watch {
            println!("Finished");
            break;
        }

        println!("Watch for changes {:?}", &source);

        let mut buffer = [0u8; 4096];

        let events = inotify
            .read_events_blocking(&mut buffer)
            .expect("Failed to read inotify events");

        println!("1");
        for event in events {
            println!("2");
            if event.mask.contains(EventMask::CREATE) {
                if event.mask.contains(EventMask::ISDIR) {
                    println!("Directory created: {:?}", event.name);
                } else {
                    println!("File created: {:?}", event.name);
                }
            } else if event.mask.contains(EventMask::DELETE) {
                if event.mask.contains(EventMask::ISDIR) {
                    println!("Directory deleted: {:?}", event.name);
                } else {
                    println!("File deleted: {:?}", event.name);
                }
            } else if event.mask.contains(EventMask::MODIFY) {
                if event.mask.contains(EventMask::ISDIR) {
                    println!("Directory modified: {:?}", event.name);
                } else {
                    println!("File modified: {:?}", event.name);
                }
            }
        }
    }
}
