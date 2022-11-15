use std::fs::File;
use std::io::Write;
use std::path::Path;

use clap::Parser;
use log::{trace, debug, info};
use tera::Tera;

use kitsuvm::cli::Args;
use kitsuvm::config::{parse_config_files, parse_vip_files, parse_project_file, check_i_v_compat, check_i_v_d_compat, instance::get_self_test_instances};
use kitsuvm::dut::parser::parse_dut;
use kitsuvm::render::{render_top, render_self_test, render_vips, get_tera_dir, vip::{get_render_vips, set_vips_port_dir}};

fn main() {
    env_logger::init();
    debug!("starting up");

    debug!("parsing cli");
    let cli = Args::parse();
    trace!("cli parsed:\n{:#?}", cli);

    let tera_dir = get_tera_dir(&cli);

    backup_output_directory(&cli);

    if !cli.no_self_test {
        gen_self_test(&cli, &tera_dir);
    }

    gen_top_vips(&cli, &tera_dir);
}

fn backup_output_directory(cli: &Args) {
    let output_dir_path = Path::new(&cli.output);

    if output_dir_path.is_dir() {
        let backup_path = format!("{}.bck", cli.output);
        info!("{} already existing, creating backup at {}", cli.output, backup_path);

        let backup_dir_path = Path::new(&backup_path);
        if backup_dir_path.is_dir() {
            info!("backup at {} already existing, removing it", backup_path);
            std::fs::remove_dir_all(backup_dir_path).unwrap();
        }

        std::fs::rename(output_dir_path, backup_dir_path).unwrap();
    } else {
        info!("new output directory at {}", cli.output);
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

fn gen_top_vips(cli: &Args, tera_dir: &Tera) {
    if !cli.no_top {
        let (project, mut instances, vips) = parse_config_files(&cli);
        instances.estimate_ids();
        check_i_v_compat(&instances, &vips);

        let dut = parse_dut(&project.dut);
        check_i_v_d_compat(&instances, &vips, &dut);

        let mut vips = get_render_vips(&vips);

        debug!("rendering top");
        render_top(&tera_dir, &vips, &instances, &cli, &project);

        copy_dut_files(cli, dut.name.clone(), project.dut.path);

        if !cli.no_vips {
            set_vips_port_dir(&mut vips, &instances, &dut);

            debug!("rendering vips");
            render_vips(&tera_dir, &vips, cli);
        }
    } else {
        if !cli.no_vips {
            debug!("no-top option enabled, cannot check ports directions");
            let vips = parse_vip_files(&cli.vips);

            let vips = get_render_vips(&vips);

            debug!("rendering vips");
            render_vips(&tera_dir, &vips, cli);
        }
    }
}

fn copy_dut_files(cli: &Args, dut_name: String, dut_path: String) {
    let dut_files_str = format!("{}.sv", dut_name);
    let output_directory_path = format!("{}/dut", cli.output.clone());
    info!("creating directory {}", output_directory_path);
    std::fs::create_dir_all(&output_directory_path).unwrap();

    let output_path = format!("{}/{}.txt", output_directory_path, "dut_files");
    debug!("writing {}", output_path);

    let mut file = File::create(output_path).unwrap();
    file.write_all(dut_files_str.as_bytes()).unwrap();

    let output_path = format!("{}/{}.sv", output_directory_path, dut_name);
    debug!("copying {} to {}", dut_path, output_path);
    std::fs::copy(dut_path, output_path).unwrap();
}
