use crate::data_processing::format_vectors;
use crate::models::{Assay, DeviceType};
use crate::utils::stringify_collection;

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
// TODO: is there a need to have axes limits for graph?
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
        let cycle_str = self
            .cycle
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        println!("{:?}", cycle_str);
        filled = filled.replace("{{cycle}}", &cycle_str);
        filled = filled.replace("{{datetime}}", &self.datetime);
        filled = filled.replace("{{units}}", &self.units);
        // derive a list of tags to replace in the for {{tag_i}}
        let test_tags = Self::generate_tags(String::from("test"), self.test.len());
        let value1_tags = Self::generate_tags(String::from("value1"), self.value1.len());
        let upper1_tags = Self::generate_tags(String::from("upper1"), self.upper1.len());
        let lower1_tags = Self::generate_tags(String::from("lower1"), self.lower1.len());
        let group1values_tags =
            Self::generate_tags(String::from("groupvalues1"), self.group1values.len());
        let value2_tags = Self::generate_tags(String::from("value2"), self.value2.len());
        let upper2_tags = Self::generate_tags(String::from("upper2"), self.upper2.len());
        let lower2_tags = Self::generate_tags(String::from("lower2"), self.lower2.len());
        let group2values_tags =
            Self::generate_tags(String::from("group2values"), self.group2values.len());
        let table_tags = Self::generate_tags(String::from("table"), self.find_number_of_tests());
        filled = self
            .value1
            .iter()
            .enumerate()
            .fold(filled, |acc, (i, item)| {
                value1_tags
                    .get(i)
                    .map_or(acc.clone(), |tag| acc.replace(tag, &item.to_string()))
            });
        filled = self.test.iter().enumerate().fold(filled, |acc, (i, item)| {
            test_tags
                .get(i)
                .map_or(acc.clone(), |tag| acc.replace(tag, &format!("{:?}", item)))
        });
        filled = self
            .upper1
            .iter()
            .enumerate()
            .fold(filled, |acc, (i, item)| {
                upper1_tags.get(i).map_or(acc.clone(), |tag| {
                    acc.replace(tag, &format!("{:?}", &item.to_string()))
                })
            });
        filled = self
            .lower1
            .iter()
            .enumerate()
            .fold(filled, |acc, (i, item)| {
                lower1_tags.get(i).map_or(acc.clone(), |tag| {
                    acc.replace(tag, &format!("{:?}", &item.to_string()))
                })
            });
        filled = self
            .value2
            .iter()
            .enumerate()
            .fold(filled, |acc, (i, item)| {
                value2_tags
                    .get(i)
                    .map_or(acc.clone(), |tag| acc.replace(tag, &item.to_string()))
            });
        filled = self
            .upper2
            .iter()
            .enumerate()
            .fold(filled, |acc, (i, item)| {
                upper2_tags.get(i).map_or(acc.clone(), |tag| {
                    acc.replace(tag, &format!("{:?}", &item.to_string()))
                })
            });
        filled = self
            .lower2
            .iter()
            .enumerate()
            .fold(filled, |acc, (i, item)| {
                lower2_tags.get(i).map_or(acc.clone(), |tag| {
                    acc.replace(tag, &format!("{:?}", &item.to_string()))
                })
            });
        filled = self
            .group1values
            .iter()
            .enumerate()
            .fold(filled, |acc, (i, item)| {
                group1values_tags.get(i).map_or(acc.clone(), |tag| {
                    acc.replace(tag, &stringify_collection(item))
                })
            });
        filled = self
            .group2values
            .iter()
            .enumerate()
            .fold(filled, |acc, (i, item)| {
                group2values_tags.get(i).map_or(acc.clone(), |tag| {
                    acc.replace(tag, &stringify_collection(item))
                })
            });
        filled = table_tags
            .iter()
            .enumerate()
            .fold(filled, |acc, (i, item)| {
                table_tags.get(i).map_or(acc.clone(), |tag| {
                    acc.replace(
                        tag,
                        &format_vectors(&self.group1values[i], &self.group2values[i]),
                    )
                })
            });
        filled = filled.replace(&String::from("\""), "");
        return filled;
    }

    pub fn generate_tags(tag: String, number_of_tags: usize) -> Vec<String> {
        (0..number_of_tags)
            .map(|i| format!("{{{{{}_{}}}}}", tag, i))
            .collect()
    }

    fn find_number_of_tests(&self) -> usize {
        return self.test.len();
    }
}
mod tests {
    use super::*;
    use crate::data_processing::format_vectors;
    use crate::models::{Assay, DeviceType};
    use std::fs;

    #[test]
    fn test_template_filling() {
        // Read the template file
        let template_content = fs::read_to_string("./Template/test_template.tex")
            .expect("Failed to read test_template.tex");

        // Create a FillTemplate instance with test data
        let fill_template = FillTemplate::new(
            template_content,
            "Lab A".to_string(),
            DeviceType::Roche, // Example device type
            "12345".to_string(),
            vec![Assay::Troponin, Assay::Troponin], // Example assays
            vec!["Cycle 1".to_string(), "Cycle 2".to_string()],
            "2024-01-18".to_string(),
            vec![1.0, 2.0],
            vec![1.1, 2.1],
            vec![0.9, 1.9],
            vec![vec![1.0, 2.0], vec![3.0, 4.0]],
            vec![5.0, 6.0],
            vec![5.1, 6.1],
            vec![4.9, 5.9],
            vec![vec![5.0, 6.0], vec![7.0, 8.0]],
            "Units".to_string(),
        );

        // Fill the template with the test data
        let filled_content = fill_template.fill();

        // Read the expected filled template file
        let expected_filled_content = fs::read_to_string("./Template/test_filled.tex")
            .expect("Failed to read test_filled.tex");

        // Compare the filled content with the expected content
        assert_eq!(
            filled_content, expected_filled_content,
            "The filled template does not match the expected content."
        );
    }
}
