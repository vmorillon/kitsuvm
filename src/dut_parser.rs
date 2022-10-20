use std::path::{Path,PathBuf};
use std::collections::HashMap;
use sv_parser::{parse_sv, unwrap_node, Locate, Number, Bracket, ConstantRange, ConstantExpression, RefNode};

pub fn parse_dut<T: AsRef<Path>>(path: T) {
    let defines = HashMap::new();
    let includes: Vec<PathBuf> = Vec::new();

    let result = parse_sv(&path, &defines, &includes, false, false);

    if let Ok((syntax_tree, _def)) = result {
        for node in &syntax_tree {
            match node {
                RefNode::ModuleDeclarationAnsi(x) => {
                    let id = unwrap_node!(x, ModuleIdentifier).unwrap();
                    //println!("{:#?}", x);
                    let id = get_identifier(id).unwrap();
                    let id = syntax_tree.get_str(&id).unwrap();
                    println!("module: {}", id);

                    for p in x {
                        match p {
                            RefNode::AnsiPortDeclaration(po) => {
                                let port = unwrap_node!(po, PortIdentifier).unwrap();
                                let id = get_identifier(port).unwrap();
                                let id = syntax_tree.get_str(&id).unwrap();
                                println!("port: {}", id);

                                if let Some(range) = unwrap_node!(po, PackedDimensionRange) {
                                    let (range_end, range_start) = get_range(range).unwrap();
                                    //println!("{:#?}", range_end);
                                    let str_start = syntax_tree.get_str(&range_start).unwrap();
                                    let str_end = syntax_tree.get_str(&range_end).unwrap();
                                    let num_start = str_start.parse::<u32>().unwrap();
                                    let num_end = str_end.parse::<u32>().unwrap();
                                    println!("range: {} {}", num_end, num_start);
                                }
                                //println!("port! {:?}", po);
                            },
                            _ => ()
                        }
                    }
                }
                _ => (),
            }
        }
        //println!("{}", syntax_tree);
        //println!("{syntax_tree}");
    } else {
        println!("parse failed");
    }
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

fn get_range(node: RefNode) -> Option<(ConstantExpression, ConstantExpression)> {
    match unwrap_node!(node, ConstantRange) {
        Some(RefNode::ConstantRange(x)) => {
            return Some((x.nodes.0.clone(), x.nodes.2.clone()));
        }
        _ => None,
    }
}
