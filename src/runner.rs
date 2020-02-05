use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

use log::{error, trace};

use crate::cpp_lang;
use crate::rust_lang;
use crate::Language;

pub trait Compiler {
    fn compile(&self, source: &Path) -> Result<Executable, TestError>;
}

pub struct Executable(PathBuf);

impl Executable {
    pub fn execute(&self, input_file: &Path) -> Result<ActualOutput, TestError> {
        let output = run_test(&self.0, input_file);
        // FIXME Add info from stderr
        output
            .ok_or(TestError::RuntimeError("FIXME".into()))
            .map(|output| ActualOutput(output))
    }

    pub fn new(path: PathBuf) -> Self {
        Executable(path)
    }
}

#[derive(Debug)]
pub struct ActualOutput(String);

#[derive(Debug)]
pub struct ExpectedOutput(String);

#[derive(Debug)]
pub enum TestError {
    InvalidLanguage,
    MissingInput,
    CompilerError(String),
    RuntimeError(String),
    OutputMismatch(ExpectedOutput, ActualOutput),
    ManualCheck(ActualOutput),
}

impl std::fmt::Display for TestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            TestError::InvalidLanguage => write!(f, "Invalid language"),
            TestError::MissingInput => write!(f, "Missing input"),
            TestError::CompilerError(e) => write!(f, "Compiler error: {:?}", e),
            TestError::RuntimeError(e) => write!(f, "Runtime error: {:?}", e),
            TestError::OutputMismatch(expected, actual) => write!(f, "Expected:\n{}\nActual:\n{}", expected.0, actual.0),
            TestError::ManualCheck(actual) => write!(f, "Actual:\n{}", actual.0),
        }
    }
}

impl ExpectedOutput {
    pub fn check(&self, test_output: &ActualOutput) -> bool {
        self.0 == test_output.0
    }

    pub fn new(s: String) -> ExpectedOutput {
        ExpectedOutput(s)
    }
}

pub fn make_compiler(lang: Option<Language>) -> Option<Box<dyn Compiler>> {
    if let Some(lang) = lang {
        use Language::*;

        let compiler: Box<dyn Compiler> = match lang {
            Rust => Box::new(rust_lang::RustCompiler::default()),
            CPlusPlus => Box::new(cpp_lang::CppCompiler::default()),
        };

        Some(compiler)
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
        .spawn()
        .ok()?;

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
