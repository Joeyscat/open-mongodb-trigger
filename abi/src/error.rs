#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("ObjectId error")]
    OidError(#[from] bson::oid::Error),

    #[error("Database error: {0}")]
    DbError(#[from] mongodb::error::Error),

    #[error("Failed to read configuration file: {0}")]
    ConfigReadError(String),

    #[error("Failed to parse configuration file: {0}")]
    ConfigParseError(String),

    #[error("Unsupported trigger type: {0}")]
    UnsupportedTriggerType(i32),

    #[error("Unsupported function lang: {0}")]
    UnsupportedFunctionLang(i32),

    #[error("Unsupported function lang string: {0}")]
    UnsupportedFunctionLangStr(String),

    #[error("Invalid trigger")]
    InvalidTrigger,

    #[error("Invalid function")]
    InvalidFunction,

    #[error("No trigger found by the given condition")]
    TriggerNotFound,

    #[error("No function found by the given condition")]
    FunctionNotFound,

    #[error("Unknown error")]
    Unknown,
}

impl From<Error> for tonic::Status {
    fn from(e: Error) -> Self {
        match e {
            Error::OidError(_)
            | Error::DbError(_)
            | Error::ConfigReadError(_)
            | Error::ConfigParseError(_)
            | Error::UnsupportedTriggerType(_)
            | Error::UnsupportedFunctionLang(_)
            | Error::UnsupportedFunctionLangStr(_)
            | Error::InvalidTrigger
            | Error::InvalidFunction => tonic::Status::internal(e.to_string()),
            Error::TriggerNotFound => {
                tonic::Status::not_found("No trigger found by the given condition")
            }
            Error::FunctionNotFound => {
                tonic::Status::not_found("No function found by the given condition")
            }
            Error::Unknown => tonic::Status::unknown("unknown error"),
        }
    }
}
