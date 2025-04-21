// RCSS Project File Imports
mod compile;
mod error;

pub mod process_x {
    pub mod variables;
    pub mod rule_normal;
    pub mod functions;
    pub mod keyframes;
    pub mod imports;
}

use process_x::{ variables, rule_normal, functions, keyframes, imports };

use error::Result;

use clap::{ Arg, Command };
use compile::compile;
use std::path::Path;
use std::collections::HashMap;

use notify::event::{ AccessKind, AccessMode };
use notify::{ recommended_watcher, Event, RecursiveMode, Watcher, EventKind };
use std::sync::mpsc;

#[derive(Debug, Clone)]
#[allow(unused)]
pub enum MetaData {
    Variables {
        name: String,
        value: String,
    },
    Function {
        name: String,
        body: Vec<String>,
    },
    Keyframes {
        name: String,
        body: HashMap<String, Vec<String>>,
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
    let css_input_path = rcss_input_path.join("../css");

    if !css_input_path.exists() {
        std::fs::create_dir_all(&css_input_path)?;
    }
    let css_input_path = css_input_path.canonicalize()?;

    let verbose = matches.get_flag("verbose");

    let mut project_meta_data: HashMap<String, Vec<MetaData>> = HashMap::new();

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
                rcss_input_path.to_str().unwrap(),
                &mut project_meta_data,
                verbose,
                true
            )
        {
            initial_compile_errors += 1;
        }
    }

    if initial_compile_errors > 0 {
        println!("Stopping execution due to initial errors. Fix above before continuing..");
        std::process::exit(1);
    } else {
        println!(
            "Initial check successful. Watching {} for changes...",
            &rcss_input_path.display()
        );
    }

    let (tx, rx) = mpsc::channel::<notify::Result<Event>>();

    let mut watcher = recommended_watcher(tx).map_err(|e|
        std::io::Error::new(std::io::ErrorKind::Other, e)
    )?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher
        .watch(Path::new(input_path), RecursiveMode::Recursive)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    let mut dynamic_err_count = 0;

    for res in rx {
        match res {
            Ok(path) => {
                // if file is written to
                if let EventKind::Access(AccessKind::Close(AccessMode::Write)) = path.kind {
                    if path.paths[0].extension().and_then(|s| s.to_str()) == Some("rcss") {
                        dynamic_err_count = 0;

                        let rcss_file = path.paths[0].strip_prefix(&rcss_input_path).unwrap();

                        let rcss_combined_path = rcss_input_path.join(rcss_file);
                        let css_combined_path = css_input_path
                            .join(rcss_file)
                            .with_extension("css");

                        let _ = compile(
                            rcss_combined_path.to_str().unwrap(),
                            css_combined_path.to_str().unwrap(),
                            rcss_input_path.to_str().unwrap(),
                            &mut project_meta_data,
                            verbose,
                            false
                        );
                    }
                }
            }

            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}
