use serde::{Deserialize, Serialize};

use std::collections::HashMap;

#[derive(Clone, Default, Debug)]
pub struct DUT {
    pub name: String,
    pub ports: HashMap<String, PortProperties>,
}

#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub enum PortDirection {
    INPUT,
    OUTPUT,
    #[default] INOUT,
}

use std::ops::Not;
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
    pub dimensions: Vec<(u32,u32)>,
}

#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct Port {
    pub name: String,
    pub properties: PortProperties,
}

#[derive(Debug)]
pub struct ParsePortError;

use std::fmt;

impl fmt::Display for ParsePortError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid port description\nexpected: <port_name> <dim0> <dim1>...")
    }
}

use std::str::FromStr;

impl FromStr for Port {
    type Err = ParsePortError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split_whitespace().collect::<Vec<&str>>();

        if split.len() > 0 {
            let name = split[0].to_string();
            let direction = PortDirection::INOUT;
            let mut dimensions = Vec::new();
            if split.len() > 1 {
                for i in 1..split.len() {
                    let s = split[i];
                    let (end, start) = s
                        .strip_prefix('[')
                        .and_then(|s| s.strip_suffix(']'))
                        .and_then(|s| s.split_once(':'))
                        .unwrap();

                    let end = end.parse::<u32>().unwrap();
                    let start = start.parse::<u32>().unwrap();

                    dimensions.push((end, start));
                }
            }
            let properties = PortProperties {
                direction,
                dimensions,
            };
            Ok(Port {
                name,
                properties,
            })
        } else {
            Err(ParsePortError)
        }
    }
}
