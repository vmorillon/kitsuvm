use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Agent {
    pub name: String,
    #[serde(default = "default_is_active")]
    pub is_active: bool,
    #[serde(default = "default_number_of_instances")]
    pub number_of_instances: u32,
}

fn default_is_active() -> bool {
    true
}
fn default_number_of_instances() -> u32 {
    1
}
