// RCSS Project File Imports
mod compile;
mod error;

use error::Result;

use clap::{ Arg, Command };
use compile::compile;
use std::path::Path;
use std::collections::HashMap;

#[derive(Debug, Clone)]
#[allow(unused)]
enum MetaData {
    Variables {
        name: String,
        value: String,
    },
    Function {
        name: String,
        body: Vec<String>,
    },
}

fn main() -> Result<()> {
    let matches = Command::new("RCSS")
        .version("0.1.1")
        .about("Bringing Rust to CSS")
        .long_about(
            "For more information and to contribute, visit: https://github.com/ved-patel226/RCSS"
        )
        .arg(Arg::new("folder").help("Input directory to watch").required(true).index(1))
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Print verbose processing information")
                .action(clap::ArgAction::SetTrue)
        )
        .get_matches();

    let input_path = Path::new(matches.get_one::<String>("folder").unwrap());
    let current_path = std::env::current_dir()?;

    let rcss_input_path = current_path.join(input_path);
    let css_input_path = rcss_input_path.join("../css").canonicalize()?;

    let verbose = matches.get_flag("verbose");

    let mut project_meta_data: HashMap<String, Vec<MetaData>> = HashMap::new();

    if !rcss_input_path.exists() {
        println!(
            "The specified RCSS input path does not exist. Creating it: {:?}",
            rcss_input_path
        );
        std::fs::create_dir_all(&rcss_input_path)?;
    }

    if !css_input_path.exists() {
        println!("The specified CSS input path does not exist. Creating it: {:?}", css_input_path);
        std::fs::create_dir_all(&css_input_path)?;
    }

    let mut rcss_files = Vec::new();

    fn collect_rcss_files(
        dir: &Path,
        rcss_files: &mut Vec<std::path::PathBuf>,
        base_path: &Path
    ) -> Result<()> {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                collect_rcss_files(&path, rcss_files, base_path)?;
            } else if path.extension().and_then(|ext| ext.to_str()) == Some("rcss") {
                if let Ok(relative_path) = path.strip_prefix(base_path) {
                    rcss_files.push(relative_path.to_path_buf());
                }
            }
        }
        Ok(())
    }

    collect_rcss_files(&rcss_input_path, &mut rcss_files, &rcss_input_path)?;

    let mut initial_compile_errors = 0;

    for rcss_file in &rcss_files {
        if
            let Err(_) = compile(
                rcss_input_path.join(rcss_file).to_str().unwrap(),
                css_input_path.join(rcss_file).with_extension("css").to_str().unwrap(),
                &project_meta_data,
                verbose,
                true // initial_compile
            )
        {
            initial_compile_errors += 1;
        }
    }

    if initial_compile_errors > 0 {
        println!(
            "Stopping execution due to initial compilation errors. Fix above before continuing.."
        );
        std::process::exit(1);
    }

    Ok(())
}
