use std::path::Path;
use crate::Language;
use crate::rust_lang;
use std::process::{Command, Stdio, Output};
use log::{trace, error};

pub enum Error {
    CompileError(String),
    RuntimeError(String),
    MissingExecutable(),
    TestError(String),
}

pub trait Runner {
    fn compile(&mut self, source: &Path, exe: &Path) -> Result<(), Error>;
    fn execute(&self, input_file: &Path) -> Result<String, Error>;
}

pub fn make_runner(lang: Option<Language>) -> Option<Box<dyn Runner>> {
    if let Some(lang) = lang {
        use Language::*;

        let runner = match lang {
            Rust => Box::new(rust_lang::RustRunner::new()),
            _ => todo!(),
        };

        Some(runner)
    } else {
        None
    }
}

pub fn execute_command(mut command: Command) -> Result<(), Box<dyn std::error::Error>> {
    trace!("{:?}", command);
    let status = command
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .status()?;

    // TODO Get stdout and stderr, add output to Result in case of error

    if status.success() {
        Ok(())
    } else {
        Err(format!("Error executing command {:?}", command).into())
    }
}

pub fn run_test(exe: &Path, test_input: &Path) -> Option<String> {
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
        if !stderr_str.is_empty() {
            error!("Error: {}", stderr_str);
        }
    }

    stdout_str
}