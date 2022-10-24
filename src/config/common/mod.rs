use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Common {
    #[serde(default = "default_dut_path")]
    pub dut_path: String,
    #[serde(default = "default_generate_file_header")]
    pub generate_file_header: bool,
    #[serde(default = "default_top_default_sequence")]
    pub top_default_sequence: u32,
}

fn default_dut_path() -> String {
    "dut.sv".to_string()
}
fn default_generate_file_header() -> bool {
    false
}
fn default_top_default_sequence() -> u32 {
    5
}
