use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use clap::Parser;
use log::{trace, debug, info, error};
use serde::{Deserialize, Serialize};
use tera::Tera;
use toml;

use kitsuvm_poc::cli;
use kitsuvm_poc::config::{parse_config_files, check_i_v_compat, check_i_v_d_compat, project::Project, pinlist, vip::{VIP, Item}, instance::{Instances, Instance, Mode}};
use kitsuvm_poc::dut_parser;
use kitsuvm_poc::uvm::{tb, th};
/*
#[derive(Serialize, Debug)]
struct Variable {
    name: String,
}

#[derive(Serialize, Debug)]
struct Function {
    name: String,
    parameters: Vec<Variable>,
    body: String,
    is_declared_internally: bool,
}

#[derive(Serialize, Debug)]
struct Class {
    name: String,
    members: Vec<Variable>,
    functions: Vec<Function>,
}
*/
fn main() {
    env_logger::init();
    debug!("starting up");

    debug!("parsing cli");
    let cli = cli::Args::parse();
    trace!("cli parsed:\n{:#?}", cli);

    let (project, mut instances, vips) = parse_config_files(&cli);
    instances.estimate_ids();
    check_i_v_compat(&instances, &vips);

    let dut = dut_parser::parse_dut(&project.dut);
    check_i_v_d_compat(&instances, &vips, &dut);

    info!("loading tera templates from {}", cli.templates);
    let mut tera_dir = match Tera::new(&cli.templates) {
        Ok(t) => t,
        Err(e) => {
            error!("Parsing error(s): {}", e);
            panic!();
        }
    };
    tera_dir.autoescape_on(vec![]);

    let mut render_vips = Vec::new();
    for v in &vips {
        let vip = kitsuvm_poc::render::VIP::try_from(v).unwrap();
        render_vips.push(vip);
    }

    let vip_components = vec![
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
    ];

    for v in render_vips {
        let output_directory_path = format!("{}/vip/{}", cli.output.clone(), v.name);
        std::fs::create_dir_all(&output_directory_path).unwrap();

        let mut context = tera::Context::new();
        context.insert("vip", &v);

        for c in &vip_components {
            let template_path = format!("vip/{}.sv.j2", c);

            match tera_dir.render(&template_path, &context) {
                Ok(render) => {
                    //println!("{}", render);

                    let output_path = format!("{}/{}.sv", output_directory_path, c);
                    let mut file = File::create(output_path).unwrap();
                    file.write_all(render.as_bytes()).unwrap();
                },
                Err(e) => {
                    println!("Error: {}", e);
                    let mut cause = e.source();
                    while let Some(e) = cause {
                        println!("Reason: {}", e);
                        cause = e.source();
                    }
                }
            };
        }
    }

    /*
    let fifo_in_instances = instances.instances.iter().filter(|i| i.vip_name == "fifo_out").collect::<Vec<_>>();
    println!("{:#?}", fifo_in_instances);
*/
    /*
    let th = th::build(&dut, &pinlist, &templates);
    th.check_pinlists().unwrap();

    let tb = tb::build(&common, &templates);
*/
/*
    let templates_path = "templates/**/*.sv.j2";
    let mut tera_dir = Tera::new(templates_path).unwrap();
    tera_dir.autoescape_on(vec![]);
*/

/*
    let class = Class {
        name: "not_base_test".to_string(),
        members: vec![Variable { name: "int val".to_string() }, Variable { name: "bool is_valid".to_string() }],
        functions: vec![
            Function {
                name:"new".to_string(),
                parameters: vec![Variable { name:"string name".to_string() }, Variable { name:"uvm_component parent".to_string() }],
                body: "\n\
                    // this is a multiline function\n\
                    // comment\n\
                    super.new(name, parent);".to_string(),
                is_declared_internally:true},
            Function {
                name:"new2".to_string(),
                parameters: vec![Variable { name:"string name".to_string() }, Variable { name:"uvm_component parent".to_string() }],
                body: "".to_string(),
                is_declared_internally:true},
            Function {
                name:"not_new".to_string(),
                parameters: vec![Variable { name:"string name".to_string() }, Variable { name:"uvm_component parent".to_string() }],
                body: "".to_string(),
                is_declared_internally:false},
            Function {
                name:"not_new2".to_string(),
                parameters: vec![Variable { name:"string name".to_string() }, Variable { name:"uvm_component parent".to_string() }],
                body: "\n\
                    // this is another multiline\n\
                    // function body".to_string(),
                is_declared_internally:false},
        ]
    };
*/
/*
    let mut context = tera::Context::new();
    context.insert("class", &class);
    context.insert("project_name", "better_easier_uvm");
    context.insert("header", &true);

    match tera_dir.render("driver.sv.j2", &context) {
        Ok(render) => {
            println!("{}", render);
            let mut file = File::create("test_out.sv").unwrap();
            file.write_all(render.as_bytes()).unwrap();
        },
        Err(e) => {
            println!("Error: {}", e);
            let mut cause = e.source();
            while let Some(e) = cause {
                println!("Reason: {}", e);
                cause = e.source();
            }
        }
    };
*/
}

