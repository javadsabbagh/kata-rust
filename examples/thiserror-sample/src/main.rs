use thiserror::Error;

use crate::ServiceCallError::{ApplicationError, ConnectionError};

#[derive(Error, Debug)]
pub enum ServiceCallError {
    #[error("Remote service call returned with error (rejected the request)")]
    ServiceError(),
    #[error("There was a connection error, e.g. timeout, token error, gateway error, etc.")]
    ConnectionError,
    #[error("There was an error (bug) in our application. Cause: `{0}`")]
    ApplicationError(String),
}

#[derive(Error, Debug)]
pub enum TokenError {
    #[error("Invalid token provided")]
    InvalidToken,
    #[error("This token is expired, code: `{0}`")]
    ExpiredToken(i32),
    #[error("This token doesn't have access to the resource, code: `{0}`")]
    UnAuthorizedAccess(i32),
}


fn main() {
    println!("Hello, world!");

    let result = Err::<(), ServiceCallError>(ApplicationError("Division by zero".to_string()));
    eprintln!("{:?}", result);

    match result {
        Err(ApplicationError(_)) => println!("Got an application error"),
        Ok(()) => println!("Success"),
        _ => println!("Other error types")
    }
}
