use std::convert::{TryFrom, TryInto};
use std::path::{Path, PathBuf};

mod rust_lang;
mod runner;

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
            _ => Err(())
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

pub fn launch(lang: Option<Language>, source: PathBuf, test_input: Option<PathBuf>, test_output: Option<PathBuf>) -> Option<bool> {
    let runner = runner::make_runner(lang);
    let mut runner = runner?;

    let exe = format!("{}.exe", source.to_str().unwrap());
    let exe = Path::new(&exe);

    runner.compile(&source, exe).ok()?;

    // TODO optimization: instead of string, get a stream
    let test_input = test_input?;
    let actual_output = runner.execute(&test_input).ok()?;

    let test_output = test_output?;
    let expected_output = std::fs::read_to_string(test_output).ok()?;

    Some(actual_output == expected_output)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::convert::TryInto;
    use std::path::PathBuf;

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
}