#![feature(matches_macro)]

use std::convert::{TryFrom, TryInto};
use std::path::Path;

use runner::ExpectedOutput;

use crate::runner::TestError;

mod cpp_lang;
mod lang;
mod runner;
mod rust_lang;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Language {
    Rust,
    CPlusPlus,
}

impl TryFrom<&str> for Language {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use Language::*;

        match value {
            "rust" | "rs" => Ok(Rust),
            "cplusplus" | "cpp" | "cxx" => Ok(CPlusPlus),
            _ => Err(()),
        }
    }
}

impl TryFrom<&Path> for Language {
    type Error = ();

    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        let extension = value.extension().ok_or(())?;
        let extension = extension.to_str().ok_or(())?;
        extension.try_into()
    }
}

impl TryFrom<(Option<&str>, &Path)> for Language {
    type Error = ();

    fn try_from((lang, path): (Option<&str>, &Path)) -> Result<Self, Self::Error> {
        if let Some(lang) = lang {
            lang.try_into()
        } else {
            path.try_into()
        }
    }
}

pub fn launch(
    lang: Option<Language>,
    source: &Path,
    test_input: Option<&Path>,
    test_output: Option<&Path>,
) -> Result<(), TestError> {
    let source = std::fs::canonicalize(source).expect("Invalid source file");
    let test_input = test_input.and_then(|x| std::fs::canonicalize(x).ok());
    let test_output = test_output.and_then(|x| std::fs::canonicalize(x).ok());

    runner::make_compiler(lang)
        .ok_or(TestError::InvalidLanguage)
        .and_then(|compiler| compiler.compile(&source))
        .and_then(|exe| {
            let test_input = test_input.ok_or(TestError::MissingInput)?;
            exe.execute(&test_input)
        })
        .and_then(|actual_output| {
            if let Some(test_output) = test_output {
                let expected_output = std::fs::read_to_string(test_output).unwrap();
                let expected_output = ExpectedOutput::new(expected_output);
                Ok((expected_output, actual_output))
            } else {
                Err(TestError::ManualCheck(actual_output))
            }
        })
        .and_then(|(expected_output, actual_output)| {
            if expected_output.check(&actual_output) {
                Ok(())
            } else {
                Err(TestError::OutputMismatch(expected_output, actual_output))
            }
        })
}

#[cfg(test)]
mod test {
    use std::convert::TryInto;
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_language_from_str() {
        let lang: Option<Language> = "rs".try_into().ok();
        assert_eq!(lang, Some(Language::Rust));
    }

    #[test]
    fn test_language_from_filename() {
        let lang: Option<Language> = PathBuf::from("main.rs").as_path().try_into().ok();
        assert_eq!(lang, Some(Language::Rust));
    }

    #[test]
    fn test_language_from_str_or_filename() {
        let path = PathBuf::from("main.rs");
        let pair = (None, path.as_path());
        let lang: Option<Language> = pair.try_into().ok();
        assert_eq!(lang, Some(Language::Rust));
    }

    #[test]
    fn test_language_from_str_overrides_filename() {
        let path = PathBuf::from("main.rs");
        let pair = (Some("cpp"), path.as_path());
        let lang: Option<Language> = pair.try_into().ok();
        assert_eq!(lang, Some(Language::CPlusPlus));
    }

    #[test]
    fn test_launch() {
        let lan = Some(Language::Rust);
        let source = PathBuf::from("test_samples/reverse.rs");
        let test_input = PathBuf::from("test_samples/input.txt");
        let test_output = PathBuf::from("test_samples/output.txt");

        let err = launch(lan, &source, Some(&test_input), Some(&test_output));
        assert_eq!(err,Ok(()));
    }

    #[test]
    fn test_launch_no_input() {
        let lan = Some(Language::CPlusPlus);
        let source = PathBuf::from("test_samples/nop.cpp");
        let test_input = PathBuf::from("test_samples/wrong_file.txt");
        let test_output = PathBuf::from("test_samples/output.txt");

        let err = launch(lan, &source, Some(&test_input), Some(&test_output));
        assert_eq!(err, Err(TestError::MissingInput));
    }

    #[test]
    fn test_launch_runtime_error() {
        let lan = Some(Language::Rust);
        let source = PathBuf::from("test_samples/runtime_error.rs");
        let test_input = PathBuf::from("test_samples/input.txt");
        let test_output = PathBuf::from("test_samples/output.txt");

        let err = launch(lan, &source, Some(&test_input), Some(&test_output));
        assert!(matches!(err, Err(TestError::RuntimeError(_))));
    }

    #[test]
    fn test_launch_compile_error() {
        let lan = Some(Language::Rust);
        let source = PathBuf::from("test_samples/compile_error.rs");
        let test_input = PathBuf::from("test_samples/input.txt");
        let test_output = PathBuf::from("test_samples/output.txt");

        let err = launch(lan, &source, Some(&test_input), Some(&test_output));
        assert!(matches!(err, Err(TestError::CompilerError(_))));
    }

    #[test]
    fn test_launch_manual_check() {
        let lan = Some(Language::Rust);
        let source = PathBuf::from("test_samples/nop.rs");
        let test_input = PathBuf::from("test_samples/input.txt");

        let err = launch(lan, &source, Some(&test_input), None);
        assert!(matches!(err, Err(TestError::ManualCheck(_))));
    }
}
