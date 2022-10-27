use std::path::PathBuf;
use std::collections::HashMap;
use log::{debug, info};
use sv_parser::{parse_sv, unwrap_node, SyntaxTree, Locate, RefNode, ModuleDeclarationAnsi, AnsiPortDeclaration, PortDirection};

use crate::uvm::th::{DUT, Port, PortProperties, PortDirection as PortDir};

pub fn parse_dut(path: String) -> DUT {
    info!("parsing dut {}", path);
    let defines = HashMap::new();
    let includes: Vec<PathBuf> = Vec::new();

    let (syntax_tree, _def) = parse_sv(&path, &defines, &includes, false, false).expect("failed to parse DUT file");

    let dut = get_dut(&syntax_tree).expect("DUT not found in file");
    debug!("dut parsed:\n{:#?}", dut);
    dut
}

fn get_dut(syntax_tree: &SyntaxTree) -> Option<DUT> {
    for n in syntax_tree {
        match n {
            RefNode::ModuleDeclarationAnsi(x) => {
                let name = get_dut_name(syntax_tree, x);
                let ports = get_ports(syntax_tree, x);

                return Some(DUT { name, ports });
            }
            _ => (),
        }
    }
    None
}

fn get_dut_name(syntax_tree: &SyntaxTree, module: &ModuleDeclarationAnsi) -> String {
    let id = unwrap_node!(module, ModuleIdentifier).unwrap();
    let port_locate = get_identifier(id);
    let name = syntax_tree.get_str(&port_locate).unwrap().to_string();

    name
}

fn get_ports(syntax_tree: &SyntaxTree, module: &ModuleDeclarationAnsi) -> HashMap<String, PortProperties> {
    let mut ports = HashMap::new();
    for n in module {
        match n {
            RefNode::AnsiPortDeclaration(x) => {
                let port = get_port(syntax_tree, x);
                ports.insert(port.name, port.properties);
            }
            _ => ()
        }
    }
    ports
}

fn get_port(syntax_tree: &SyntaxTree, port: &AnsiPortDeclaration) -> Port {
    let name = get_port_name(syntax_tree, port);

    let dimensions = get_dimensions(syntax_tree, port);
    let direction = get_direction(port);

    let properties = PortProperties { direction, dimensions };

    let port = Port { name, properties };
    port
}

fn get_port_name(syntax_tree: &SyntaxTree, module: &AnsiPortDeclaration) -> String {
    let id = unwrap_node!(module, PortIdentifier).unwrap();
    let port_locate = get_identifier(id);
    let name = syntax_tree.get_str(&port_locate).unwrap().to_string();

    name
}

fn get_dimensions(syntax_tree: &SyntaxTree, port: &AnsiPortDeclaration) -> Vec<(u32,u32)> {
    let mut dimensions = Vec::new();
    for n in port {
        match n {
            RefNode::ConstantRange(x) => {
                let end = syntax_tree.get_str(&x.nodes.0.clone()).unwrap();
                let end = end.parse().unwrap();
                let start = syntax_tree.get_str(&x.nodes.2.clone()).unwrap();
                let start = start.parse().unwrap();
                dimensions.push((end,start));
            }
            _ => ()
        }
    }
    dimensions
}

fn get_direction(port: &AnsiPortDeclaration) -> PortDir {
    for n in port {
        match n {
            RefNode::PortDirection(x) => {
                match x {
                    PortDirection::Input(_) => {
                        return PortDir::INPUT;
                    }
                    PortDirection::Output(_) => {
                        return PortDir::OUTPUT;
                    }
                    PortDirection::Inout(_) => {
                        return PortDir::INOUT;
                    }
                    _ => ()
                }
            }
            _ => ()
        }
    }
    PortDir::INOUT
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
