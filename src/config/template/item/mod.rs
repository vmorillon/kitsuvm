use std::collections::HashSet;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Item {
    pub name: String,
    pub members: HashSet<String>,
    pub constraints: HashSet<String>,
}
