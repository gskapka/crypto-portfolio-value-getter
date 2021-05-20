pub const USAGE_INFO: &str = "
❍ Crypto Portfolio Value Getter ❍

    Copyright Greg Kapka 2020
    Questions: greg@kapka.co.uk

❍ Info ❍

A simple CLI for getting & summing the price of various crypto assets.

❍ Usage ❍

Usage:  getprice --help
        getprice version
        getprice of (<symbol> <amount>)... <keyFilePath>

Commands:

    version               ❍ Show version info.
    of                    ❍ Gets the price of a given token.
    <symbol>              ❍ The symbol of a given token, eg ETH.
    <amount>              ❍ The amount of that given token to calculate the price of.
    <keyFilePath>         ❍ The path to your GPP-encrypted CoinMarketCap API key file.

Options:

    --help                ❍ Show this message.

";
