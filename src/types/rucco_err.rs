#[derive(thiserror::Error, Debug)]
pub enum RuccoErr {
    #[error("ReplEmptyError")]
    ReplEmptyError,
}
