use std::path::Path;

use crate::lang::{cc_compile_cmd, executable_path, execute_compiler};
use crate::runner::{Compiler, Executable, TestError};

pub struct CppCompiler;

impl Default for CppCompiler {
    fn default() -> Self {
        CppCompiler {}
    }
}

impl Compiler for CppCompiler {
    fn compile(&self, source: &Path) -> Result<Executable, TestError> {
        let exe = executable_path(source);
        let cmd = cc_compile_cmd("g++", source, &exe);
        execute_compiler(cmd, &exe)
    }
}