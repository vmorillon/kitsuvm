use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::Write;

use confy;
use serde::Serialize;
use tera::Tera;

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
#[derive(Serialize, Debug)]
struct AgentInterfaceConfig {
    agent_name: String,
    agent_is_active: bool,
    number_of_instance: u32,

    item_name: String,
    item_members: HashSet<String>,
    item_constraints: HashSet<String>,

    interfaces_port: HashSet<String>,
    interface_clock: String,
}

#[derive(Serialize, Debug)]
struct CommonConfig {
    dut_path: String,
    generate_file_header: bool,
    top_default_sequence: u32,
}

fn main() {

    let cfg = CommonConfig {
        dut_path: "fifo.sv".to_string(),
        generate_file_header: true,
        top_default_sequence: 10,
    };
    confy::store_path("common.toml", &cfg).unwrap();
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

