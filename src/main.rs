use clap::{ error::Result, Arg, Command };
use std::path::Path;

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

    println!("RCSS Input Path: {:?}", rcss_input_path);
    println!("CSS Input Path: {:?}", css_input_path);

    Ok(())
}
