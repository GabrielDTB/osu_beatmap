use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("failed to parse {token:?} as {type_name:?}")]
    InvalidToken { token: String, type_name: String },
    #[error("missing section {section_name:?} in .osu file")]
    MissingSection { section_name: String },
    #[error("invalid line in {section:?} {line:?}")]
    InvalidLine { line: String, section: String },
    #[error("missing field {field:?} for collection {collection:?}")]
    MissingField { field: String, collection: String },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
