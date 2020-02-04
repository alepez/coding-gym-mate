use std::process::Command;
use crate::runner::{Compiler, Executable, execute_command};
use crate::runner::Error as RunnerError;
use std::path::Path;

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

pub struct RustCompiler;

impl RustCompiler {
    pub fn new() -> Self {
        RustCompiler {}
    }
}

impl Compiler for RustCompiler {
    fn compile(&self, source: &Path) -> Result<Executable, RunnerError> {
        let exe = format!("{}.exe", source.to_str().unwrap());
        let exe = Path::new(&exe);

        let cmd = compile_cmd(source, exe);

        let result = execute_command(cmd);
        if result.is_ok() {
            Ok(Executable::new(exe.to_path_buf()))
        } else {
            log::error!("Compiler error: {:?}", result.unwrap_err());
            Err(RunnerError::CompileError("Unknown compiler error".into()))
        }
    }
}