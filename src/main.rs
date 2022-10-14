use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use serde::Serialize;
use sv_parser::{parse_sv, unwrap_node, Locate, RefNode};
use tera::Tera;

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

fn main() {
    let path = "fifo.sv";
    let defines = HashMap::new();
    let includes: Vec<PathBuf> = Vec::new();

    let result = parse_sv(&path, &defines, &includes, false, false);

    if let Ok((syntax_tree, _def)) = result {
        for node in &syntax_tree {
            match node {
                RefNode::ModuleDeclarationAnsi(x) => {
                    let id = unwrap_node!(x, ModuleIdentifier).unwrap();
                    let id = get_identifier(id).unwrap();
                    let id = syntax_tree.get_str(&id).unwrap();
                    println!("module: {}", id);
                }
                RefNode::AnsiPortDeclarationNet(x) => {
                    let id = unwrap_node!(x, PortIdentifier).unwrap();
                    let id = get_identifier(id).unwrap();
                    let id = syntax_tree.get_str(&id).unwrap();
                    println!("port: {}", id);
                }
                _ => (),
            }
        }
    } else {
        println!("parse failed");
    }

    let mut tera = Tera::new("templates/*.sv").unwrap();
    tera.autoescape_on(vec![]);

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

    let mut context = tera::Context::new();
    context.insert("class", &class);
    context.insert("project_name", "better_easier_uvm");
    context.insert("header", &true);

    match tera.render("driver.sv", &context) {
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
}

fn get_identifier(node: RefNode) -> Option<Locate> {
    // unwrap_node! can take multiple types
    match unwrap_node!(node, SimpleIdentifier, EscapedIdentifier) {
        Some(RefNode::SimpleIdentifier(x)) => {
            return Some(x.nodes.0);
        }
        Some(RefNode::EscapedIdentifier(x)) => {
            return Some(x.nodes.0);
        }
        _ => None,
    }
}
