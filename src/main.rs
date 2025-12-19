mod cli;
mod generator;
mod interactive;

use clap::Parser;
use cli::{Cli, Commands};
use generator::{generate_id, CustomCharset, IdType};
use arboard::Clipboard;

fn main() {
    let args = Cli::parse();

    // Determine if we should run in interactive mode
    // If no subcommand is provided, we assume interactive mode
    // However, clap might handle "no subcommand" differently depending on configuration.
    // But here `command` is Option<Commands>.
    
    let (id_type, count, copy) = if let Some(command) = args.command {
        // CLI mode
        let id_type = match command {
            Commands::Uuid => IdType::Uuid,
            Commands::Nanoid => IdType::NanoId,
            Commands::Custom { length, numeric, alpha, hex, chars } => {
                let charset = if numeric {
                    CustomCharset::Numeric
                } else if alpha {
                    CustomCharset::Alphabetic
                } else if hex {
                    CustomCharset::Hex
                } else if let Some(c) = chars {
                    CustomCharset::Custom(c)
                } else {
                    CustomCharset::Alphanumeric
                };
                IdType::Custom { length, charset }
            }
        };
        (id_type, args.count, args.copy)
    } else {
        // Interactive mode
        // If the user just ran `idgen` without args
        match interactive::run_wizard(args.count, args.copy) {
            Some(opts) => (opts.id_type, opts.count, opts.copy),
            None => return, // User cancelled or error
        }
    };

    let mut outputs = Vec::new();
    for _ in 0..count {
        let id = generate_id(&id_type);
        println!("{}", id);
        outputs.push(id);
    }

    if copy {
        let text_to_copy = outputs.join("\n");
        match Clipboard::new() {
            Ok(mut clipboard) => {
                if let Err(e) = clipboard.set_text(text_to_copy) {
                    eprintln!("Warning: Failed to copy to clipboard: {}", e);
                } else {
                    eprintln!("(Copied to clipboard)");
                }
            }
            Err(e) => {
                eprintln!("Warning: Failed to access clipboard: {}", e);
            }
        }
    }
}
