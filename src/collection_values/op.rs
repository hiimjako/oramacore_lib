use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub enum CollectionValueOperation {
    Set { key: String, value: String },
    Delete { key: String },
}
