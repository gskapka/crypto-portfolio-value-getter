mod lib;
#[macro_use]
extern crate quick_error;
#[macro_use]
extern crate serde_derive;

use crate::lib::{
    get_cli_args::{get_cli_args, CliArgs},
    get_price::get_price_of,
    get_version_info::get_version_info,
    types::Result,
    usage_info::USAGE_INFO,
};

fn main() -> Result<()> {
    match get_cli_args().and_then(|cli_args| match cli_args {
        CliArgs { cmd_version: true, .. } => get_version_info(),
        CliArgs { cmd_of: true, .. } => get_price_of(&cli_args.arg_symbol, &cli_args.arg_amount),
        _ => Err(USAGE_INFO.into()),
    }) {
        Ok(result) => {
            println!("{}", result);
            Ok(())
        }
        Err(err) => {
            println!("{}", err);
            std::process::exit(1);
        }
    }
}
