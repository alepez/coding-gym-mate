use std::process::{Command, Stdio, Child, ChildStdout, ChildStderr, Output};
use std::path::{Path, PathBuf};
use log::{trace, error};
use crate::runner::{Runner, Error};
use crate::runner::Error as RunnerError;
use std::io::{BufReader, Read, Write};

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
    Command::new(exe.to_str().unwrap())
}

fn get_stdout_as_string(r: ChildStdout) -> String {
    let reader = BufReader::new(r);
    let mut s = String::new();
    reader.buffer().read_to_string(&mut s);
    s
}

fn get_stderr_as_string(r: ChildStderr) -> String {
    let reader = BufReader::new(r);
    let mut s = String::new();
    reader.buffer().read_to_string(&mut s);
    s
}

fn run_test(exe: &Path, test_input: &Path) -> Option<String> {
    let mut cmd = test_cmd(exe, test_input);
    let mut child = match cmd
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn() {
        Err(why) => panic!("couldn't spawn {:?}: {:?}", exe, why),
        Ok(process) => process,
    };

    {
        let child_stdin = child.stdin.as_mut().unwrap();
        child_stdin.write_all(b"Hello, world!\n").ok().unwrap();
    }

    let output = child.wait_with_output().ok()?;

    let Output { stdout, stderr, .. } = output;
    let stdout_str = String::from_utf8(stdout).ok();
    let stderr_str = String::from_utf8(stderr).ok();

    trace!("Output: {:?}", stdout_str);
    error!("Error: {:?}", stderr_str);

    stdout_str
}

//pub fn test(source: &Path, test_input: Option<PathBuf>, test_output: Option<PathBuf>) {
//    let output = format!("{}.exe", source.to_str().unwrap());
//    let output = Path::new(&output);
//    let output = execute(compile_cmd(source, output)).ok().map(|_| output);
//
//    if let Some(output) = output {
//        if let Some(test_input) = &test_input {
//            let t = run_test(output, test_input);
//        }
//    }
//}

pub struct RustRunner {
    exe: Option<Box<PathBuf>>,
}

impl RustRunner {
    pub fn new() -> Self {
        RustRunner {
            exe: None,
        }
    }
}

impl Runner for RustRunner {
    fn compile(&mut self, source: &Path, exe: &Path) -> Result<(), RunnerError> {
        let cmd = compile_cmd(source, exe);
        let result = execute(cmd);
        if result.is_ok() {
            self.exe = Some(Box::new(exe.into()));
            Ok(())
        } else {
            log::error!("Compiler error: {:?}", result.unwrap_err());
            Err(RunnerError::CompileError("Unknown compiler error".into()))
        }
    }

    fn execute(&self, input_file: &Path, _output_file: &Path) -> Result<(), Error> {
        if let Some(exe) = &self.exe {
            run_test(&exe, input_file);
            Ok(())
        } else {
            Err(Error::MissingExecutable())
        }
    }
}