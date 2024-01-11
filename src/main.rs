use csv;
use csv::StringRecord;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::process;
use std::process::Command;
mod models;
mod utils;
use utils::{ask_assay, ask_device_type, cli_input};
mod data_processing;
use data_processing::{
    calculate_cycle_mean, find_all_cycles, find_all_sites, find_allowable_range, find_group_values,
    single_analyte,
};
mod file_io;
use file_io::{open_csv_as_submissions, open_file_as_string, write_filltemplate_to_file};
mod reports;
use models::{Assay, DeviceType, Submission};
use reports::find_assays;
mod templates;
use templates::{FillTemplateMultiAnalyte, FillTemplateSingleAnalyte};

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
