use std::convert::{TryFrom, TryInto};

mod rust_lang;
pub mod runner;

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

//pub use rust_lang::test as test_rust;
use std::path::Path;

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