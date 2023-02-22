use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("split delimiter {delimiter:?} not found in {string:?}")]
    SplitError { string: String, delimiter: char },
    #[error("failed to parse {token:?} as {type_name:?}")]
    InvalidToken { token: String, type_name: String },
    #[error("missing section {section_name:?} in .osu file")]
    MissingSection { section_name: String },
    #[error("invalid line in {section:?} {line:?}")]
    InvalidLine { line: String, section: String },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
