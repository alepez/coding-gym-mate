use std::path::Path;

use crate::lang::{cc_compile_cmd, executable_path, execute_compiler};
use crate::runner::{Compiler, Executable, TestError};

pub struct RustCompiler;

impl Default for RustCompiler {
    fn default() -> Self {
        RustCompiler {}
    }
}

impl Compiler for RustCompiler {
    fn compile(&self, source: &Path) -> Result<Executable, TestError> {
        let exe = executable_path(source);
        let cmd = cc_compile_cmd("rustc", source, &exe);
        execute_compiler(cmd, &exe)
    }
}
