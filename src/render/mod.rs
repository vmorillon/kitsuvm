pub mod top;
pub mod vip;

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Write;

use log::{trace, debug, info, error};
use tera::Tera;

use crate::cli::Args;
use crate::config::{instance::Instances, project::Project};

use top::Top;
use vip::VIP;

enum Mode {
    VIP(VIP),
    Top(Top, Vec<VIP>, Instances),
    TopTest(Top, Vec<VIP>, Instances),
    TopTb(Top, Vec<VIP>, Instances),
    Bin(Top, Vec<VIP>),
}

impl Mode {
    fn get_components(&self) -> Vec<String> {
        match self {
            Mode::VIP(_) => {
                vec![
                    "agent".to_string(),
                    "config".to_string(),
                    "coverage".to_string(),
                    "driver".to_string(),
                    "if".to_string(),
                    "monitor".to_string(),
                    "pkg".to_string(),
                    "seq_lib".to_string(),
                    "sequencer".to_string(),
                    "tx".to_string()
                ]
            }
            Mode::Top(_, _, _) => {
                vec![
                    "config".to_string(),
                    "env".to_string(),
                    "pkg".to_string(),
                    "scoreboard".to_string(),
                    "seq_lib".to_string(),
                ]
            }
            Mode::TopTest(_, _, _) => {
                vec![
                    "test".to_string(),
                    "test_pkg".to_string(),
                ]
            }
            Mode::TopTb(_, _, _) => {
                vec![
                    "tb".to_string(),
                    "th".to_string(),
                ]
            }
            Mode::Bin(_, _) => {
                vec![
                    "run".to_string(),
                ]
            }
        }
    }

    fn get_output_directory_path(&self, cli: &Args) -> String {
        match self {
            Mode::VIP(vip) => format!("{}/vip/{}", cli.output.clone(), vip.name),
            Mode::Top(_, _, _) => format!("{}/top", cli.output.clone()),
            Mode::TopTest(_, _, _) => format!("{}/top/test", cli.output.clone()),
            Mode::TopTb(_, _, _) => format!("{}/top/tb", cli.output.clone()),
            Mode::Bin(_, _) => format!("{}/bin", cli.output.clone()),
        }
    }

    fn get_output_filename(&self, component: String) -> String {
        match self {
            Mode::VIP(vip) => format!("{}_{}.sv", vip.name, component),
            Mode::Top(top, _, _) => format!("{}_{}.sv", top.name, component),
            Mode::TopTest(top, _, _) => format!("{}_{}.sv", top.name, component),
            Mode::TopTb(top, _, _) => format!("{}_{}.sv", top.name, component),
            Mode::Bin(_, _) => format!("{}.sh", component),
        }
    }

    fn get_template_path(&self, component: String) -> String {
        match self {
            Mode::VIP(_) => format!("vip/{}.sv.j2", component),
            Mode::Top(_, _, _) => format!("top/{}.sv.j2", component),
            Mode::TopTest(_, _, _) => format!("top/test/{}.sv.j2", component),
            Mode::TopTb(_, _, _) => format!("top/tb/{}.sv.j2", component),
            Mode::Bin(_, _) => format!("bin/{}.sh.j2", component),
        }
    }

    fn get_context(&self) -> tera::Context {
        let mut context = tera::Context::new();
        match self {
            Mode::VIP(vip) => {
                context.insert("vip", &vip);
            }
            Mode::Top(top, vips, instances) => {
                context.insert("instances", &instances.instances);
                context.insert("vips", &vips);
                context.insert("top", &top);
            }
            Mode::TopTest(top, vips, instances) => {
                context.insert("instances", &instances.instances);
                context.insert("vips", &vips);
                context.insert("top", &top);
            }
            Mode::TopTb(top, vips, instances) => {
                context.insert("instances", &instances.instances);
                context.insert("top", &top);

                let (vips_clk, vips_rst, vips_ports) = get_vips_clk_rst_ports(vips);
                context.insert("vips_clk", &vips_clk);
                context.insert("vips_rst", &vips_rst);
                context.insert("vips_ports", &vips_ports);
            }
            Mode::Bin(top, vips) => {
                context.insert("vips", &vips);
                context.insert("top", &top);
            }
        }
        context
    }
}

fn get_vips_clk_rst_ports(vips: &Vec<VIP>) -> (HashMap<String,String>, HashMap<String,String>, HashMap<String,Vec<String>>) {
    let mut vips_clk = HashMap::new();
    for v in vips {
        if let Some(clk) = v.clock.clone() {
            vips_clk.insert(v.name.clone(), clk);
        }
    }

    let mut vips_rst = HashMap::new();
    for v in vips {
        if let Some(rst) = v.reset.clone() {
            vips_rst.insert(v.name.clone(), rst);
        }
    }

    let mut vips_ports = HashMap::new();
    for v in vips {
        let mut ports = Vec::new();
        for p in &v.ports {
            ports.push(p.name.clone());
        }
        vips_ports.insert(v.name.clone(), ports);
    }

    (vips_clk, vips_rst, vips_ports)
}

fn render(mode: Mode, tera_dir: &Tera, cli: &Args) {
    let components = mode.get_components();

    let output_directory_path = mode.get_output_directory_path(cli);
    info!("creating directory {}", output_directory_path);
    std::fs::create_dir_all(&output_directory_path).unwrap();

    let context = mode.get_context();

    for c in components {
        let template_path = mode.get_template_path(c.clone());
        match tera_dir.render(&template_path, &context) {
            Ok(render) => {
                let output_filename = mode.get_output_filename(c.clone());
                let output_path = format!("{}/{}", output_directory_path, output_filename);
                debug!("writing {}", output_path);

                let mut file = File::create(output_path).unwrap();
                file.write_all(render.as_bytes()).unwrap();
            },
            Err(e) => {
                error!("{}", e);
                let mut cause = e.source();
                while let Some(e) = cause {
                    error!("Reason: {}", e);
                    cause = e.source();
                }
            }
        };
    }
}

pub fn render_self_test(tera_dir: &Tera, vip: &VIP, instances: &Instances, cli: &Args, project: &Project) {
    let name = format!("self_test_{}", vip.name);
    let top = Top {
        name,
        default_sequence_repeat: project.top_default_sequence,
        dut_name: "".to_string(),
        dut_clk: None,
        dut_rst: None,
    };

    let mut modes = Vec::new();

    for m in modes {
        render(m, tera_dir, cli);
    }
}

pub fn render_top(tera_dir: &Tera, vips: &Vec<VIP>, instances: &Instances, cli: &Args, project: &Project) {
    let top = Top {
        name: "top".to_string(),
        default_sequence_repeat: project.top_default_sequence,
        dut_name: project.dut.name.clone().unwrap(),
        dut_clk: project.dut.clock.clone(),
        dut_rst: project.dut.reset.clone(),
    };

    let mut modes = Vec::new();
    modes.push(Mode::Top(top.clone(), vips.clone(), instances.clone()));
    modes.push(Mode::TopTest(top.clone(), vips.clone(), instances.clone()));
    modes.push(Mode::TopTb(top.clone(), vips.clone(), instances.clone()));
    modes.push(Mode::Bin(top.clone(), vips.clone()));

    for m in modes {
        render(m, tera_dir, cli);
    }
}

pub fn render_vips(tera_dir: &Tera, vips: &Vec<VIP>, cli: &Args) {
    let mut modes = Vec::new();
    for v in vips {
        modes.push(Mode::VIP(v.clone()));
    }

    for m in modes {
        render(m, tera_dir, cli);
    }
}

pub fn get_tera_dir(cli: &Args) -> Tera {
    let templates_realpath = std::fs::canonicalize(&cli.templates).unwrap();
    let templates_query = format!("{}/**/*.j2", templates_realpath.to_str().unwrap());
    info!("loading tera templates from {}", templates_query);
    let mut tera_dir = match Tera::new(&templates_query) {
        Ok(t) => t,
        Err(e) => {
            error!("Parsing error(s): {}", e);
            panic!();
        }
    };
    let names: Vec<_> = tera_dir.get_template_names().collect();
    trace!("loaded templates:\n{:#?}", names);
    tera_dir.autoescape_on(vec![]);

    tera_dir
}
