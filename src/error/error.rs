use crate::{point::fetch::FetchError, utils::NestedParseError};

#[derive(Debug, thiserror::Error)]
pub enum AssetPairError {
    #[error(transparent)]
    Fetch(#[from] FetchError),

    #[error(transparent)]
    Parse(#[from] NestedParseError),
}
