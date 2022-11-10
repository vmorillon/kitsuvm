pub mod project;
pub mod vip;
pub mod instance;

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

use log::{trace, debug, info, warn, error};

use project::Project;
use vip::VIP;
use instance::Instances;

use crate::cli::Args;
use crate::dut::utils::DUT;

pub fn parse_config_files(cli: &Args) -> (Project, Instances, Vec<VIP>) {
    let project = parse_project_file(cli.project.clone());

    let instances = parse_instances_file(cli.instances.clone());

    let vips = parse_vip_files(&cli.vips);

    (project, instances, vips)
}

pub fn parse_project_file(path: String) -> Project {
    info!("reading project {}", path);
    let project_str = std::fs::read_to_string(path).unwrap();
    let mut project: Project = toml::from_str(&project_str).unwrap();
    if project.dut.name == None {
        let name = get_name_from_file_path(project.dut.path.clone());
        debug!("default dut name to {}", name);
        project.dut.name = Some(name);
    }

    trace!("project parsed:\n{:#?}", project);
    project
}

fn parse_instances_file(path: String) -> Instances {
    info!("reading instances {}", path);
    let instances_str = std::fs::read_to_string(path).unwrap();
    let instances: Instances = toml::from_str(&instances_str).unwrap();

    trace!("instances parsed:\n{:#?}", instances);
    instances
}

pub fn parse_vip_files(paths: &Vec<String>) -> Vec<VIP> {
    info!("reading vip templates");
    let mut vips = Vec::new();
    for path in paths {
        info!("reading vip template {}", path);
        let vip_str = std::fs::read_to_string(&path).unwrap();
        let mut vip: VIP = toml::from_str(&vip_str).unwrap();
        if vip.name == None {
            let name = get_name_from_file_path(path.clone());
            debug!("default vip name to {}", name);
            vip.name = Some(name);
        }
        trace!("template parsed:\n{:#?}", vip);
        vips.push(vip);
    }

    vips
}

fn get_name_from_file_path(file_path: String) -> String {
    let file_name = if let Some((_path, file_name)) = file_path.rsplit_once('/') {
        file_name
    } else {
        &file_path
    };

    let name = if let Some((name, _extension)) = file_name.rsplit_once('.') {
        name
    } else {
        file_name
    };
    let name = name.to_string();

    name
}

pub fn check_i_v_compat(instances: &Instances, vips: &Vec<VIP>) {
    info!("checking instances <-> vip ports compatibility");
    let mut vip_ports = HashMap::new();
    for v in vips {
        vip_ports.insert(v.name.clone().unwrap(), &v.ports);
    }
    for i in &instances.instances {
        match vip_ports.get(&i.vip_name) {
            Some(ports) => {
                debug!("instanciate vip {}", i.vip_name);

                match ports.len().cmp(&i.connected_to.len()) {
                    Ordering::Equal => {
                        debug!("all ports connected {}", i);
                    }
                    Ordering::Greater => {
                        warn!("found less connected ports than declared in {}\ngot {} expected {}", i, i.connected_to.len(), ports.len());
                    }
                    Ordering::Less => {
                        error!("found more connected ports than declared in {}\ngot {} expected {}", i, i.connected_to.len(), ports.len());
                    }
                }
            }
            None => {
                warn!("unknown vip {}", i.vip_name);
            }
        }
    }
}

pub fn check_i_v_d_compat(instances: &Instances, _vips: &Vec<VIP>, dut: &DUT) {
    // #TODO add dimensions checks using vips info
    info!("checking (instances vip) <-> DUT ports compatibility");
    let mut connected = HashSet::new();

    for i in &instances.instances {
        for p in &i.connected_to {
            if dut.ports.contains_key(p) {
                if connected.insert(p) {
                    debug!("{} connected succefully by {}", p, i);
                } else {
                    error!("{} is already connected by {}", p, i);
                }
            } else {
                error!("{} in {} does not exist in dut {}", p, i, dut.name);
            }
        }
    }
}
