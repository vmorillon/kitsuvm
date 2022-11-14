use std::collections::HashMap;
use std::iter::zip;
use std::str::FromStr;

use log::{debug, warn, error};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::config::{vip::VIP as VIPcfg, instance::{Instance, Instances, Mode::{Controller, Passive}}};
use crate::dut::utils::{ParsePortError, Port, PortDirection, DUT};

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
    PortError(#[from] ParsePortError),

    #[error("invalid member")]
    MemberError(#[from] ParseMemberError),
}

impl TryFrom<&VIPcfg> for VIP {
    type Error = ParseVIPError;

    fn try_from(vip: &VIPcfg) -> Result<Self, Self::Error> {
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

pub fn get_render_vips(vips: &Vec<VIPcfg>) -> Vec<VIP> {
    let mut render_vips = Vec::new();
    for v in vips {
        let vip = VIP::try_from(v).unwrap();
        render_vips.push(vip);
    }
    render_vips
}

pub fn set_vips_port_dir(vips: &mut Vec<VIP>, instances: &Instances, dut: &DUT) {
    for v in vips {
        let instances: Vec<Instance> = instances.instances.clone()
            .into_iter()
            .filter(|instance| instance.vip_name == v.name)
            .filter(|instance| instance.mode != Passive)
            .collect();

        let mut directions = HashMap::<String, PortDirection>::new();

        for i in instances {
            let zip_ports = zip(v.ports.clone(), i.connected_to);

            for (vp, dp) in zip_ports {
                if let Some(dir) = directions.get(&vp.name) {
                    let expected_dir = if i.mode == Controller {
                        !dir.clone()
                    } else {
                        dir.clone()
                    };
                    let port_dir = dut.ports.get(&dp).unwrap().direction.clone();

                    if expected_dir != port_dir {
                        error!("port_dir not matching");
                    }
                } else {
                    let port_dir = dut.ports.get(&dp).unwrap().direction.clone();
                    let dir = if i.mode == Controller {
                        !port_dir
                    } else {
                        port_dir
                    };
                    directions.insert(vp.name, dir);
                }
            }
        }

        for p in &mut v.ports {
            if let Some(dir) = directions.get(&p.name) {
                debug!("port {} direction set to {:#?}", p.name, dir);
                p.properties.direction = dir.clone();
            } else {
                warn!("port {} direction not set", p.name);
            }
        }
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
            "",
            "    ",
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
