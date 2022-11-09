use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::dut::utils::Port;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct VIP {
    pub name: String,
    pub ports: Vec<Port>,
    pub clock: Option<String>,
    pub reset: Option<String>,
    pub use_clock_block: bool,

    pub item: Item,
}

#[derive(Debug)]
pub struct ParseVIPError {}

impl TryFrom<&crate::config::vip::VIP> for VIP {
    type Error = ParseVIPError;

    fn try_from(vip: &crate::config::vip::VIP) -> Result<Self, Self::Error> {
        let mut ports = Vec::new();
        for p in &vip.ports {
            let port: Port = p.parse().unwrap();
            ports.push(port);
        }

        let mut members = Vec::new();
        for m in &vip.item.members {
            let member: Member = m.parse().unwrap();
            members.push(member);
        }

        let item = Item {
            members,
            constraints: vip.item.constraints.clone(),
        };

        Ok(VIP {
            name: vip.name.clone().unwrap(),
            ports,
            clock: vip.clock.clone(),
            reset: vip.reset.clone(),
            use_clock_block: vip.use_clock_block,

            item,
        })
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Item {
    pub members: Vec<Member>,
    pub constraints: Vec<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Member {
    name: String,
    kind: String,
    is_randomized: bool,
}

#[derive(Debug)]
pub struct ParseMemberError {}

impl fmt::Display for ParseMemberError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid member description\nexpected: <rand (opt)> <type> <name>")
    }
}

impl FromStr for Member {
    type Err = ParseMemberError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split_whitespace().collect::<Vec<&str>>();
        if split.len() == 2 {
            if split[0] == "rand" {
                Err(ParseMemberError{})
            } else {
                Ok(Member {
                    name: split[1].to_string(),
                    kind: split[0].to_string(),
                    is_randomized: false,
                })
            }
        } else if split.len() == 3 {
            if split[0] == "rand" {
                Ok(Member {
                    name: split[2].to_string(),
                    kind: split[1].to_string(),
                    is_randomized: true,
                })
            } else {
                Err(ParseMemberError{})
            }
        } else {
            Err(ParseMemberError{})
        }
    }
}
