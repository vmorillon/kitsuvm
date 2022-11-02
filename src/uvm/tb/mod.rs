use log::{debug, info};

use crate::config::{common, template};

#[derive(Debug)]
pub struct EnvConfig {

}

#[derive(Debug)]
pub struct Env {
    agents: Vec<Agent>,

    config: EnvConfig,
}

pub fn build(common: &common::Common, templates: &Vec<template::Template>) -> Env {
    info!("generating env");

    let mut agents = Vec::new();

    for t in templates {
        for i in 0..t.agent.number_of_instances {
            let name = if t.agent.number_of_instances == 1 {
                format!("{}",t.agent.name).to_string()
            } else {
                format!("{}_{}",t.agent.name, i).to_string()
            };

            let mut sequences = Vec::new();
            sequences.push(Sequence {
                name: "default".to_string(),
            });

            let sequencer = Sequencer {
                sequences,
            };
            let driver = Driver {};
            let monitor = Monitor {};
            let config = AgentConfig {
                is_active: t.agent.is_active,
            };

            let agent = Agent {
                name,
                sequencer,
                driver,
                monitor,
                config,
            };

            agents.push(agent);
        }
    }

    let config = EnvConfig {};
    
    let tb = Env {
        agents,
        config,
    };
    debug!("env generated:\n{:#?}", tb);
    tb
}

#[derive(Debug)]
pub struct AgentConfig {
    is_active: bool,
}

#[derive(Debug)]
pub struct Agent {
    name: String,
    sequencer: Sequencer,
    driver: Driver,
    monitor: Monitor,

    config: AgentConfig,
}

#[derive(Debug)]
pub struct Sequence {
    name: String,
}

#[derive(Debug)]
pub struct Sequencer {
    sequences: Vec<Sequence>,
}

#[derive(Debug)]
pub struct Driver {

}

#[derive(Debug)]
pub struct Monitor {

}
