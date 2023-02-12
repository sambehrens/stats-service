use aws_sdk_dynamodb::types::SdkError;
use std::fmt::{Debug, Formatter};

pub enum QueryError {
    Db(SdkError<aws_sdk_dynamodb::error::QueryError>),
    Parse(std::num::ParseIntError),
}

impl From<std::num::ParseIntError> for QueryError {
    fn from(err: std::num::ParseIntError) -> QueryError {
        QueryError::Parse(err)
    }
}

impl From<SdkError<aws_sdk_dynamodb::error::QueryError>> for QueryError {
    fn from(err: SdkError<aws_sdk_dynamodb::error::QueryError>) -> QueryError {
        QueryError::Db(err)
    }
}

impl Debug for QueryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Db(e) => e.fmt(f),
            Self::Parse(e) => e.fmt(f),
        }
    }
}
