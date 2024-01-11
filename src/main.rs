use csv;
use csv::StringRecord;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::process;
use std::process::Command;

#[derive(Debug, Clone)]
enum ReportType {
    SingleAnalyte,
    WBC,
    Lipids,
    BloodGas,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Assay {
    Troponin,
    CRP,
    Lactate,
    Creatinine,
    Potassium,
    Sodium,
    Chloride,
    Bicarbonate,
    Glucose,
    Hemoglobin,
    WBC,
    INR,
    Bilirubin,
    Calcium,
    Magnesium,
    Phosphorus,
    NtProBNP,
}

#[derive(Debug, Clone)]
enum DeviceType {
    Abbott,
    CobasH232,
    Roche,
    Siemens,
    Radiometer,
    Nova,
    IStat,
    Alere,
    Beckman,
    Sysmex,
    Other,
}

#[derive(Debug, Clone)]
struct Submission {
    site: String,
    devicetype: DeviceType,
    deviceid: String,
    test: Assay,
    cycle: String,
    datetime: String,
    value: f64,
    units: String,
}

// TODO: Refactor FillTemplateSingleAnalyte and FillTemplateMultiAnalyte into one struct, move to a new file
struct FillTemplateSingleAnalyte {
    template: String,
    site: String,
    devicetype: DeviceType,
    deviceid: String,
    test: Assay,
    cycle: String,
    datetime: String,
    value1: f64,
    upper1: f64,
    lower1: f64,
    group1values: Vec<f64>,
    value2: f64,
    upper2: f64,
    lower2: f64,
    group2values: Vec<f64>,
    units: String,
}

impl FillTemplateSingleAnalyte {
    fn new(
        template: String,
        site: String,
        devicetype: DeviceType,
        deviceid: String,
        test: Assay,
        cycle: String,
        datetime: String,
        value1: f64,
        upper1: f64,
        lower1: f64,
        group1values: Vec<f64>,
        value2: f64,
        upper2: f64,
        lower2: f64,
        group2values: Vec<f64>,
        units: String,
    ) -> FillTemplateSingleAnalyte {
        FillTemplateSingleAnalyte {
            template,
            site,
            devicetype,
            deviceid,
            test,
            cycle,
            datetime,
            value1,
            upper1,
            lower1,
            group1values,
            value2,
            upper2,
            lower2,
            group2values,
            units,
        }
    }

    // TODO: refactor to work with multiple analytes
    fn fill(&self) -> String {
        let mut filled = self.template.clone();
        filled = filled.replace("{{site}}", &self.site);
        filled = filled.replace("{{devicetype}}", &format!("{:?}", self.devicetype));
        filled = filled.replace("{{deviceid}}", &self.deviceid);
        filled = filled.replace("{{test}}", format!("{:?}", self.test).as_str());
        filled = filled.replace("{{cycle}}", format!("{:?}", self.cycle).as_str());
        filled = filled.replace("{{datetime}}", &self.datetime);
        filled = filled.replace("{{value1}}", &self.value1.to_string());
        filled = filled.replace("{{upper1}}", &self.upper1.to_string());
        filled = filled.replace("{{lower1}}", &self.lower1.to_string());
        filled = filled.replace("{{value2}}", &self.value2.to_string());
        filled = filled.replace("{{upper2}}", &self.upper2.to_string());
        filled = filled.replace("{{lower2}}", &self.lower2.to_string());
        filled = filled.replace("{{units}}", &self.units);
        filled = filled.replace(
            "{{group1values}}",
            &self
                .group1values
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(", "),
        );
        filled = filled.replace(
            "{{group2values}}",
            &self
                .group2values
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(", "),
        );
        filled = filled.replace(
            "{{table}}",
            &format_vectors(&self.group1values, &self.group2values),
        );
        return filled;
    }
}

struct FillTemplateMultiAnalyte {
    template: String,
    site: String,
    devicetype: DeviceType,
    deviceid: String,
    test: Vec<Assay>,
    cycle: String,
    datetime: String,
    value1: Vec<f64>,
    upper1: Vec<f64>,
    lower1: Vec<f64>,
    group1values: Vec<Vec<f64>>,
    value2: Vec<f64>,
    upper2: Vec<f64>,
    lower2: Vec<f64>,
    group2values: Vec<Vec<f64>>,
    units: String,
}

impl FillTemplateMultiAnalyte {
    fn find_number_of_tests(&self) -> usize {
        return self.test.len();
    }

    fn find_fields(&self, field_name: String) -> Vec<String> {
        let mut fields: Vec<String> = Vec::new();
        for i in 1..self.find_number_of_tests() {
            fields.push(format!("{{{{{}_{}}}}}", field_name, i));
        }
        return fields;
    }

    fn fill(&self) {}
}
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

fn record_to_submission(record: StringRecord) -> Result<Submission, Box<dyn Error>> {
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

fn write_filltemplate_to_file(
    filltemplate: &FillTemplateSingleAnalyte,
    new_dir: &str,
) -> Result<(), Box<dyn Error>> {
    let filled = filltemplate.fill();
    let file_name = format!(
        "{}{:?}{}.tex",
        filltemplate.site, filltemplate.test, filltemplate.cycle
    );
    let file_path = Path::new(new_dir).join(file_name); // Example filename
    let mut file = File::create(&file_path)?;
    file.write_all(filled.as_bytes())?;
    return Ok(());
}
fn find_all_cycles(data: &Vec<Submission>) -> Vec<String> {
    let mut cycles: Vec<String> = data.iter().map(|x| x.cycle.clone()).collect();
    cycles.sort();
    cycles.dedup();
    return cycles;
}

fn find_all_sites(data: &Vec<Submission>) -> Vec<String> {
    let mut sites: Vec<String> = data.iter().map(|x| x.site.clone()).collect();
    sites.sort();
    sites.dedup();
    return sites;
}

fn find_group_values(data: &Vec<Submission>, cycle: &String) -> Vec<f64> {
    let cycle_data: Vec<Submission> = data
        .iter()
        .filter(|x| x.cycle == cycle.to_string())
        .cloned()
        .collect();
    let mut values: Vec<f64> = cycle_data.iter().map(|x| x.value).collect();
    return values;
}

fn open_file_as_string(path: &str) -> Result<String, Box<dyn Error>> {
    let path = Path::new(&path);
    let mut file = File::open(&path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    return Ok(contents);
}

fn open_csv_as_submissions(path: String) -> Result<Vec<Submission>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(path)?;
    let submissions: Vec<Submission> = rdr
        .records()
        .map(|result| result.unwrap())
        .map(|record| record_to_submission(record).unwrap())
        .collect();
    return Ok(submissions);
}

fn ask_device_type(device_type: &str) -> DeviceType {
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

fn ask_assay(assay: &str) -> Assay {
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

fn cli_input() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to readline");

    let input: String = input.trim().to_string();
    return input;
}

fn calculate_cycle_mean(file: &Vec<Submission>, cycle: String) -> f64 {
    let cycle_data: Vec<Submission> = file.iter().filter(|x| x.cycle == cycle).cloned().collect();
    let sum: f64 = cycle_data.iter().map(|x| x.value).sum();
    let count: f64 = cycle_data.len() as f64;
    let mean: f64 = sum / count;
    return mean;
}

fn find_allowable_range(mean: f64, assay: Assay) -> (f64, f64) {
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

fn format_vectors(v1: &Vec<f64>, v2: &Vec<f64>) -> String {
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
//
// fn report_type(reporttype: ReportType) -> fn() {
//     match reporttype {
//         ReportType::SingleAnalyte => single_analyte(),
//         ReportType::WBC => wbc(),
//         ReportType::Lipids => lipids(),
//         ReportType::BloodGas => blood_gas(),
//     }
// }
//
fn single_analyte(
    sites: Vec<String>,
    cycles: &Vec<String>,
    assay: &Vec<Assay>,
    file: Vec<Submission>,
) -> Result<Vec<FillTemplateSingleAnalyte>, Box<dyn Error>> {
    println!("Single Analyte");
    let assay = assay[0].clone();
    let filltemplates: Vec<FillTemplateSingleAnalyte> = sites
        .iter()
        .map(|site| {
            let cycle_mean1 = calculate_cycle_mean(&file, cycles[0].to_string());
            let cycle_mean2 = calculate_cycle_mean(&file, cycles[1].to_string());
            let (upper1, lower1) = find_allowable_range(cycle_mean1, assay.clone());
            let (upper2, lower2) = find_allowable_range(cycle_mean2, assay.clone());
            let group1_values = find_group_values(&file, &cycles[0]);
            let group2_values = find_group_values(&file, &cycles[1]);
            let value1 = file
                .iter()
                .filter(|x| &x.site == site)
                .filter(|x| &x.cycle == &cycles[0])
                .map(|x| x.value)
                .sum::<f64>()
                / file
                    .iter()
                    .filter(|x| &x.site == site)
                    .filter(|x| &x.cycle == &cycles[0])
                    .count() as f64;
            let value2 = file
                .iter()
                .filter(|x| &x.site == site)
                .filter(|x| &x.cycle == &cycles[1])
                .map(|x| x.value)
                .sum::<f64>()
                / file
                    .iter()
                    .filter(|x| &x.site == site)
                    .filter(|x| &x.cycle == &cycles[1])
                    .count() as f64;
            let template_content = match open_file_as_string("./Template/template.tex") {
                Ok(template_content) => template_content,
                Err(error) => {
                    println!("Problem opening file: {:?}", error);
                    process::exit(1);
                }
            };
            let filltemplate = FillTemplateSingleAnalyte::new(
                template_content,
                site.clone(),
                file[0].devicetype.clone(),
                file[0].deviceid.clone(),
                assay.clone(),
                cycles[0].clone(),
                file[0].datetime.clone(),
                value1,
                upper1,
                lower1,
                group1_values,
                value2,
                upper2,
                lower2,
                group2_values,
                file[0].units.clone(),
            );
            return filltemplate;
        })
        .collect();
    return Ok(filltemplates);
}

fn find_assays(file: &Vec<Submission>) -> Vec<Assay> {
    let mut assays: Vec<Assay> = file.iter().map(|x| x.test.clone()).collect();
    assays.sort();
    assays.dedup();
    return assays;
}

fn wbc() {
    println!("WBC");
}

fn lipids() {
    println!("Lipids");
}

fn blood_gas() {
    println!("Blood Gas");
}
