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
        line: usize,
        column: usize,
        function_name: String,
        message: String,
        context: String,
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
            RCSSError::FunctionError {
                file_path,
                line,
                column,
                function_name,
                message,
                context,
            } => {
                write!(
                    f,
                    "Function Error for func: {} at {}:{}:{} - {} (Context: {})",
                    function_name,
                    file_path.display(),
                    line,
                    column,
                    message,
                    context
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

// ...existing code...

/// Displays a stylized parse error message with code context
fn display_error_with_context(
    file_path: &std::path::Path,
    line: usize,
    column: usize,
    message: &str,
    context: &str
) {
    let location = format!("{} --> {}:{}", file_path.display(), line, column);
    let length = std::cmp::min(message.len() + 10, 110);

    println!("{}", location);

    println!("{}{}", "╭".bright_red(), "─".repeat(length).bright_red());
    println!("{}", "│".bright_red());
    println!("{}  {}", "│".bright_red(), message.white().bold());
    println!("{}", "│".bright_red());

    // Display code snippet with highlighting
    let lines: Vec<&str> = context.lines().collect();

    for (i, line_content) in lines.iter().enumerate() {
        let line_num = (line - 1 + i).to_string();
        println!("{} {: >3} │ {}", "│".bright_red(), line_num.white(), line_content);

        if i == 1 {
            // Highlight the error position with an arrow
            let mut pointer = " ".repeat(column);
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
    println!("{}{}", "╰".bright_red(), "─".repeat(length).bright_red());
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
            println!("{}", "╭─────────────────────────────────────────────────────".bright_red());
            println!("{}", "│".bright_red());
            println!("{} {}", "│".bright_red(), " File System Error ".red().bold());
            println!("{} {}", "│".bright_red(), err);
            println!("{}", "│".bright_red());
            println!("{}", "╰─────────────────────────────────────────────────────".bright_red());
        }

        RCSSError::ParseError { file_path, line, column, message, context } => {
            let trimmed = message
                .split("expected")
                .nth(1)
                .map(|s| format!("expected:{}", s))
                .unwrap_or_else(|| message.to_string());

            display_error_with_context(file_path, *line, *column, &trimmed, context);
        }

        RCSSError::CompilationError { file_path, message } => {
            println!("{}", "╭─────────────────────────────────────────────────────".bright_red());
            println!("{}", "│".bright_red());
            println!("{} {}", "│".bright_red(), " File ".red().bold());
            println!("{} {}", "│".bright_red(), file_path.display().to_string().blue());
            println!("{}", "│".bright_red());
            println!("{} {}", "│".bright_red(), " Message ".red().bold());
            println!("{} {}", "│".bright_red(), message);
            println!("{}", "│".bright_red());
            println!("{}", "╰─────────────────────────────────────────────────────".bright_red());
        }

        RCSSError::ConfigError(message) => {
            println!("{}", "╭─────────────────────────────────────────────────────".bright_red());
            println!("{}", "│".bright_red());
            println!("{} {}", "│".bright_red(), " Configuration Issue ".red().bold());
            println!("{} {}", "│".bright_red(), message);
            println!("{}", "│".bright_red());
            println!("{}", "╰─────────────────────────────────────────────────────".bright_red());
        }

        RCSSError::ImportError { file_path, import_path, message } => {
            let location = format!("{} --> {}:{}", file_path.display(), "line", "column");

            println!("{}", "╭─────────────────────────────────────────────────────".bright_red());
            println!("{}", "│".bright_red());
            println!("{} {}", "│".bright_red(), location.red().bold());
            println!("{} {}", "│".bright_red(), file_path.display().to_string().blue());
            println!("{}", "│".bright_red());
            println!("{} {}", "│".bright_red(), " Import Path ".red().bold());
            println!("{} {}", "│".bright_red(), import_path);
            println!("{}", "│".bright_red());
            println!("{} {}", "│".bright_red(), " Message ".red().bold());
            println!("{} {}", "│".bright_red(), message);
            println!("{}", "│".bright_red());
            println!("{}", "╰─────────────────────────────────────────────────────".bright_red());
        }

        RCSSError::VariableError { file_path, variable_name, message } => {
            let location = format!("{} --> {}:{}", file_path.display(), "line", "column");

            println!("{}", "╭─────────────────────────────────────────────────────".bright_red());
            println!("{}", "│".bright_red());
            println!("{} {}", "│".bright_red(), location.red().bold());
            println!("{} {}", "│".bright_red(), file_path.display().to_string().blue());
            println!("{}", "│".bright_red());
            println!("{} {}", "│".bright_red(), " Variable ".red().bold());
            println!("{} {}", "│".bright_red(), variable_name);
            println!("{}", "│".bright_red());
            println!("{} {}", "│".bright_red(), " Message ".red().bold());
            println!("{} {}", "│".bright_red(), message);
            println!("{}", "│".bright_red());
            println!("{}", "╰─────────────────────────────────────────────────────".bright_red());
        }

        RCSSError::FunctionError {
            file_path,
            function_name: _,
            message,
            line,
            column,
            context,
        } => {
            display_error_with_context(file_path, *line, *column, message, context);
        }
    }

    println!("\n{}\n", "For help, open an issue on GitHub.".dimmed());
}

pub fn get_error_context(file_content: &str, error_line: usize, context_lines: usize) -> String {
    let lines: Vec<&str> = file_content.lines().collect();

    // Calculate start and end lines for context, ensuring bounds
    let start_line = error_line.saturating_sub(context_lines);
    let end_line = std::cmp::min(error_line + context_lines, lines.len());

    // Build context string with line numbers
    let mut context = String::new();
    for i in start_line..end_line {
        if i < lines.len() {
            context.push_str(&format!("{}\n", lines[i]));
        }
    }

    context
}

/// For displaying warnings that aren't critical errors
#[allow(unused)]
pub fn display_warning(message: &str) {
    let header = " WARNING ".black().on_yellow().bold();
    let length = std::cmp::min(message.len(), 100);

    println!("\n{}", header);

    println!("{}{}", "╭".yellow(), "─".repeat(length).yellow());
    println!("{}", "│".yellow());
    println!("{} {}", "│".yellow(), message);
    println!("{}", "│".yellow());
    println!("{}{}", "╰".yellow(), "─".repeat(length).yellow());

    println!();
}

/// A Result type using RCSSError
pub type Result<T> = std::result::Result<T, RCSSError>;
