use std::process;
use std::process::Command;
mod models;
mod utils;
use utils::{cli_input, find_all_cycles, find_all_sites};
mod data_processing;
mod file_io;
use file_io::{open_csv_as_submissions, write_filltemplate_to_file};
mod reports;
use reports::single_analyte;
mod templates;
use data_processing::find_assays;

fn main() {
    println!("Please input file path: ");
    let file = match open_csv_as_submissions(cli_input()) {
        Ok(file_path) => file_path,
        Err(error) => {
            println!("Problem opening file: {:?}", error);
            process::exit(1);
        }
    };
    println!("Please input assay: ");
    let assay = find_assays(&file);
    let cycles: Vec<String> = find_all_cycles(&file);
    let sites = find_all_sites(&file);
    let filltemplates = single_analyte(sites, &cycles, &assay, file).unwrap();
    let new_dir = format!("./Output{:?}{:?}{:?}", assay, cycles[0], cycles[1]);
    let com = Command::new("mkdir")
        .arg(new_dir.clone())
        .output()
        .expect("failed to execute process");
    for filltemplate in filltemplates {
        write_filltemplate_to_file(&filltemplate, &new_dir);
    }
}
