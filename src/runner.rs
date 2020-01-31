use std::path::Path;
use crate::Language;
use crate::rust_lang;

pub enum Error {
    CompileError(String),
    RuntimeError(String),
    TestError(String),
}

pub trait Runner {
    fn compile(&self, source: &Path, exe: &Path) -> Result<(), Error>;
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