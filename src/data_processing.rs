use crate::models::{Assay, Submission};
use crate::utils::{ask_assay, ask_device_type};
use csv;
use csv::StringRecord;
use std::error::Error;

pub fn find_assays(file: &Vec<Submission>) -> Vec<Assay> {
    let mut assays: Vec<Assay> = file.iter().map(|x| x.test.clone()).collect();
    assays.sort();
    assays.dedup();
    return assays;
}

pub fn format_vectors(v1: &Vec<f64>, v2: &Vec<f64>) -> String {
    if v1.len() != v2.len() {
        // Handle the case where vectors are of different lengths
        // For example, return an error message or truncate the longer vector
        return "Vectors are of different lengths".to_string();
    }

    let mut result = String::new();
    result.push_str("  x   y\n"); // Header

    for (x, y) in v1.iter().zip(v2.iter()) {
        let line = format!("{:<5} {:<5}\n", x, y); // Adjust the width as needed
        result.push_str(&line);
    }

    result
}

pub fn calculate_cycle_mean(file: &Vec<Submission>, cycle: String) -> f64 {
    let cycle_data: Vec<Submission> = file.iter().filter(|x| x.cycle == cycle).cloned().collect();
    let sum: f64 = cycle_data.iter().map(|x| x.value).sum();
    let count: f64 = cycle_data.len() as f64;
    let mean: f64 = sum / count;
    return mean;
}

pub fn find_allowable_range(mean: f64, assay: Assay) -> (f64, f64) {
    let upper = match assay {
        Assay::Troponin => {
            if mean < 50.0 {
                mean + 10.0
            } else {
                mean * 1.1
            }
        }
        Assay::CRP => mean * 1.05,
        Assay::Lactate => mean * 1.05,
        Assay::Creatinine => mean * 1.05,
        Assay::Potassium => mean * 1.05,
        Assay::Sodium => mean * 1.05,
        Assay::Chloride => mean * 1.05,
        Assay::Bicarbonate => mean * 1.05,
        Assay::Glucose => mean * 1.05,
        Assay::Hemoglobin => mean * 1.05,
        Assay::WBC => mean * 1.05,
        Assay::INR => mean * 1.05,
        Assay::Bilirubin => mean * 1.05,
        Assay::Calcium => mean * 1.05,
        Assay::Magnesium => mean * 1.05,
        Assay::Phosphorus => mean * 1.05,
        Assay::NtProBNP => mean * 1.05,
    };
    let lower = match assay {
        Assay::Troponin => {
            if mean < 50.0 {
                mean - 10.0
            } else {
                mean * 0.9
            }
        }
        Assay::CRP => mean * 1.05,
        Assay::Lactate => mean * 1.05,
        Assay::Creatinine => mean * 1.05,
        Assay::Potassium => mean * 1.05,
        Assay::Sodium => mean * 1.05,
        Assay::Chloride => mean * 1.05,
        Assay::Bicarbonate => mean * 1.05,
        Assay::Glucose => mean * 1.05,
        Assay::Hemoglobin => mean * 1.05,
        Assay::WBC => mean * 1.05,
        Assay::INR => mean * 1.05,
        Assay::Bilirubin => mean * 1.05,
        Assay::Calcium => mean * 1.05,
        Assay::Magnesium => mean * 1.05,
        Assay::Phosphorus => mean * 1.05,
        Assay::NtProBNP => mean * 1.05,
    };
    return (upper, lower);
}

pub fn record_to_submission(record: StringRecord) -> Result<Submission, Box<dyn Error>> {
    let submission = Submission {
        site: record.get(0).ok_or_else(|| "No Site Found")?.to_string(),
        devicetype: ask_device_type(record.get(1).ok_or_else(|| "No Device Type Found")?),
        deviceid: record
            .get(2)
            .ok_or_else(|| "No Device ID Found")?
            .to_string(),
        test: ask_assay(record.get(3).ok_or_else(|| "No Test Found")?),
        cycle: record.get(4).ok_or_else(|| "No Cycle Found")?.to_string(),
        datetime: record
            .get(5)
            .ok_or_else(|| "No Datetime Found")?
            .to_string(),
        value: match record.get(6) {
            Some(value_str) if !value_str.is_empty() => value_str.parse::<f64>().unwrap_or(0.0),
            _ => 0.0,
        },
        units: record.get(7).ok_or_else(|| "No Units Found")?.to_string(),
    };
    return Ok(submission);
}
