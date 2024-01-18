use crate::data_processing::format_vectors;
use crate::models::{Assay, DeviceType, Submission};
use std::error::Error;
use std::io;
use std::process;
use std::process::Command;

pub fn find_all_cycles(data: &Vec<Submission>) -> Vec<String> {
    let mut cycles: Vec<String> = data.iter().map(|x| x.cycle.clone()).collect();
    cycles.sort();
    cycles.dedup();
    return cycles;
}

pub fn find_all_sites(data: &Vec<Submission>) -> Vec<String> {
    let mut sites: Vec<String> = data.iter().map(|x| x.site.clone()).collect();
    sites.sort();
    sites.dedup();
    return sites;
}

pub fn find_group_values(data: &Vec<Submission>, cycle: &String) -> Vec<f64> {
    let cycle_data: Vec<Submission> = data
        .iter()
        .filter(|x| x.cycle == cycle.to_string())
        .cloned()
        .collect();
    let mut values: Vec<f64> = cycle_data.iter().map(|x| x.value).collect();
    return values;
}

pub fn ask_device_type(device_type: &str) -> DeviceType {
    let device_type_enum = match device_type {
        "Abbott" => DeviceType::Abbott,
        "Roche" => DeviceType::Roche,
        "Siemens" => DeviceType::Siemens,
        "Radiometer" => DeviceType::Radiometer,
        "Nova" => DeviceType::Nova,
        "IStat" => DeviceType::IStat,
        "Alere" => DeviceType::Alere,
        "Beckman" => DeviceType::Beckman,
        "Sysmex" => DeviceType::Sysmex,
        "Other" => DeviceType::Other,
        "cobas h 232" => DeviceType::CobasH232,
        _ => {
            println!("Please input a valid device type");
            process::exit(1);
        }
    };
    return device_type_enum;
}

pub fn ask_assay(assay: &str) -> Assay {
    let assay_enum = match assay {
        "Troponin" => Assay::Troponin,
        "CRP" => Assay::CRP,
        "Lactate" => Assay::Lactate,
        "Creatinine" => Assay::Creatinine,
        "Potassium" => Assay::Potassium,
        "Sodium" => Assay::Sodium,
        "Chloride" => Assay::Chloride,
        "Bicarbonate" => Assay::Bicarbonate,
        "Glucose" => Assay::Glucose,
        "Hemoglobin" => Assay::Hemoglobin,
        "WBC" => Assay::WBC,
        "INR" => Assay::INR,
        "Bilirubin" => Assay::Bilirubin,
        "Calcium" => Assay::Calcium,
        "Magnesium" => Assay::Magnesium,
        "Phosphorus" => Assay::Phosphorus,
        "NtProBNP" => Assay::NtProBNP,
        _ => {
            println!("Please input a valid assay");
            process::exit(1);
        }
    };
    return assay_enum;
}

pub fn cli_input() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to readline");

    let input: String = input.trim().to_string();
    return input;
}
