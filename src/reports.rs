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
pub fn single_analyte(
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
pub fn wbc() {
    println!("WBC");
}

pub fn lipids() {
    println!("Lipids");
}

pub fn blood_gas() {
    println!("Blood Gas");
}
