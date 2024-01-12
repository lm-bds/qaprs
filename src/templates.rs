use crate::data_processing::format_vectors;
use crate::models::{Assay, DeviceType};

// TODO: Refactor FillTemplate and FillTemplate into one pub struct, move to a new file

pub struct FillTemplate {
    pub template: String,
    pub site: String,
    pub devicetype: DeviceType,
    pub deviceid: String,
    pub test: Vec<Assay>,
    pub cycle: Vec<String>,
    pub datetime: String,
    pub value1: Vec<f64>,
    pub upper1: Vec<f64>,
    pub lower1: Vec<f64>,
    pub group1values: Vec<Vec<f64>>,
    pub value2: Vec<f64>,
    pub upper2: Vec<f64>,
    pub lower2: Vec<f64>,
    pub group2values: Vec<Vec<f64>>,
    pub units: String,
}
impl FillTemplate {
    pub fn new(
        template: String,
        site: String,
        devicetype: DeviceType,
        deviceid: String,
        test: Vec<Assay>,
        cycle: Vec<String>,
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
    ) -> FillTemplate {
        FillTemplate {
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
    pub fn fill(&self) -> String {
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
}
