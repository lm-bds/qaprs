#[derive(Debug, Clone)]
pub enum ReportType {
    SingleAnalyte,
    WBC,
    Lipids,
    BloodGas,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Assay {
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
pub enum DeviceType {
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
pub struct Submission {
    pub site: String,
    pub devicetype: DeviceType,
    pub deviceid: String,
    pub test: Assay,
    pub cycle: String,
    pub datetime: String,
    pub value: f64,
    pub units: String,
}
