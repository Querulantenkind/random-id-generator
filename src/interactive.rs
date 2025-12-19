use dialoguer::{theme::ColorfulTheme, Input, Select, Confirm};
use crate::generator::{IdType, CustomCharset};

pub struct InteractiveOptions {
    pub id_type: IdType,
    pub count: usize,
    pub copy: bool,
}

pub fn run_wizard(default_count: usize, default_copy: bool) -> Option<InteractiveOptions> {
    let theme = ColorfulTheme::default();

    let types = vec!["UUID v4", "NanoID", "Custom"];
    let selection = Select::with_theme(&theme)
        .with_prompt("What type of ID do you want to generate?")
        .default(0)
        .items(&types)
        .interact()
        .ok()?;

    let id_type = match selection {
        0 => IdType::Uuid,
        1 => IdType::NanoId,
        2 => {
            let length: usize = Input::with_theme(&theme)
                .with_prompt("How long should the ID be?")
                .default(16)
                .interact_text()
                .ok()?;

            let charsets = vec!["Alphanumeric (Default)", "Numeric (0-9)", "Alphabetic (A-Z, a-z)", "Hex (0-9, a-f)", "Custom String"];
            let charset_sel = Select::with_theme(&theme)
                .with_prompt("Which character set?")
                .default(0)
                .items(&charsets)
                .interact()
                .ok()?;

            let charset = match charset_sel {
                0 => CustomCharset::Alphanumeric,
                1 => CustomCharset::Numeric,
                2 => CustomCharset::Alphabetic,
                3 => CustomCharset::Hex,
                4 => {
                    let custom_str: String = Input::with_theme(&theme)
                        .with_prompt("Enter allowed characters")
                        .interact_text()
                        .ok()?;
                    CustomCharset::Custom(custom_str)
                }
                _ => CustomCharset::Alphanumeric,
            };

            IdType::Custom { length, charset }
        }
        _ => return None,
    };

    let count: usize = Input::with_theme(&theme)
        .with_prompt("How many IDs to generate?")
        .default(default_count)
        .interact_text()
        .ok()?;

    let copy = Confirm::with_theme(&theme)
        .with_prompt("Copy to clipboard?")
        .default(default_copy)
        .interact()
        .unwrap_or(false);

    Some(InteractiveOptions {
        id_type,
        count,
        copy,
    })
}
