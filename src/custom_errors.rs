use thiserror::Error;

#[derive(Error, Debug)]

pub enum Errors {
    #[error("this aint ma guy")]
    TestError,
}