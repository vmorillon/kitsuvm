use std::collections::HashSet;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Interface {
    pub ports: HashSet<String>,
    pub clock: String,
    pub reset: String,
    #[serde(default = "default_use_clock_block")]
    pub use_clock_block: bool,
}

fn default_use_clock_block() -> bool {
    true
}
