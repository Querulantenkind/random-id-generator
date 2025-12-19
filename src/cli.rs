use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "idgen")]
#[command(about = "A CLI tool to generate random IDs", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Generate multiple IDs
    #[arg(short, long, default_value_t = 1)]
    pub count: usize,

    /// Copy the output to clipboard (if generating single ID, copies the ID. If multiple, copies list)
    #[arg(long)]
    pub copy: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate a UUID v4
    Uuid,
    /// Generate a NanoID
    Nanoid,
    /// Generate a custom random string
    Custom {
        /// Length of the string
        #[arg(short, long, default_value_t = 16)]
        length: usize,

        /// Use numeric characters only (0-9)
        #[arg(long, conflicts_with_all = ["alpha", "hex", "chars"])]
        numeric: bool,

        /// Use alphabetic characters only (a-z, A-Z)
        #[arg(long, conflicts_with_all = ["numeric", "hex", "chars"])]
        alpha: bool,

        /// Use hexadecimal characters (0-9, a-f)
        #[arg(long, conflicts_with_all = ["numeric", "alpha", "chars"])]
        hex: bool,

        /// Use a custom character set
        #[arg(long, conflicts_with_all = ["numeric", "alpha", "hex"])]
        chars: Option<String>,
    },
}
