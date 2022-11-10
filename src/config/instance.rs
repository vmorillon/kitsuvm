use std::collections::{HashMap, HashSet};

use log::{debug, info, error};
use serde::{Deserialize, Serialize};

use crate::render::vip::VIP;

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Instances {
    pub instances: Vec<Instance>,
}

impl From<Vec<Instance>> for Instances {
    fn from(instances: Vec<Instance>) -> Self {
        Instances {
            instances
        }
    }
}

pub fn get_self_test_instances(vip: &VIP) -> Instances {
    let modes = vec![Mode::Controller, Mode::Passive, Mode::Responder];
    let mut instances = Vec::new();
    let mut connected_to = Vec::new();
    for p in &vip.ports {
        connected_to.push(p.name.clone());
    }
    for m in modes {
        let instance = Instance {
            vip_name: vip.name.clone(),
            connected_to: connected_to.clone(),
            id: Some(0),
            mode: m,
        };
        instances.push(instance);
    }

    instances.into()
}

impl Instances {
    fn get_already_used_ids(&self) -> HashMap<String, HashMap<Mode, HashSet<u32>>> {
        info!("checking already used IDs");
        let mut used = HashMap::<String, HashMap<Mode, HashSet<u32>>>::new();

        for i in &self.instances {
            match i.id {
                Some(id) => {
                    match used.get_mut(&i.vip_name) {
                        Some(used_per_mode) => {
                            match used_per_mode.get_mut(&i.mode) {
                                Some(used) => {
                                    if used.insert(id) {
                                        debug!("register ID {} for mode {:?} of vip {}", id, i.mode, i.vip_name);
                                    } else {
                                        error!("already registered ID {} for mode {:?} of vip {}, check your instances file", id, i.mode, i.vip_name);
                                    }

                                }
                                None => {
                                    let mut used = HashSet::new();
                                    used.insert(id);
                                    used_per_mode.insert(i.mode.clone(), used);
                                    debug!("register ID {} for mode {:?} of vip {}", id, i.mode, i.vip_name);
                                }
                            }

                        }
                        None => {
                            let mut used_per_mode = HashMap::new();
                            used_per_mode.insert(i.mode.clone(), HashSet::from([id]));
                            used.insert(i.vip_name.clone(), used_per_mode);
                            debug!("register ID {} for mode {:?} of vip {}", id, i.mode, i.vip_name);
                        }
                    }
                }
                None => {
                    debug!("pass unset ID for mode {:?} of vip {}", i.mode, i.vip_name);
                }
            }
        }
        debug!("checked already used IDs");
        used
    }

    pub fn estimate_ids(&mut self) {
        info!("estimating unset IDs");
        let used = self.get_already_used_ids();
        let mut counts = HashMap::<String, HashMap<Mode, u32>>::new();

        for i in &mut self.instances {
            match i.id {
                Some(_id) => {
                    debug!("user registered {}", i);
                }
                None => {
                    i.set_next_available_id(&mut counts, &used);
                    debug!("set {}", i);
                }
            }
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Instance {
    pub vip_name: String,
    #[serde(default)]
    pub connected_to: Vec<String>,
    pub id: Option<u32>,
    #[serde(default)]
    pub mode: Mode,
}

impl std::fmt::Display for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ID {} mode {:?} vip {}", self.id.unwrap(), self.mode, self.vip_name)
    }
}

impl Instance {
    fn set_next_available_id(&mut self, counts: &mut HashMap<String, HashMap<Mode, u32>>, used: &HashMap<String, HashMap<Mode, HashSet<u32>>>) {
        let available_id = match counts.get_mut(&self.vip_name) {
            Some(count_per_mode) => {
                match count_per_mode.get_mut(&self.mode) {
                    Some(count) => {
                        *count += 1;
                        let next = self.get_next_available_id(*count, used);
                        *count = next;
                        next
                    }
                    None => {
                        let next = self.get_next_available_id(0, used);
                        count_per_mode.insert(self.mode.clone(), next);
                        next
                    }
                }
            }
            None => {
                let next = self.get_next_available_id(0, used);

                let mut count_per_mode = HashMap::new();
                count_per_mode.insert(self.mode.clone(), next);
                counts.insert(self.vip_name.clone(), count_per_mode);

                next
            }
        };
        self.id = Some(available_id);
    }

    fn get_next_available_id(&self, start: u32, used: &HashMap<String, HashMap<Mode, HashSet<u32>>>) -> u32 {
        match used.get(&self.vip_name) {
            Some(used_per_mode) => {
                match used_per_mode.get(&self.mode) {
                    Some(used) => {
                        let mut count = start;
                        while used.contains(&count) {
                            count += 1;
                        }
                        count
                    }
                    None => {
                        start
                    }
                }
            }
            None => {
                start
            }
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Default, PartialEq, Eq, Hash, Clone)]
pub enum Mode {
    #[default]
    Controller,
    Responder,
    Passive,
}
