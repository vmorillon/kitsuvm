use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct PinList {
    #[serde(default = "default_top_dec")]
    pub top_wire_dec: HashSet<String>,
    #[serde(default = "default_top_dec")]
    pub top_param_dec: HashSet<String>,
    pub global_map: HashMap<String, String>,
    pub interface_map: HashMap<String, HashMap<String, String>>,
}

fn default_top_dec() -> HashSet<String> {
    HashSet::new()
}

