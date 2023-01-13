use thiserror::Error;

#[derive(Error, debug)]
pub enum Errors {
    #[error("this aint ma guy")]
    TestError,
}