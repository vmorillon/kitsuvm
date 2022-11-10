use std::fs::File;
use std::io::Write;

use clap::Parser;
use log::{trace, debug, info, error};
use tera::Tera;

use kitsuvm_poc::cli::Args;
use kitsuvm_poc::config::{parse_config_files, parse_vip_files, parse_project_file, check_i_v_compat, check_i_v_d_compat, instance::get_self_test_instances};
use kitsuvm_poc::dut::parser::parse_dut;
use kitsuvm_poc::render::{render_top, render_self_test, render_vips, get_tera_dir, vip::get_render_vips};

fn main() {
    env_logger::init();
    debug!("starting up");

    debug!("parsing cli");
    let cli = Args::parse();
    trace!("cli parsed:\n{:#?}", cli);

    let tera_dir = get_tera_dir(&cli);

    if !cli.no_self_test {
        gen_self_test(&cli, &tera_dir);
    }
    if !cli.no_top {
        gen_top(&cli, &tera_dir);
    }
    if !cli.no_vips {
        gen_vips(&cli, &tera_dir);
    }
}

fn gen_self_test(cli: &Args, tera_dir: &Tera) {
    info!("generating self-test");
    let vips = parse_vip_files(&cli.vips);
    let project = parse_project_file(cli.project.clone());
    let vips = get_render_vips(&vips);

    for v in &vips {
        let instances = get_self_test_instances(v);

        debug!("rendering self-test {}", v.name);
        render_self_test(&tera_dir, &v, &instances, cli, &project);
    }
}

fn gen_top(cli: &Args, tera_dir: &Tera) {
    info!("generating top");
    let (project, mut instances, vips) = parse_config_files(&cli);
    instances.estimate_ids();
    check_i_v_compat(&instances, &vips);

    let dut = parse_dut(&project.dut);
    check_i_v_d_compat(&instances, &vips, &dut);

    let vips = get_render_vips(&vips);

    debug!("rendering top");
    render_top(&tera_dir, &vips, &instances, &cli, &project);

    let dut_files_str = format!("{}.sv", dut.name);
    let output_directory_path = format!("{}/dut", cli.output.clone());
    info!("creating directory {}", output_directory_path);
    std::fs::create_dir_all(&output_directory_path).unwrap();

    let output_path = format!("{}/{}.txt", output_directory_path, "dut_files");
    debug!("writing {}", output_path);

    let mut file = File::create(output_path).unwrap();
    file.write_all(dut_files_str.as_bytes()).unwrap();

    let output_path = format!("{}/{}.sv", output_directory_path, dut.name);
    debug!("copying {} to {}", project.dut.path, output_path);
    std::fs::copy(project.dut.path, output_path).unwrap();
}

fn gen_vips(cli: &Args, tera_dir: &Tera) {
    info!("generating vips");

    let vips = parse_vip_files(&cli.vips);
    let vips = get_render_vips(&vips);

    debug!("rendering vips");
    render_vips(&tera_dir, &vips, cli);
}
