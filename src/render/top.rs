use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Top {
    pub name: String,
    pub default_sequence_repeat: u32,

    pub dut_name: String,
    pub dut_clk: Option<String>,
    pub dut_rst: Option<String>,
}
