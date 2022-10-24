pub mod agent;
pub mod interface;
pub mod item;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Template {
    pub agent: agent::Agent,
    pub interface: interface::Interface,
    pub item: item::Item,
}
