use std::collections::HashMap;
use std::ops::Not;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Default, Debug)]
pub struct DUT {
    pub name: String,
    pub ports: HashMap<String, PortProperties>,
}

#[derive(Deserialize, Serialize, Clone, Default, Debug, PartialEq)]
pub enum PortDirection {
    INPUT,
    OUTPUT,
    #[default]
    INOUT,
}

impl Not for PortDirection {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            PortDirection::INPUT => PortDirection::OUTPUT,
            PortDirection::OUTPUT => PortDirection::INPUT,
            PortDirection::INOUT => PortDirection::INOUT,
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct PortProperties {
    pub direction: PortDirection,
    pub dimensions: Vec<(u32, u32)>,
}

#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct Port {
    pub name: String,
    pub properties: PortProperties,
}

#[derive(Debug, Error)]
pub enum ParsePortError {
    #[error("invalid port description (expected: '<port_name> <dim0> <dim1>...', found: {0})")]
    InvalidPortDescription(String),

    #[error("invalid port name description")]
    InvalidPortNameDescription,

    #[error("invalid dimension description (expected: '[<u32>:<u32>]', found: {0})")]
    InvalidDimDescription(String),

    #[error("dimension is not a positive numeric value")]
    InvalidDimParsing(#[from] std::num::ParseIntError),
}

impl FromStr for Port {
    type Err = ParsePortError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split_whitespace().collect::<Vec<&str>>();

        if split.len() > 0 {
            let name = if split[0] == "" {
                return Err(Self::Err::InvalidPortNameDescription);
            } else {
                split[0].to_string()
            };
            let direction = PortDirection::INOUT;
            let mut dimensions = Vec::new();
            if split.len() > 1 {
                for i in 1..split.len() {
                    let s = split[i];
                    let parsed_dim = s
                        .strip_prefix('[')
                        .and_then(|s| s.strip_suffix(']'))
                        .and_then(|s| s.split_once(':'));

                    if let Some((end, start)) = parsed_dim {
                        let end = match end.parse::<u32>() {
                            Ok(end) => end,
                            Err(e) => return Err(Self::Err::InvalidDimParsing(e)),
                        };
                        let start = match start.parse::<u32>() {
                            Ok(start) => start,
                            Err(e) => return Err(Self::Err::InvalidDimParsing(e)),
                        };

                        dimensions.push((end, start));
                    } else {
                        return Err(Self::Err::InvalidDimDescription(s.to_string()));
                    }
                }
            }
            let properties = PortProperties {
                direction,
                dimensions,
            };
            Ok(Port { name, properties })
        } else {
            Err(Self::Err::InvalidPortDescription(s.to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Port;

    #[test]
    fn successful_ports_parsing() {
        let descriptions = vec![
            "port_name",
            "aaaa [5:2]",
            "aaaw  [6:3]",
            "aaww   [9:4]",
            "aagg [5:2]  ",
            "  ggaa [5:2]",
            "  gggg [5:2]  ",
            "bbbb [4:42]",
            "cc [5:2] [2:58]",
            "cw  [5:2]  [2:58]",
            "ww   [5:2]   [2:58]",
        ];

        for d in descriptions {
            let parsed_port = d.parse::<Port>();
            assert!(parsed_port.is_ok());
        }
    }

    #[test]
    fn failed_ports_parsing() {
        let descriptions = vec![
            "",
            "          ",
            "aaav [B:2]",
            "aavv [B:H]",
            "aava [9:F]",
            "aaan [-5:2]",
            "aann [-5:-2]",
            "aana [5:-2]",
            "aaab [ 5:2]",
            "aabb [ 5 :2]",
            "abbb [ 5 : 2]",
            "bbbb [ 5 : 2 ]",
            "a [5:2] b",
        ];

        for d in descriptions {
            let parsed_port = d.parse::<Port>();
            assert!(parsed_port.is_err());
        }
    }
}
