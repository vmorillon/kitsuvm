use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use clap::Parser;
use serde::{Deserialize, Serialize};
use tera::Tera;
use toml;

use kitsuvm_poc::config::{common, template};
use kitsuvm_poc::dut_parser;
use kitsuvm_poc::uvm;
/*
#[derive(Serialize, Default, Debug)]
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
/*
#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
struct PinList {
    #[serde(default = "default_top_dec")]
    top_wire_dec: HashSet<String>,
    #[serde(default = "default_top_dec")]
    top_param_dec: HashSet<String>,
    global_map: HashMap<String, String>,
    interface_map: HashMap<String, HashMap<String, String>>,
}

fn default_top_dec() -> HashSet<String> {
    HashSet::new()
}
*/

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Relative path to common config file
    #[arg(short, long, default_value = "./common.toml" )]
    common: String,
    /// Relative path to pinlist file
    #[arg(short, long, default_value = "./pinlist.toml" )]
    pinlist: String,

    /// Relative path to template files
    #[arg(required = true)]
    templates: Vec<String>,
}

fn main() {
    let cli = Args::parse();

    println!("{:#?}", cli);
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
    let templates_path = "templates/**/*.sv.j2";
    let mut tera_dir = Tera::new(templates_path).unwrap();
    tera_dir.autoescape_on(vec![]);

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

    match tera_dir.render("driver.sv", &context) {
        Ok(render) => {
            println!("{}", render);
            let mut file = File::create("test_out.sv").unwrap();
            file.write_all(render.as_bytes());
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

