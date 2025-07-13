//! Command-line interface for the Rust AVM

use clap::{Command, Parser, Subcommand};
use std::path::PathBuf;

mod commands;
pub use commands::*;

/// Rust AVM - Algorand Virtual Machine implementation in Rust
#[derive(Parser)]
#[command(
    name = "avm-rs",
    version = env!("CARGO_PKG_VERSION"),
    about = "Algorand Virtual Machine (AVM) implementation in Rust",
    long_about = "A complete implementation of the Algorand Virtual Machine (AVM) that can execute TEAL smart contracts. Supports assembly, disassembly, execution, validation, and debugging of TEAL programs."
)]
pub struct Cli {
    /// Global options
    #[command(flatten)]
    pub global: GlobalOptions,

    /// Subcommands
    #[command(subcommand)]
    pub command: Commands,
}

/// Global options available for all commands
#[derive(Parser)]
pub struct GlobalOptions {
    /// Enable verbose output
    #[arg(short = 'v', long = "verbose", global = true)]
    pub verbose: bool,

    /// Suppress all output except errors
    #[arg(short = 'q', long = "quiet", global = true)]
    pub quiet: bool,

    /// Output format (text, json)
    #[arg(
        long = "output-style",
        value_enum,
        default_value = "text",
        global = true
    )]
    pub format: OutputFormat,

    /// Enable colored output (auto, always, never)
    #[arg(long = "color", value_enum, default_value = "auto", global = true)]
    pub color: ColorChoice,
}

/// Output format options
#[derive(clap::ValueEnum, Clone, Debug)]
pub enum OutputFormat {
    Text,
    Json,
}

/// Color output options
#[derive(clap::ValueEnum, Clone, Debug)]
pub enum ColorChoice {
    Auto,
    Always,
    Never,
}

/// Tracing level options
#[cfg(feature = "tracing")]
#[derive(clap::ValueEnum, Clone, Debug)]
pub enum TracingLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

/// Available CLI commands
#[derive(Subcommand)]
pub enum Commands {
    /// Execute TEAL programs
    #[command(alias = "run")]
    Execute(ExecuteCommand),

    /// Assemble TEAL source to bytecode
    #[command(aliases = ["asm", "compile"])]
    Assemble(AssembleCommand),

    /// Validate TEAL programs
    #[command(alias = "check")]
    Validate(ValidateCommand),
}

/// Execute command for running TEAL programs
#[derive(Parser)]
pub struct ExecuteCommand {
    /// Input source (file path, hex bytecode, or inline TEAL)
    #[arg(value_name = "INPUT")]
    pub input: String,

    /// Input type (auto-detect, file, bytecode, inline)
    #[arg(short = 't', long = "type", value_enum, default_value = "auto")]
    pub input_type: InputType,

    /// TEAL version to use
    #[arg(short = 'V', long = "version", value_parser = clap::value_parser!(u8).range(1..=11))]
    pub version: Option<u8>,

    /// Execution mode (signature or application)
    #[arg(short = 'm', long = "mode", value_enum, default_value = "signature")]
    pub mode: ExecutionMode,

    /// Maximum cost budget
    #[arg(short = 'b', long = "budget", default_value = "100000")]
    pub budget: u64,

    /// Step through execution (debug mode)
    #[arg(short = 's', long = "step")]
    pub step: bool,

    /// Mock ledger data from JSON file
    #[arg(short = 'l', long = "ledger")]
    pub ledger: Option<PathBuf>,

    /// Transaction data from JSON file
    #[arg(short = 'x', long = "txn")]
    pub transaction: Option<PathBuf>,

    /// Arguments to pass to the program
    #[arg(short = 'a', long = "arg")]
    pub args: Vec<String>,

    /// Tracing level (trace, debug, info, warn, error) - enables tracing when specified
    #[cfg(feature = "tracing")]
    #[arg(long = "trace-level", value_enum)]
    pub trace_level: Option<TracingLevel>,

    /// Enable opcode-level tracing
    #[cfg(feature = "tracing")]
    #[arg(long = "trace-opcodes")]
    pub trace_opcodes: bool,

    /// Enable stack state tracing
    #[cfg(feature = "tracing")]
    #[arg(long = "trace-stack")]
    pub trace_stack: bool,
}

/// Assemble command for converting TEAL to bytecode
#[derive(Parser)]
pub struct AssembleCommand {
    /// Input TEAL file
    #[arg(value_name = "INPUT")]
    pub input: PathBuf,

    /// Output bytecode file
    #[arg(short = 'o', long = "output")]
    pub output: Option<PathBuf>,

    /// Output format (hex, base64, binary)
    #[arg(short = 'f', long = "output-format", value_enum, default_value = "hex")]
    pub output_format: BytecodeFormat,

    /// Optimize the bytecode
    #[arg(long = "optimize")]
    pub optimize: bool,

    /// Show assembly statistics
    #[arg(long = "stats")]
    pub stats: bool,
}

/// Disassemble command for converting bytecode to TEAL
#[derive(Parser)]
pub struct DisassembleCommand {
    /// Input bytecode (file path or hex string)
    #[arg(value_name = "INPUT")]
    pub input: String,

    /// Input format (auto, hex, base64, binary)
    #[arg(short = 'i', long = "input-format", value_enum, default_value = "auto")]
    pub input_format: BytecodeFormat,

    /// Output TEAL file
    #[arg(short = 'o', long = "output")]
    pub output: Option<PathBuf>,

    /// Include comments in output
    #[arg(short = 'c', long = "comments")]
    pub comments: bool,

    /// Show program analysis
    #[arg(long = "analyze")]
    pub analyze: bool,
}

/// Validate command for checking TEAL programs
#[derive(Parser)]
pub struct ValidateCommand {
    /// Input files to validate
    #[arg(value_name = "FILES")]
    pub files: Vec<PathBuf>,

    /// TEAL version to validate against
    #[arg(short = 'V', long = "version", value_parser = clap::value_parser!(u8).range(1..=11))]
    pub version: Option<u8>,

    /// Execution mode to validate for
    #[arg(short = 'm', long = "mode", value_enum)]
    pub mode: Option<ExecutionMode>,

    /// Strict validation (fail on warnings)
    #[arg(short = 's', long = "strict")]
    pub strict: bool,

    /// Show detailed analysis
    #[arg(long = "detailed")]
    pub detailed: bool,
}

/// Examples command for running built-in examples
#[derive(Parser)]
pub struct ExamplesCommand {
    /// List available examples
    #[arg(short = 'l', long = "list")]
    pub list: bool,

    /// Example to run
    #[arg(value_name = "EXAMPLE")]
    pub example: Option<String>,

    /// Show example source code
    #[arg(short = 's', long = "show")]
    pub show: bool,

    /// Execute the example
    #[arg(short = 'r', long = "run")]
    pub run: bool,
}

/// Info command for showing AVM information
#[derive(Parser)]
pub struct InfoCommand {
    /// Show supported TEAL versions
    #[arg(long = "versions")]
    pub versions: bool,

    /// Show available opcodes
    #[arg(long = "opcodes")]
    pub opcodes: bool,

    /// Filter opcodes by version
    #[arg(short = 'V', long = "version", value_parser = clap::value_parser!(u8).range(1..=11))]
    pub version: Option<u8>,

    /// Show opcode details
    #[arg(short = 'd', long = "details")]
    pub details: bool,

    /// Show system information
    #[arg(long = "system")]
    pub system: bool,
}

/// REPL command for interactive mode
#[derive(Parser)]
pub struct ReplCommand {
    /// TEAL version for the session
    #[arg(short = 'V', long = "version", value_parser = clap::value_parser!(u8).range(1..=11), default_value = "11")]
    pub version: u8,

    /// Execution mode
    #[arg(short = 'm', long = "mode", value_enum, default_value = "signature")]
    pub mode: ExecutionMode,

    /// Load initial script
    #[arg(short = 'l', long = "load")]
    pub load: Option<PathBuf>,
}

/// Input type enumeration
#[derive(clap::ValueEnum, Clone, Debug)]
pub enum InputType {
    /// Auto-detect input type
    Auto,
    /// File path
    File,
    /// Hex bytecode string
    Bytecode,
    /// Inline TEAL source
    Inline,
}

/// Execution mode enumeration
#[derive(clap::ValueEnum, Clone, Debug)]
pub enum ExecutionMode {
    /// Signature verification mode (stateless)
    Signature,
    /// Application mode (stateful)
    Application,
}

/// Bytecode format enumeration
#[derive(clap::ValueEnum, Clone, Debug)]
pub enum BytecodeFormat {
    /// Auto-detect format
    Auto,
    /// Hexadecimal string
    Hex,
    /// Base64 encoding
    Base64,
    /// Raw binary
    Binary,
}

impl Cli {
    /// Parse CLI arguments
    pub fn parse_args() -> Self {
        Self::parse()
    }

    /// Create CLI command structure (for building help, completions, etc.)
    pub fn command() -> Command {
        <Self as clap::CommandFactory>::command()
    }
}

/// Initialize and run the CLI application
pub fn run() -> anyhow::Result<()> {
    let cli = Cli::parse_args();

    // Handle global options
    setup_logging(&cli.global)?;
    setup_colors(&cli.global)?;

    // Dispatch to appropriate command handler
    match cli.command {
        Commands::Execute(cmd) => commands::execute::handle(cmd, &cli.global),
        Commands::Assemble(cmd) => commands::assemble::handle(cmd, &cli.global),
        Commands::Validate(cmd) => commands::validate::handle(cmd, &cli.global),
    }
}

/// Setup logging based on global options
fn setup_logging(_global: &GlobalOptions) -> anyhow::Result<()> {
    // TODO: Implement proper logging setup
    Ok(())
}

/// Setup color output based on global options
fn setup_colors(_global: &GlobalOptions) -> anyhow::Result<()> {
    // TODO: Implement color setup
    Ok(())
}
