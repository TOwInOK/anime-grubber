use thiserror::Error;
/// Represents the errors that can occur in the application.
///
/// This enum defines various error types that may arise during the execution
/// of the application. It implements the `thiserror::Error` trait, allowing
/// for convenient error handling and formatting.
///
/// # Variants
/// - `NotFound`: An error indicating that a requested resource was not found.
/// - `Reqwest`: An error originating from the `reqwest` crate, typically related
///   to HTTP requests.
/// - `MiniSerde`: An error that occurs during deserialization using the
///   `miniserde` library.
#[derive(Error, Debug)]
pub enum Error {
    #[error("Not found")]
    NotFound,
    #[error("Some reqwest trouble")]
    Reqwest(#[from] reqwest::Error),
    #[error("Desirialise error")]
    MiniSerde(#[from] miniserde::Error),
}
