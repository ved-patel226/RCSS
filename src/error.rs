use std::fmt;
use std::path::PathBuf;
use colored::Colorize;

/// The different types of errors that can occur in RCSS
#[derive(Debug)]
pub enum RCSSError {
    IoError(std::io::Error),
    ParseError {
        file_path: PathBuf,
        line: usize,
        column: usize,
        message: String,
        context: String,
    },
    CompilationError {
        file_path: PathBuf,
        message: String,
    },
    ConfigError(String),
    ImportError {
        file_path: PathBuf,
        import_path: String,
        message: String,
    },
    VariableError {
        file_path: PathBuf,
        variable_name: String,
        message: String,
    },
    FunctionError {
        file_path: PathBuf,
        function_name: String,
        message: String,
    },
}

impl fmt::Display for RCSSError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RCSSError::IoError(err) => write!(f, "IO Error: {}", err),
            RCSSError::ParseError { file_path, line, column, message } => {
                write!(
                    f,
                    "Parse Error at {}:{}:{} - {}",
                    file_path.display(),
                    line,
                    column,
                    message
                )
            }
            RCSSError::CompilationError { file_path, message } => {
                write!(f, "Compilation Error in {} - {}", file_path.display(), message)
            }
            RCSSError::ConfigError(message) => { write!(f, "Configuration Error: {}", message) }
            RCSSError::ImportError { file_path, import_path, message } => {
                write!(
                    f,
                    "Import Error in {} importing '{}' - {}",
                    file_path.display(),
                    import_path,
                    message
                )
            }
            RCSSError::VariableError { file_path, variable_name, message } => {
                write!(
                    f,
                    "Variable Error in {} for '{}' - {}",
                    file_path.display(),
                    variable_name,
                    message
                )
            }
            RCSSError::FunctionError { file_path, function_name, message } => {
                write!(
                    f,
                    "Function Error in {} for '{}' - {}",
                    file_path.display(),
                    function_name,
                    message
                )
            }
        }
    }
}

impl std::error::Error for RCSSError {}

impl From<std::io::Error> for RCSSError {
    fn from(error: std::io::Error) -> Self {
        RCSSError::IoError(error)
    }
}

/// Displays a stylized error message to the console
pub fn display_error(error: &RCSSError) {
    let error_title = match error {
        RCSSError::IoError(_) => "I/O ERROR",
        RCSSError::ParseError { .. } => "PARSE ERROR",
        RCSSError::CompilationError { .. } => "COMPILATION ERROR",
        RCSSError::ConfigError(_) => "CONFIG ERROR",
        RCSSError::ImportError { .. } => "IMPORT ERROR",
        RCSSError::VariableError { .. } => "VARIABLE ERROR",
        RCSSError::FunctionError { .. } => "FUNCTION ERROR",
    };

    // Create the header
    let header = format!(" {} ", error_title).black().on_red().bold();
    let top_border = "═".repeat(error_title.len() + 2).red();

    println!("\n╔{}╗", top_border);
    println!("║{}║", header);
    println!("╚{}╝\n", top_border);

    // Display the error message
    match error {
        RCSSError::IoError(err) => {
            println!("{}", " File System Error ".red().bold());
            println!("{}", err);
        }

        RCSSError::ParseError { file_path, line, column, message } => {
            println!("{}", " Location ".yellow().bold());
            println!(
                "  {} at line {} column {}",
                file_path.display().to_string().blue(),
                line,
                column
            );

            println!("\n{}", " Message ".yellow().bold());
            println!("  {}", message);
        }

        RCSSError::CompilationError { file_path, message } => {
            println!("{}", " File ".yellow().bold());
            println!("  {}", file_path.display().to_string().blue());

            println!("\n{}", " Message ".yellow().bold());
            println!("  {}", message);
        }

        RCSSError::ConfigError(message) => {
            println!("{}", " Configuration Issue ".yellow().bold());
            println!("  {}", message);
        }

        RCSSError::ImportError { file_path, import_path, message } => {
            println!("{}", " In File ".yellow().bold());
            println!("  {}", file_path.display().to_string().blue());

            println!("\n{}", " Import Path ".yellow().bold());
            println!("  {}", import_path);

            println!("\n{}", " Message ".yellow().bold());
            println!("  {}", message);
        }

        RCSSError::VariableError { file_path, variable_name, message } => {
            println!("{}", " In File ".yellow().bold());
            println!("  {}", file_path.display().to_string().blue());

            println!("\n{}", " Variable ".yellow().bold());
            println!("  {}", variable_name);

            println!("\n{}", " Message ".yellow().bold());
            println!("  {}", message);
        }

        RCSSError::FunctionError { file_path, function_name, message } => {
            println!("{}", " In File ".yellow().bold());
            println!("  {}", file_path.display().to_string().blue());

            println!("\n{}", " Function ".yellow().bold());
            println!("  {}", function_name);

            println!("\n{}", " Message ".yellow().bold());
            println!("  {}", message);
        }
    }

    println!("\n{}\n", "For help, check the documentation or open an issue on GitHub.".dimmed());
}

/// For displaying warnings that aren't critical errors
pub fn display_warning(message: &str) {
    let header = " WARNING ".black().on_yellow().bold();
    let top_border = "═".repeat(9).yellow();

    println!("\n╔{}╗", top_border);
    println!("║{}║", header);
    println!("╚{}╝\n", top_border);

    println!("  {}\n", message);
}
/// A Result type using RCSSError
pub type Result<T> = std::result::Result<T, RCSSError>;
