use std::{path::Path, process::Command, str::from_utf8};

use crate::lib::types::{Byte, Result};

fn strip_new_lines_from_str(string: String) -> String {
    string.replace("\n", "")
}

fn convert_bytes_to_string(bytes: &[Byte]) -> Result<String> {
    Ok(from_utf8(bytes)?.to_string())
}

fn convert_bytes_to_string_with_no_new_lines(bytes: &[Byte]) -> Result<String> {
    convert_bytes_to_string(bytes).map(strip_new_lines_from_str)
}

fn check_keyfile_exists(path: &str) -> Result<()> {
    match Path::new(path).is_file() {
        false => Err(format!("✘ API keyfile does not exist @ path: {}", path).into()),
        true => {
            info!("✔ Path exists");
            Ok(())
        },
    }
}

pub fn maybe_decrypt_api_keyfile(path: &str) -> Result<String> {
    info!("✔ Decrypting API key @ path: {}", path);
    check_keyfile_exists(path).and_then(|_| {
        let output = Command::new("gpg").arg("-d").arg(path).output()?;
        match output.stdout.len() {
            0 => {
                info!("✘ Error decrypting keyfile!");
                Err(convert_bytes_to_string_with_no_new_lines(&output.stderr)?.into())
            },
            _ => {
                info!("✔ Keyfile decrypted!");
                convert_bytes_to_string_with_no_new_lines(&output.stdout)
            },
        }
    })
}
