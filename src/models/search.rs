use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResultItem {
    pub title: String,
    pub url: String,
    pub body: String,
}
