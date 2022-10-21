use std::collections::{HashMap, HashSet};

#[derive(Default, Debug)]
pub struct TestHarness {
    pub interfaces: HashMap<String, Interface>,
    pub dut: DUT,
}

impl TestHarness {
    fn check_pinlists(&self) -> Result<(), String> {
        let mut connected = HashSet::new();

        for (_n,i) in &self.interfaces {
            i.check_pinlist(&mut connected).unwrap();
        }

        for pi in connected {
            if !self.dut.ports.contains_key(&pi) {
                return Err(format!("{} does not exist in {}",pi, self.dut.name));
            }
        }

        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct InterfaceProperties {
    protocol: String,
    ports: HashMap<String, PortProperties>,
    use_clocking_block: bool,
}

#[derive(Default, Debug)]
pub struct Interface {
    pinlist: HashMap<String, String>,
    properties: InterfaceProperties,
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

#[derive(Default, Debug)]
pub enum PortDirection {
    INPUT,
    OUTPUT,
    #[default] INOUT,
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
