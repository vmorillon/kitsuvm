use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct VIP {
    pub name: Option<String>,
    pub ports: Vec<String>,
    pub clock: Option<String>,
    pub reset: Option<String>,
    #[serde(default = "default_use_clock_block")]
    pub use_clock_block: bool,

    #[serde(default)]
    pub item: Item,
}

fn default_use_clock_block() -> bool {
    true
}

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct Item {
    pub members: Vec<String>,
    pub constraints: Vec<String>,
}
