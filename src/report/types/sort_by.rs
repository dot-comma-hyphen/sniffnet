use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum SortBy {
    #[default]
    Packets,
    Bytes,
    Latency,
}
