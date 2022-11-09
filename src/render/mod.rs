pub mod top;
pub mod vip;

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Write;

use log::{debug, info, error};
use tera::Tera;

use crate::cli::Args;
use crate::config::{instance::Instances, project::Project};

use top::Top;
use vip::VIP;

enum Mode {
    VIP(VIP),
    Top(Top),
    TopTest(Top),
    TopTb(Top),
    Bin(Top),
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
            Mode::Top(_) => {
                vec![
                    "config".to_string(),
                    "env".to_string(),
                    "pkg".to_string(),
                    "scoreboard".to_string(),
                    "seq_lib".to_string(),
                ]
            }
            Mode::TopTest(_) => {
                vec![
                    "test".to_string(),
                    "test_pkg".to_string(),
                ]
            }
            Mode::TopTb(_) => {
                vec![
                    "tb".to_string(),
                    "th".to_string(),
                ]
            }
            Mode::Bin(_) => {
                vec![
                    "run".to_string(),
                ]
            }
        }
    }

    fn get_output_directory_path(&self, cli: &Args) -> String {
        match self {
            Mode::VIP(vip) => {
                format!("{}/vip/{}", cli.output.clone(), vip.name)
            }
            Mode::Top(_) => {
                format!("{}/top", cli.output.clone())
            }
            Mode::TopTest(_) => {
                format!("{}/top/test", cli.output.clone())
            }
            Mode::TopTb(_) => {
                format!("{}/top/tb", cli.output.clone())
            }
            Mode::Bin(_) => {
                format!("{}/bin", cli.output.clone())
            }
        }
    }

    fn get_output_filename(&self, component: String) -> String {
        match self {
            Mode::VIP(vip) => {
                format!("{}_{}.sv", vip.name, component)
            }
            Mode::Top(top) => {
                format!("{}_{}.sv", top.name, component)
            }
            Mode::TopTest(top) => {
                format!("{}_{}.sv", top.name, component)
            }
            Mode::TopTb(top) => {
                format!("{}_{}.sv", top.name, component)
            }
            Mode::Bin(_) => {
                format!("{}.sh", component)
            }
        }
    }

    fn get_template_path(&self, component: String) -> String {
        match self {
            Mode::VIP(_) => {
                format!("vip/{}.sv.j2", component)
            }
            Mode::Top(_) => {
                format!("top/{}.sv.j2", component)
            }
            Mode::TopTest(_) => {
                format!("top/test/{}.sv.j2", component)
            }
            Mode::TopTb(_) => {
                format!("top/tb/{}.sv.j2", component)
            }
            Mode::Bin(_) => {
                format!("bin/{}.sh.j2", component)
            }
        }
    }

    fn get_context(&self, render_vips: &Vec<VIP>, instances: &Instances) -> tera::Context {
        let mut context = tera::Context::new();
        match self {
            Mode::VIP(vip) => {
                context.insert("vip", &vip);
            }
            Mode::Top(top) => {
                context.insert("instances", &instances.instances);
                context.insert("vips", &render_vips);
                context.insert("top", &top);
            }
            Mode::TopTest(top) => {
                context.insert("instances", &instances.instances);
                context.insert("vips", &render_vips);
                context.insert("top", &top);
            }
            Mode::TopTb(top) => {
                context.insert("instances", &instances.instances);
                context.insert("top", &top);

                let (vips_clk, vips_rst, vips_ports) = get_vips_clk_rst_ports(render_vips);
                context.insert("vips_clk", &vips_clk);
                context.insert("vips_rst", &vips_rst);
                context.insert("vips_ports", &vips_ports);
            }
            Mode::Bin(top) => {
                context.insert("vips", &render_vips);
                context.insert("top", &top);
            }
        }
        context
    }
}

fn get_vips_clk_rst_ports(render_vips: &Vec<VIP>) -> (HashMap<String,String>, HashMap<String,String>, HashMap<String,Vec<String>>) {
    let mut vips_clk = HashMap::new();
    for v in render_vips {
        if let Some(clk) = v.clock.clone() {
            vips_clk.insert(v.name.clone(), clk);
        }
    }

    let mut vips_rst = HashMap::new();
    for v in render_vips {
        if let Some(rst) = v.reset.clone() {
            vips_rst.insert(v.name.clone(), rst);
        }
    }

    let mut vips_ports = HashMap::new();
    for v in render_vips {
        let mut ports = Vec::new();
        for p in &v.ports {
            ports.push(p.name.clone());
        }
        vips_ports.insert(v.name.clone(), ports);
    }

    (vips_clk, vips_rst, vips_ports)
}

fn render(mode: Mode, tera_dir: &Tera, render_vips: &Vec<VIP>, instances: &Instances, cli: &Args) {
    let components = mode.get_components();

    let output_directory_path = mode.get_output_directory_path(cli);
    info!("creating directory {}", output_directory_path);
    std::fs::create_dir_all(&output_directory_path).unwrap();

    let context = mode.get_context(render_vips, instances);

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

pub fn render_all(tera_dir: &Tera, render_vips: &Vec<VIP>, instances: &Instances, cli: &Args, project: &Project) {
    let top = Top {
        name: "top".to_string(),
        default_sequence_repeat: project.top_default_sequence,
        dut_name: project.dut.name.clone().unwrap(),
        dut_clk: project.dut.clock.clone(),
        dut_rst: project.dut.reset.clone(),
    };

    let mut modes = Vec::new();
    for v in render_vips {
        modes.push(Mode::VIP(v.clone()));
    }
    modes.push(Mode::Top(top.clone()));
    modes.push(Mode::TopTest(top.clone()));
    modes.push(Mode::TopTb(top.clone()));
    modes.push(Mode::Bin(top.clone()));

    for m in modes {
        render(m, tera_dir, render_vips, instances, cli);
    }
}
