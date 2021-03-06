use docopt::Docopt;

use crate::lib::{types::Result, usage_info::USAGE_INFO};

#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct CliArgs {
    pub cmd_of: bool,
    pub cmd_version: bool,
    pub arg_amount: Vec<f64>,
    pub arg_symbol: Vec<String>,
    pub arg_keyFilePath: String,
}

pub fn get_cli_args() -> Result<CliArgs> {
    match Docopt::new(USAGE_INFO).and_then(|d| d.deserialize()) {
        Ok(cli_args) => Ok(cli_args),
        Err(e) => Err(e.into()),
    }
}
