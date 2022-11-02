use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use clap::Parser;
use log::{trace, debug, info};
use serde::{Deserialize, Serialize};
use tera::Tera;
use toml;

use kitsuvm_poc::cli;
use kitsuvm_poc::config::{parse_config_files, check_instances_vip_compat, project::Project, pinlist, vip::{VIP, Item}, instance::{Instances, Instance, Mode}};
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

    let (project, mut instances, templates) = parse_config_files(cli);
    instances.estimate_ids();
    check_instances_vip_compat(&instances, &templates);

    let dut = dut_parser::parse_dut(&project.dut.path);

    /*
    let project_str = "".to_string();
    let cfg: Project = toml::from_str(&project_str).unwrap();
    println!("{:#?}", cfg);
    let project_str = toml::to_string(&cfg).unwrap();
    println!("{}", project_str);
    */
    /*
    let fifo_in_vip = VIP {
        name: "fifo_in".to_string(),
        ports: Vec::from(["aaa".to_string(), "bbb".to_string()]),
        clock: Some("clk".to_string()),
        reset: None,
        use_clock_block: true,
        item: Default::default(),
    };
    let fifo_in_vip_str = toml::to_string(&fifo_in_vip).unwrap();
    println!("{}", fifo_in_vip_str);
    let fifo_in_vip_str = std::fs::read_to_string("examples/fifo/fifo_in.toml").unwrap();
    let fifo_in_vip: VIP = toml::from_str(&fifo_in_vip_str).unwrap();
    println!("{:#?}", fifo_in_vip);
    */
    /*
    let instance_in = Instance {
        vip_name: "fifo_in".to_string(),
        connected_to: Vec::from(["aaa".to_string(), "bbb".to_string()]),
        id: None,
        mode: Mode::Controller,
    };
    let instance_out = Instance {
        vip_name: "fifo_out".to_string(),
        connected_to: Vec::from(["aaa".to_string(), "bbb".to_string()]),
        id: None,
        mode: Mode::Responder,
    };
    let instances: Instances = vec![instance_in, instance_out].into();
    let instances_str = toml::to_string(&instances).unwrap();
    println!("{}", instances_str);

    let instances_str = std::fs::read_to_string("examples/fifo/instances.toml").unwrap();
    let mut instances: Instances = toml::from_str(&instances_str).unwrap();
    println!("{:#?}", instances);
    instances.estimate_ids();
    println!("{:#?}", instances);
*/
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
    let cfg = common::Common {
        dut_path: "fifo.sv".to_string(),
        generate_file_header: false,
        top_default_sequence: 10,
    };
    let common = toml::to_string(&cfg).unwrap();
    println!("{}", common);

    let common_file = std::fs::read_to_string("common.toml").unwrap();
    let cfg: common::Common = toml::from_str(&common_file).unwrap();
    println!("{:#?}", cfg);
*/
/*
    let pl = PinList {
        top_wire_dec: HashSet::from(["[31:0] random_top_signal".to_string()]),
        top_param_dec: HashSet::from(["my_param 52".to_string()]),
        global_map: HashMap::from([("clk".to_string(), "clock".to_string())]),
        interface_map: HashMap::from([
            ("agent_test_0".to_string(), HashMap::from([
                ("data_rdy".to_string(),"data_rdy_0".to_string()),
                ("data_vld".to_string(),"data_vld_0".to_string()),
                ("data".to_string(),"data_0".to_string()),
            ])),
            ("agent_test_1".to_string(), HashMap::from([
                ("data_rdy".to_string(),"data_rdy_1".to_string()),
                ("data_vld".to_string(),"data_vld_1".to_string()),
                ("data".to_string(),"data_1".to_string()),
            ])),
        ]),
    };
    let pinlist = toml::to_string(&pl).unwrap();
    println!("{}", pinlist);
    
    let pinlist_file = std::fs::read_to_string("pinlist.toml").unwrap();
    let pl: PinList = toml::from_str(&pinlist_file).unwrap();
    println!("{:#?}", pl);
*/
/*
    let tpl = template::Template {
        agent: template::agent::Agent {
            name: "agent_test".to_string(),
            is_active: true,
            number_of_instances: 2,
        },
        interface: template::interface::Interface {
            ports: HashSet::from(["data_vld".to_string(), "data_rdy".to_string(), "data [31:0]".to_string()]),
            clock: "clk".to_string(),
            reset: "rst".to_string(),
            use_clock_block: true,
        },
        item: template::item::Item {
            name: "item_test".to_string(),
            members: HashSet::from(["rand bit[31:0] data".to_string(), "rand int delay".to_string()]),
            constraints: HashSet::from(["delay inside {[1:100]}".to_string()]),
        },
    };
    let template = toml::to_string(&tpl).unwrap();
    println!("{}", template);
*/
    //let tpl: template::Template = toml::from_str("").unwrap();
    //println!("{:#?}", tpl);
    //let yaml = serde_yaml::to_string(&cfg).unwrap();
    //println!("{}", yaml);
/*
    let dut = dut_parser::parse_dut(cfg.dut_path);
    //println!("{:#?}", dut);

    let th = uvm::th::TestHarness {
        interfaces: HashMap::new(),
        dut,
    };
    println!("{:#?}", th);
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
    let yaml = serde_yaml::to_string(&class).unwrap();
    println!("{}", yaml);
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

