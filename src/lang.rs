use std::path::{Path, PathBuf};
use std::process::Command;

use crate::runner::{execute_command, Executable, TestError};

pub fn executable_path(source_path: &Path) -> PathBuf {
    let exe = format!("{}.exe", source_path.to_str().unwrap());
    PathBuf::from(&exe)
}

pub fn cc_compile_cmd(program: &str, source: &Path, output: &Path) -> Command {
    let mut command = Command::new(program);
    let source = source.to_str().unwrap();
    let output = output.to_str().unwrap();
    command.arg("-o").arg(output).arg(source);
    command
}

pub fn execute_compiler(cmd: Command, exe: &Path) -> Result<Executable, TestError> {
    execute_command(cmd)
        .and_then(|_| Ok(Executable::new(exe.to_path_buf())))
        .map_err(|err| TestError::CompilerError(err.to_string()))
}
