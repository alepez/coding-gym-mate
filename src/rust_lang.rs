use std::process::{Command};
use std::path::{Path, PathBuf};
use crate::runner::{Runner, Error, execute_command, run_test};
use crate::runner::Error as RunnerError;

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
        let result = execute_command(cmd);
        if result.is_ok() {
            self.exe = Some(Box::new(exe.into()));
            Ok(())
        } else {
            log::error!("Compiler error: {:?}", result.unwrap_err());
            Err(RunnerError::CompileError("Unknown compiler error".into()))
        }
    }

    fn execute(&self, input_file: &Path) -> Result<String, Error> {
        if let Some(exe) = &self.exe {
            let output = run_test(&exe, input_file);
            // FIXME Add info from stderr
            output.ok_or(Error::RuntimeError("FIXME".into()))
        } else {
            Err(Error::MissingExecutable())
        }
    }
}