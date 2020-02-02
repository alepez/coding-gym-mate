use std::process::{Command, Stdio, Output};
use std::path::{Path, PathBuf};
use log::{trace, error};
use crate::runner::{Runner, Error};
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

fn run_test(exe: &Path, test_input: &Path) -> Option<String> {
    // TODO Keep information about errors, return Result instead of Option
    let mut cmd = Command::new(exe.to_str()?);

    let mut child = cmd
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn().ok()?;

    let mut file = std::fs::File::open(test_input).ok()?;
    let stdin = child.stdin.as_mut()?;
    std::io::copy(&mut file, stdin).ok()?;

    let Output { stdout, stderr, .. } = child.wait_with_output().ok()?;

    let stdout_str = String::from_utf8(stdout).ok();
    let stderr_str = String::from_utf8(stderr).ok();

    trace!("Output: {:?}", stdout_str);

    if let Some(stderr_str) = stderr_str {
        error!("Error: {}", stderr_str);
    }

    stdout_str
}

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