use thiserror::Error;

#[derive(Error, Debug)]
pub enum PacinfoError {
    /// Given package is not present in database
    #[error("package not found")]
    PackageNotFound,

    /// Pacman database failed to initialize
    #[error("could not initialize pacman db: `{0}`")]
    PacmanDbInit(#[from] alpm::Error),

    /// Unknown cases
    #[error("unknown error")]
    Unknown,
}
