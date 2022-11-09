use std::fs::File;
use std::io::Write;

use clap::Parser;
use log::{trace, debug, info, error};
use tera::Tera;

use kitsuvm_poc::cli;
use kitsuvm_poc::config::{parse_config_files, check_i_v_compat, check_i_v_d_compat};
use kitsuvm_poc::dut_parser;

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

    let mut render_vips = Vec::new();
    for v in &vips {
        let vip = kitsuvm_poc::render::vip::VIP::try_from(v).unwrap();
        render_vips.push(vip);
    }

    kitsuvm_poc::render::render_all(&tera_dir, &render_vips, &instances, &cli, &project);

    let dut_files_str = format!("{}.sv", dut.name);
    let output_directory_path = format!("{}/bin", cli.output.clone());
    let output_path = format!("{}/{}.txt", output_directory_path, "dut_files");
    let mut file = File::create(output_path).unwrap();
    file.write_all(dut_files_str.as_bytes()).unwrap();

    let output_path = format!("{}/{}.sv", output_directory_path, dut.name);
    std::fs::copy(project.dut.path, output_path).unwrap();
}

