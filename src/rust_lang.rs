use std::process::{Command, Stdio};
use std::path::{Path, PathBuf};
use log::trace;
use crate::runner::Runner;
use crate::runner::Error as RunnerError;

fn execute(mut command: Command) -> Result<(), Box<dyn std::error::Error>> {
    trace!("{:?}", command);
    let status = command
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("Error executing command {:?}", command).into())
    }
}


fn compile_cmd(source: &Path, output: &Path) -> Command {
    let mut command = Command::new("rustc");
    let source = source.to_str().unwrap();
    let output = output.to_str().unwrap();
    command
        .arg("-o")
        .arg(output)
        .arg(source);
    command
}

fn test_cmd(exe: &Path, test_input: &Path) -> Command {
    let mut command = Command::new(exe.to_str().unwrap());
    let test_input = test_input.to_str().unwrap();
    command
        .arg("-i")
        .arg(test_input);

    dbg!(command)
}

fn run_test(exe: &Path, test_input: &Path) -> Option<String> {
//    let input_process = match test_cmd(exe, test_input)
//        .stdin(Stdio::piped())
//        .stdout(Stdio::piped())
//        .spawn() {
//        Err(why) => panic!("couldn't spawn {:?}: {:?}", cmd, why),
//        Ok(process) => process,
//    };
//
//    let test_process = match test_cmd(exe, test_input)
//        .stdin(Stdio::piped())
//        .stdout(Stdio::piped())
//        .spawn() {
//        Err(why) => panic!("couldn't spawn {:?}: {:?}", cmd, why),
//        Ok(process) => process,
//    };
//
    Some(String::new())
}

pub fn test(source: &Path, test_input: Option<PathBuf>, test_output: Option<PathBuf>) {
    let output = format!("{}.exe", source.to_str().unwrap());
    let output = Path::new(&output);
    let output = execute(compile_cmd(source, output)).ok().map(|_| output);

    if let Some(output) = output {
        if let Some(test_input) = &test_input {
            let t = run_test(output, test_input);
        }
    }
}

pub struct RustRunner {}

impl RustRunner {
    pub fn new() -> Self {
        RustRunner {}
    }
}

impl Runner for RustRunner {
    fn compile(&self, source: &Path, exe: &Path) -> Result<(), RunnerError> {
        let cmd = compile_cmd(source, exe);
        let result = execute(cmd);
        if result.is_ok() {
            Ok(())
        } else {
            Err(RunnerError::CompileError("Unknown compiler error".into()))
        }
    }
}