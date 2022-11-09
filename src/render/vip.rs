use std::str::FromStr;

use serde::{Deserialize, Serialize};
use thiserror::Error;

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

#[derive(Debug, Error)]
pub enum ParseVIPError {
    #[error("invalid port")]
    PortError(#[from] crate::dut::utils::ParsePortError),

    #[error("invalid member")]
    MemberError(#[from] ParseMemberError),
}

impl TryFrom<&crate::config::vip::VIP> for VIP {
    type Error = ParseVIPError;

    fn try_from(vip: &crate::config::vip::VIP) -> Result<Self, Self::Error> {
        let mut ports = Vec::new();
        for p in &vip.ports {
            let port: Port = match p.parse() {
                Ok(port) => port,
                Err(e) => return Err(Self::Error::PortError(e)),
            };
            ports.push(port);
        }

        let mut members = Vec::new();
        for m in &vip.item.members {
            let member: Member = match m.parse() {
                Ok(member) => member,
                Err(e) => return Err(Self::Error::MemberError(e)),
            };
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

#[derive(Debug, Error)]
pub enum ParseMemberError {
    #[error("invalid member description (expected: <rand (opt)> <type> <name>, found: {0})")]
    InvalidMemberDescription(String),

    #[error("unexpected rand found (expected: <type> <name>, found: {0})")]
    UnexpectedRand(String),

    #[error("expected rand (expected: rand <type> <name>, found: {0})")]
    RandNotFound(String),
}

impl FromStr for Member {
    type Err = ParseMemberError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split_whitespace().collect::<Vec<&str>>();

        match split.len() {
            2 => {
                if split[0] == "rand" {
                    Err(Self::Err::UnexpectedRand(s.to_string()))
                } else {
                    Ok(Member {
                        name: split[1].to_string(),
                        kind: split[0].to_string(),
                        is_randomized: false,
                    })
                }
            },
            3 => {
                if split[0] == "rand" {
                    Ok(Member {
                        name: split[2].to_string(),
                        kind: split[1].to_string(),
                        is_randomized: true,
                    })
                } else {
                    Err(Self::Err::RandNotFound(s.to_string()))
                }
            },
            _ => {
                Err(Self::Err::InvalidMemberDescription(s.to_string()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Member;

    #[test]
    fn successful_members_parsing() {
        let descriptions = vec![
            "__type__ __name__",
            "rand __type__ __name__",
            "bit is_valid",
            "rand bit[24:16] is_ready",
            "rand bit is_ready",
            "  rand   bit   is_ready  ",
        ];

        for d in descriptions {
            let parsed_member = d.parse::<Member>();
            assert!(parsed_member.is_ok());
        }
    }

    #[test]
    fn failed_members_parsing() {
        let descriptions = vec![
            "__not_rand__ __type__ __name__",
            "__name__",
            "a b c d",
            "rand bit [31:2] is_ready",
        ];

        for d in descriptions {
            let parsed_member = d.parse::<Member>();
            assert!(parsed_member.is_err());
        }
    }
}
