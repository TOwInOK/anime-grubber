use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Not found")]
    NotFound,
    #[error("Some reqwest trouble")]
    Reqwest(#[from] reqwest::Error),
    #[error("Desirialise error")]
    MiniSerde(#[from] miniserde::Error),
}
