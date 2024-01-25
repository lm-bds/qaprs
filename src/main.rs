use rfd;
use std::fmt;
slint::include_modules!();
use slint::SharedString;
use std::path::{Path, PathBuf};
use std::process;
use std::process::Command;
mod models;
mod utils;
use utils::{cli_input, find_all_cycles, find_all_sites};
mod data_processing;
mod file_io;
use file_io::{open_csv_as_submissions, write_filltemplate_to_file};
mod reports;
use reports::single_analyte;
mod templates;
use data_processing::find_assays;
use std::fs;
use std::sync::mpsc;
use std::thread;

fn run_lualatex_on_directory(directory: &str) {
    println!("Running lualatex on {:?}", directory);

    let entries = fs::read_dir(directory).expect("Directory not found");

    for entry in entries {
        let entry = entry.expect("Error reading file");
        let path = entry.path();

        if path.extension().and_then(std::ffi::OsStr::to_str) == Some("tex") {
            println!("Processing {:?}", path);

            let output = Command::new("lualatex")
                .arg("-output-directory")
                .arg(directory)
                .arg(path.to_str().unwrap())
                .output()
                .expect("Failed to execute lualatex");

            if !output.status.success() {
                println!("Error processing file {:?}", path);
                println!("Error: {}", String::from_utf8_lossy(&output.stderr));
            } else {
                println!("Processed {:?}", path);
                println!("Output: {}", String::from_utf8_lossy(&output.stdout));
            }
        }
    }
}
fn generate_latex_files(file_str: String) -> String {
    let file = match open_csv_as_submissions(&file_str) {
        Ok(file_path) => file_path,
        Err(error) => {
            println!("Problem opening file: {:?} {:?}", error, &file_str);
            process::exit(1);
        }
    };
    let cycles: Vec<String> = find_all_cycles(&file);
    let sites = find_all_sites(&file);
    let assays = find_assays(&file);
    let filltemplates = single_analyte(sites, &cycles, &assays, file).unwrap();
    let mut new_dir = format!("./Output{:?}{:?}{:?}", assays, cycles[0], cycles[1]);
    new_dir = new_dir.replace("[", "");
    new_dir = new_dir.replace("]", "");
    new_dir = new_dir.replace(" ", "");
    new_dir = new_dir.replace("\"", "");
    let com = Command::new("mkdir")
        .arg(new_dir.clone())
        .output()
        .expect("failed to execute process");
    for filltemplate in filltemplates {
        write_filltemplate_to_file(&filltemplate, &new_dir);
    }
    return new_dir;
}

#[derive(Debug, Clone)]
struct Manifest(PathBuf);

impl From<PathBuf> for Manifest {
    fn from(mut directory_or_file: PathBuf) -> Self {
        if directory_or_file.is_dir() {
            directory_or_file.push("Cargo.toml");
        }
        Self(directory_or_file)
    }
}
impl fmt::Display for Manifest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // You need to provide a way to convert Manifest to a String.
        // Assuming you want to display the inner PathBuf as a string.
        write!(f, "{}", self.0.display())
    }
}

impl Manifest {
    fn directory(&self) -> Option<&Path> {
        self.0.parent()
    }
}

fn show_open_dialog(file: Manifest) -> Manifest {
    let mut dialog = rfd::FileDialog::new();
    dialog = dialog.set_title("Select a file");

    if let Some(directory) = file.directory() {
        dialog = dialog.set_directory(directory);
    }

    match dialog.pick_file() {
        Some(new_path) => new_path.into(),
        None => file,
    }
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    ui.on_show_open_dialog({
        let ui_handle = ui.as_weak();
        move || {
            if let Some(ui) = ui_handle.upgrade() {
                let manifest_str: SharedString = ui.get_manifest();
                let manifest_path: PathBuf = PathBuf::from(manifest_str.as_str());
                let manifest = Manifest(manifest_path);
                let new_manifest: Manifest = show_open_dialog(manifest);

                // Assuming Manifest implements Display or similar for conversion to SharedString
                ui.set_manifest(new_manifest.to_string().into());
            }
        }
    });
    ui.on_generate_latex_files({
        let ui_handle = ui.as_weak();
        move |_| {
            // Only if the UI is still available
            if let Some(ui) = ui_handle.upgrade() {
                let manifest_str = ui.get_manifest().to_string();
                let (tx, rx) = mpsc::channel();
                // Use a separate thread to run the long task
                std::thread::spawn(move || {
                    let output_directory = generate_latex_files(manifest_str);
                    tx.send(output_directory).unwrap();
                });
                let output_directory = rx.recv().unwrap();
                ui.set_output_directory(output_directory.into());
            }
        }
    });

    ui.on_run_lualatex_on_directory({
        let ui_handle = ui.as_weak();
        move |_| {
            // Only if the UI is still available
            if let Some(ui) = ui_handle.upgrade() {
                let output_directory = ui.get_output_directory().to_string();
                // Use a separate thread to run the long task
                std::thread::spawn(move || {
                    run_lualatex_on_directory(&output_directory);
                });
            }
        }
    });

    ui.run()
}
