#[derive(thiserror::Error, Debug)]
pub enum RuccoReplErr {
    #[error("EmptyInput")]
    EmptyInput,
}

#[derive(thiserror::Error, Debug)]
pub enum RuccoReaderErr {
    #[error("UnexpectedEof")]
    UnexpectedEof,
}
