// use std::fmt::Display;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("Invalid input")]
    ValidationError,
}

// MEMO: Errorトレイトの導出に必要
// impl Display for DomainError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{self:?}")
//     }
// }
