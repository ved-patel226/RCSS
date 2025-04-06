use std::fmt;
use std::path::PathBuf;
use colored::Colorize;

/// The different types of errors that can occur in RCSS
#[derive(Debug)]
#[allow(unused)]
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
            RCSSError::ParseError { file_path, line, column, message, context } => {
                write!(
                    f,
                    "Parse Error at {}:{}:{} - {} (Context: {})",
                    file_path.display(),
                    line,
                    column,
                    message,
                    context
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
        RCSSError::ParseError { .. } => "SYNTAX ERROR",
        RCSSError::CompilationError { .. } => "COMPILATION ERROR",
        RCSSError::ConfigError(_) => "CONFIG ERROR",
        RCSSError::ImportError { .. } => "IMPORT ERROR",
        RCSSError::VariableError { .. } => "VARIABLE ERROR",
        RCSSError::FunctionError { .. } => "FUNCTION ERROR",
    };

    // Create the header
    let header = format!(" {} ", error_title).black().on_red().bold();
    println!("\n{}", header);

    // Display the error message
    match error {
        RCSSError::IoError(err) => {
            println!("{}", " File System Error ".red().bold());
            println!("{}", err);
        }

        RCSSError::ParseError { file_path, line, column, message, context } => {
            let location = format!("{} --> {}:{}", file_path.display(), line, column);

            println!("{}\n", location);
            let trimmed = message.split("expected").nth(1).unwrap_or(&message);
            println!("{}  Expected: {}\n", "→".red().bold(), trimmed.white().bold());

            // Display code snippet with highlighting
            let lines: Vec<&str> = context.lines().collect();
            println!("{}", "╭─────────────────────────────────────────────────────".bright_red());
            println!("{}", "│".bright_red());

            for (i, line_content) in lines.iter().enumerate() {
                let line_num = (line - 1 + i).to_string();
                println!("{} {: >3} │ {}", "│".bright_red(), line_num.white(), line_content);

                if i == 1 {
                    // Highlight the error position with an arrow
                    let mut pointer = " ".repeat(*column);
                    pointer.push('↑');
                    println!(
                        "{} {: >3} │ {}",
                        "│".bright_red(),
                        " ".bright_yellow(),
                        pointer.bright_red().bold()
                    );
                }
            }

            println!("{}", "│".bright_red());
            println!("{}", "╰─────────────────────────────────────────────────────".bright_red());
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

    println!("\n{}\n", "For help, open an issue on GitHub.".dimmed());
}

/// For displaying warnings that aren't critical errors
#[allow(unused)]
pub fn display_warning(message: &str) {
    let header = " WARNING ".black().on_yellow().bold();
    let top_border = "═".repeat(9).yellow();

    println!("{}", header);

    println!("  {}\n", message);
}
/// A Result type using RCSSError
pub type Result<T> = std::result::Result<T, RCSSError>;
