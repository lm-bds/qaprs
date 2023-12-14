
use polars::prelude::*;
use plotly::{Plot, Scatter, ImageFormat};

fn main() {
    let df: DataFrame   = read_csv()
        .expect("Error reading csv file"); 
    //let string_counts: std::prelude::v1::Result<DataFrame, PolarsError> = df.group_by("Test").expect("Error in groupby")
    //                .count()
    //                .unwrap_or_else(|e| panic!("Error in groupby: {}", e))
    //                .sort("count", false, false); // Sort by 'count' in descending order

    // Get the most frequent
    //let test_name: DataFrame = string_counts.expect("Issue with test_name").head(Some(1));
    let values: &ChunkedArray<Float64Type> = df.column("Value").unwrap().f64().unwrap();
    let values_vec: Vec<f64> = values.into_iter().filter_map(|x| x).collect();

    let median: f64 = values.median().unwrap(); 
    println!("{:?}", median);
    for idx in 0..df.height() {
        let site_name_str = df.column("Site").expect("issue").utf8().expect("issue").get(idx).ok_or("Missing value for site name").expect("Issue");
        let site_name = site_name_str.to_string();

        let graph_filename = format!("graph_for_{}.png", site_name);
        draw_graph(&values_vec, site_name);
        create_pdf(plot);

    }
}

fn read_csv() -> PolarsResult<DataFrame> {
    let df = CsvReader::from_path(".\\data\\Database.csv")?.has_header(true).finish();
    df
}

fn draw_graph(values_vec: &Vec<f64>, site_name: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut plot = Plot::new();
    let trace = Scatter::new(vec![0..values_vec.len()], values_vec.to_vec());
    plot.add_trace(trace);

    plot.write_image("out.png", ImageFormat::PNG, 800, 600, 1.0);
    plot
}

fn create_pdf() {
    // File path and font path
    let file_path = "out.pdf";
    let font_path = "Lato-Regular.ttf"; // Ensure this font file is available in your project directory

    // Create a new PDF document
    let root_area = PdfBackend::new(file_path, (800, 600)).into_drawing_area();

    // Draw the scatter plot
    root_area.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&root_area)
        .caption("Linear Progression: A Scatter Plot Visualization", ("sans-serif", 50).into_font())
        .build_cartesian_2d(0..10, 0..11)
        .unwrap();

    chart.configure_mesh().draw().unwrap();
    chart.draw_series(PointSeries::of_element(
        (0..10).map(|x| (x, x + 1)),
        5, 
        &RED,
        &|coord, size, style| {
            EmptyElement::at(coord) + Circle::new((0, 0), size, style.filled())
        },
    )).unwrap();

    // Draw the title and explanation text
    root_area.titled("Linear Progression: A Scatter Plot Visualization", ("sans-serif", 20)).unwrap();
    
    let explanation = "This scatter plot graphically represents a simple linear progression. \
                       Each point on the plot corresponds to a pair of x and y values, where y is directly proportional to x. \
                       This visualization aids in understanding how two variables may correlate in a linear manner. \
                       In this case, as x increases by 1, y also increases by 1, illustrating a perfect linear relationship with no deviations.";
    root_area.draw_text(explanation, &("sans-serif", 15), &BLACK).unwrap();

    // Draw the table
    let table_data = [
        ("X-Value", "Y-Value"),
        ("0", "1"),
        // ... add all other rows here ...
        ("9", "10"),
    ];

    let table_font = ("sans-serif", 15);
    for (i, (x, y)) in table_data.iter().enumerate() {
        let y_position = 550 - i as i32 * 20;
        root_area.draw_text(x, &table_font, &BLACK).unwrap();
        root_area.draw_text(y, &table_font, &BLACK).unwrap();
    }

    // Finalize the document
    root_area.present().unwrap();
    println!("PDF has been created");
}
