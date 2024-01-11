pub fn open_file_as_string(path: &str) -> Result<String, Box<dyn Error>> {
    let path = Path::new(&path);
    let mut file = File::open(&path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    return Ok(contents);
}

pub fn open_csv_as_submissions(path: String) -> Result<Vec<Submission>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(path)?;
    let submissions: Vec<Submission> = rdr
        .records()
        .map(|result| result.unwrap())
        .map(|record| record_to_submission(record).unwrap())
        .collect();
    return Ok(submissions);
}

pub fn write_filltemplate_to_file(
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
