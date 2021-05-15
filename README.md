# :bow: Crypto Portfolio Value Getter

A verrrry simple rust CLI for getting & totalling various crypto prices. Use it to calculate the price of your portfolio (which is not so easy if you're keeping your crypto off exchanges!)

&nbsp;

## :point_right: Usage:

First clone the dir, enter the dir & build the tool:

__`❍ cargo b --release`__

Then get yourself a _coinmarketcap_ API key __[via their website here](https://pro.coinmarketcap.com/signup/)__.

Once built, enter __`./target/release`__ to find the binary.

Encrypt your _coinmarketcap_ API key __[using gpg](https://www.gnupg.org/gph/en/manual/x110.html)__:

__`❍ echo <YOUR-API-KEY-HERE> | gpg -c --output coinmarketcap-api-key.gpg`__

(The __`-c`__ here meaning symmetric encryption. Use __`-e`__ if assymmetric encryption is desired!)

And finally, use the tool to get the price of some ethereum & bitcoin (or whatever you want...) via:

__`❍ ./getprice of ETH 1.337 BTC 0.623 <PATH_TO_API_KEY>/coinmarketcap-api-key.gpg --currency=GBP`__

```
{
  "grand_total": 7242.52,
  "prices": [
    {
      "amount": 1.337,
      "asset": "ETH",
      "currency": "USD",
      "price": 366.5,
      "total": 490.01
    },
    {
      "amount": 0.623,
      "asset": "BTC",
      "currency": "USD",
      "price": 10838.7,
      "total": 6752.51
    }
  ]
}
```

For more info, call the binary via:

__`❍ ./getprice --help`__


```
❍ Crypto Portfolio Value Getter ❍

    Copyright Greg Kapka 2020
    Questions: greg@kapka.co.uk

❍ Info ❍

A simple CLI for getting & summing the price of various crypto assets.

❍ Usage ❍

Usage:  getprice --help
        getprice version
        getprice of (<symbol> <amount>)... <keyFilePath> [--currency=<SYMBOL>]

Commands:

    version               ❍ Show version info.
    of                    ❍ Gets the price of a given token.
    <symbol>              ❍ The symbol of a given token, eg ETH.
    <amount>              ❍ The amount of that given token to calculate the price of.
    <keyFilePath>         ❍ The path to your GPP-encrypted CoinMarketCap API key file.

Options:

    --help                ❍ Show this message.

    --currency=<SYMBOL>    ❍ Which currency to get prices in [default: USD]

```

&nbsp;

## :guardsman: Tests

__NOTE:__ The tests require a coinmarketcap API key set as the environment-variable __`API_KEY`__!

To run the tests simply run:

__`❍ cargo test`__

&nbsp;

## :black_nib: To Do:
- [x] Add more assets
