pub mod project;
pub mod pinlist;
pub mod template;
pub mod vip;
pub mod instance;

use std::collections::{HashMap, HashSet};

use log::{trace, debug, info, warn};

use project::Project;
use vip::VIP;
use instance::Instances;

use crate::cli::Args;

pub fn parse_config_files(cli: Args) -> (Project, Instances, Vec<VIP>) {
    info!("reading project {}", cli.project);
    let project_str = std::fs::read_to_string(cli.project).unwrap();
    let project: Project = toml::from_str(&project_str).unwrap();
    trace!("project parsed:\n{:#?}", project);

    info!("reading instances {}", cli.instances);
    let instances_str = std::fs::read_to_string(cli.instances).unwrap();
    let instances: Instances = toml::from_str(&instances_str).unwrap();
    trace!("instances parsed:\n{:#?}", instances);

    info!("reading vip templates");
    let mut templates = Vec::new();
    for a in cli.templates {
        info!("reading vip template {}", a);
        let template_str = std::fs::read_to_string(a).unwrap();
        let template: VIP = toml::from_str(&template_str).unwrap();
        trace!("template parsed:\n{:#?}", template);
        templates.push(template);
    }

    (project, instances, templates)
}

pub fn check_instances_vip_compat(instances: &Instances, templates: &Vec<VIP>) {
    let mut vip_ports = HashMap::new();
    for t in templates {
        vip_ports.insert(t.name.clone(), &t.ports);
    }
    for i in &instances.instances {
        match vip_ports.get(&i.vip_name) {
            Some(ports) => {
                debug!("instanciate vip {}", i.vip_name);

                if ports.len() == i.connected_to.len() {
                    debug!("all port connected {}", i);
                } else {
                    warn!("mismatch ports number {} got {} expected {}", i, i.connected_to.len(), ports.len());
                }
            }
            None => {
                warn!("unknown vip {}", i.vip_name);
            }
        }
    }
}
