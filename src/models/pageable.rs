use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Pageable {
    pub page: Option<u64>,
    pub size: Option<u64>
}