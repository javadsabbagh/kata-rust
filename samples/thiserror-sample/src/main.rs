use crate::ServiceCallError::ApplicationError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceCallError {
    #[error("Remote service call returned with error (rejected the request)")]
    ServiceError(),
    #[error("There was a connection error, e.g. timeout, token error, gateway error, etc.")]
    ConnectionError,
    #[error("There was an error (bug) in our application. Cause: `{0}`")]
    ApplicationError(String),
}

fn main() {
    println!("Hello, world!");

    let result = Err::<(), ServiceCallError>(ApplicationError("Division by zero".to_string()));
    println!("{:?}", result)
}
