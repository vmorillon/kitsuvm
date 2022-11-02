use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Project {
    #[serde(default = "default_generate_file_header")]
    pub generate_file_header: bool,
    #[serde(default = "default_top_default_sequence")]
    pub top_default_sequence: u32,
    
    #[serde(default = "default_dut")]
    pub dut: DUT,
}

fn default_dut() -> DUT {
    DUT {
        path: default_path(),
        clock: None,
        reset: None,
    }
}
fn default_generate_file_header() -> bool {
    false
}
fn default_top_default_sequence() -> u32 {
    5
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct DUT {
    #[serde(default = "default_path")]
    pub path: String,
    pub clock: Option<String>,
    pub reset: Option<String>,
}

fn default_path() -> String {
    "dut.sv".to_string()
}
