use log::{debug, info, error};

use std::collections::{HashMap, HashSet};

#[derive(Default, Debug)]
pub struct TestHarness {
    pub interfaces: HashMap<String, Interface>,
    pub dut: DUT,
}

use crate::config::{template, pinlist};

pub fn build(dut: DUT, pinlist: pinlist::PinList, templates: Vec<template::Template>) -> TestHarness {
    info!("generating test harness");
    let mut interfaces = HashMap::new();

    for t in templates {
        for i in 0..t.agent.number_of_instances {
            let name = if t.agent.number_of_instances == 1 {
                format!("{}",t.agent.name).to_string()
            } else {
                format!("{}_{}",t.agent.name, i).to_string()
            };
            let pinlist = pinlist.interface_map.get(&name).unwrap().clone();

            let mut ports = HashMap::new();
            for p in &t.interface.ports {
                let mut port = p.parse::<Port>().unwrap();

                let dut_port_name = pinlist.get(&port.name).unwrap();
                let dut_port_prop = dut.ports.get(dut_port_name).unwrap();
                let dut_port_dir = dut_port_prop.direction.clone();
                port.properties.direction = !dut_port_dir;

                ports.insert(port.name, port.properties);
            }
            let properties = InterfaceProperties {
                protocol: t.item.name.clone(),
                ports,
                use_clocking_block: t.interface.use_clock_block,
            };

            let interface = Interface {
                pinlist,
                properties,
            };

            interfaces.insert(name, interface);
        }
    }

    let th = TestHarness {
        interfaces,
        dut,
    };
    debug!("test harness generated:\n{:#?}", th);
    th
}


impl TestHarness {
    pub fn check_pinlists(&self) -> Result<(), String> {
        info!("checking test harness pinlists");
        let mut connected = HashSet::new();

        for (n, i) in &self.interfaces {
            info!("checking interface {} pinlist", n);
            i.check_pinlist(&mut connected).unwrap();
        }

        for pi in connected {
            if !self.dut.ports.contains_key(&pi) {
                error!("{} not found in {}", pi, self.dut.name);
                return Err(format!("{} does not exist in {}",pi, self.dut.name));
            }
        }

        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct InterfaceProperties {
    pub protocol: String,
    pub ports: HashMap<String, PortProperties>,
    pub use_clocking_block: bool,
}

#[derive(Default, Debug)]
pub struct Interface {
    pub pinlist: HashMap<String, String>,
    pub properties: InterfaceProperties,
}

impl Interface {
    fn check_pinlist(&self, connected: &mut HashSet::<String>) -> Result<(), String> {
        for (po,pi) in &self.pinlist {
            let newly_connected = connected.insert(pi.clone());
            if !newly_connected {
                return Err(format!("{} already connected when connecting to {}",pi,po));
            }
            if !self.properties.ports.contains_key(po) {
                return Err(format!("{} does not exist in {}",po, self.properties.protocol));
            }
        }

        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct DUT {
    pub name: String,
    pub ports: HashMap<String, PortProperties>,
}

#[derive(Clone, Default, Debug)]
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

#[derive(Default, Debug)]
pub struct PortProperties {
    pub direction: PortDirection,
    pub dimensions: Vec<(u32,u32)>,
}

#[derive(Default, Debug)]
pub struct Port {
    pub name: String,
    pub properties: PortProperties,
}

#[derive(Debug)]
pub struct ParsePortError;

use std::fmt;

impl fmt::Display for ParsePortError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid port description: <port_name> <dim0> <dim1>...")
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
