use actix_web::error::BlockingError;
use actix_web::{HttpRequest, HttpResponse, Responder};
use diesel::result::DatabaseErrorKind::UniqueViolation;
use diesel::result::Error::{DatabaseError, NotFound};
use std::error;
use std::fmt;
use std::future::{ready, Ready};

#[derive(Debug)]
pub enum AppError {
    RecordAlreadyExists,
    RecordNotFound,
    OperationCanceled,
    InvalidData,
    CSVError(csv::Error),
    DatabaseError(diesel::result::Error),
    IPParsingError(std::net::AddrParseError),
    BadOperation(Box<dyn error::Error>),
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    err: String,
}

impl Responder for AppError {
    type Error = AppError;
    type Future = Ready<Result<HttpResponse, AppError>>;

    fn respond_to(self, _: &HttpRequest) -> Self::Future {
        let formatted_err = format!("{}", self);
        let mut builder = match self {
            AppError::RecordAlreadyExists => HttpResponse::BadRequest(),
            AppError::RecordNotFound => HttpResponse::NotFound(),
            _ => HttpResponse::InternalServerError(),
        };
        ready(Ok(builder.json(ErrorResponse { err: formatted_err })))
    }
}

impl actix_web::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let formatted_err = format!("{}", self);
        let mut builder = match self {
            AppError::RecordAlreadyExists => HttpResponse::BadRequest(),
            AppError::RecordNotFound => HttpResponse::NotFound(),
            _ => HttpResponse::InternalServerError(),
        };
        builder.json(ErrorResponse { err: formatted_err })
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::RecordAlreadyExists => write!(f, "This record violates a unique constraints"),
            AppError::InvalidData => write!(f, "invalid data is provided"),
            AppError::RecordNotFound => write!(f, "This record does not exists"),
            AppError::OperationCanceled => write!(f, "The running operation was canceled"),
            AppError::DatabaseError(e) => write!(f, "Database error: {:?}", e),
            AppError::BadOperation(e) => write!(f, "The operation failed with an error: {:?}", e),
            AppError::CSVError(e) => write!(f, "Failed to read csv file: {:?}", e),
            AppError::IPParsingError(e) => write!(f, "Failed to parse provided ip: {:}", e),
        }
    }
}

impl From<std::net::AddrParseError> for AppError {
    fn from(e: std::net::AddrParseError) -> Self {
        match e {
            _ => AppError::IPParsingError(e),
        }
    }
}

impl From<csv::Error> for AppError {
    fn from(e: csv::Error) -> Self {
        match e {
            _ => AppError::CSVError(e),
        }
    }
}

impl From<Box<dyn error::Error>> for AppError {
    fn from(e: Box<dyn error::Error>) -> Self {
        match e {
            _ => AppError::BadOperation(e),
        }
    }
}

impl From<diesel::result::Error> for AppError {
    fn from(e: diesel::result::Error) -> Self {
        match e {
            DatabaseError(UniqueViolation, _) => AppError::RecordAlreadyExists,
            NotFound => AppError::RecordNotFound,
            _ => AppError::DatabaseError(e),
        }
    }
}

impl From<BlockingError<AppError>> for AppError {
    fn from(e: BlockingError<AppError>) -> Self {
        match e {
            BlockingError::Error(inner) => inner,
            BlockingError::Canceled => AppError::OperationCanceled,
        }
    }
}
