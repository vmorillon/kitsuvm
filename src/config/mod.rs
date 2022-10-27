pub mod common;
pub mod pinlist;
pub mod template;

use log::{debug, info};

use crate::cli::Args;

pub fn parse_config_files(cli: Args) -> (common::Common, pinlist::PinList, Vec<template::Template>) {
    info!("reading common {}", cli.common);
    let common_str = std::fs::read_to_string(cli.common).unwrap();
    let common: common::Common = toml::from_str(&common_str).unwrap();
    debug!("common parsed:\n{:#?}", common);

    info!("reading pinlist {}", cli.pinlist);
    let pinlist_str = std::fs::read_to_string(cli.pinlist).unwrap();
    let pinlist: pinlist::PinList = toml::from_str(&pinlist_str).unwrap();
    debug!("pinlist parsed:\n{:#?}", pinlist);

    info!("reading agent templates");
    let mut templates = Vec::new();
    for a in cli.templates {
        info!("reading agent template {}", a);
        let template_str = std::fs::read_to_string(a).unwrap();
        let template: template::Template = toml::from_str(&template_str).unwrap();
        debug!("template parsed:\n{:#?}", template);
        templates.push(template);
    }

    (common, pinlist, templates)
}
