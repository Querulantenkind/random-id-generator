use rand::{distributions::Alphanumeric, Rng};
use uuid::Uuid;
use nanoid::nanoid;

pub enum IdType {
    Uuid,
    NanoId,
    Custom {
        length: usize,
        charset: CustomCharset,
    },
}

#[derive(Clone, Debug)]
pub enum CustomCharset {
    Alphanumeric,
    Numeric,
    Alphabetic,
    Hex,
    Custom(String),
}

pub fn generate_id(id_type: &IdType) -> String {
    match id_type {
        IdType::Uuid => Uuid::new_v4().to_string(),
        IdType::NanoId => nanoid!(),
        IdType::Custom { length, charset } => generate_custom_id(*length, charset),
    }
}

fn generate_custom_id(length: usize, charset: &CustomCharset) -> String {
    match charset {
        CustomCharset::Alphanumeric => {
            rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(length)
                .map(char::from)
                .collect()
        }
        CustomCharset::Numeric => {
            let mut rng = rand::thread_rng();
            (0..length)
                .map(|_| rng.gen_range(0..=9).to_string())
                .collect()
        }
        CustomCharset::Alphabetic => {
            // A-Z, a-z
            const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
            let mut rng = rand::thread_rng();
            (0..length)
                .map(|_| {
                    let idx = rng.gen_range(0..CHARSET.len());
                    CHARSET[idx] as char
                })
                .collect()
        }
        CustomCharset::Hex => {
            const CHARSET: &[u8] = b"0123456789abcdef";
            let mut rng = rand::thread_rng();
            (0..length)
                .map(|_| {
                    let idx = rng.gen_range(0..CHARSET.len());
                    CHARSET[idx] as char
                })
                .collect()
        }
        CustomCharset::Custom(chars) => {
            if chars.is_empty() {
                return String::new();
            }
            let chars_vec: Vec<char> = chars.chars().collect();
            let mut rng = rand::thread_rng();
            (0..length)
                .map(|_| {
                    let idx = rng.gen_range(0..chars_vec.len());
                    chars_vec[idx]
                })
                .collect()
        }
    }
}
