use std::fmt;

#[derive(Debug)]
pub struct ParseVersionError {}

impl fmt::Display for ParseVersionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unable to parse version")
    }
}

impl std::error::Error for ParseVersionError {}

#[derive(Debug)]
pub enum SkillTreeUrlError {
    Decode,
    UnknownVersion(u32),
    Eof,
}

impl fmt::Display for SkillTreeUrlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Decode => write!(f, "unable to decode skill tree url"),
            Self::UnknownVersion(version) => {
                write!(f, "version {version} unknown in skill tree url")
            }
            Self::Eof => write!(f, "unexpected eof while parsing skill tree url"),
        }
    }
}

impl std::error::Error for SkillTreeUrlError {}
