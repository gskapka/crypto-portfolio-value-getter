# :bow: Crypto Portfolio Value Getter

A verrrry simple rust CLI for getting & totalling various crypto prices. Use it to calculate the price of your portfolio (which is not so easy if you're keeping your crypto off exchanges!)

&nbsp;

***

&nbsp;

### :point_right: Usage:

First clone the dir, enter the dir & build the tool:

__`❍ cargo b --release`__

Then call the binary via to see the usage info:

__`❍ ./target/release/getprice of ethereum 1.337 bitcoin 0.623`__

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

__`❍ ./target/release/getprice --help`__

```
❍ Crypto Portfolio Value Getter ❍

    Copyright Greg Kapka 2020
    Questions: greg@kapka.co.uk

❍ Info ❍

A simple CLI for getting & summing the price of various crypto assets.

❍ Usage ❍

Usage:  getprice --help
        getprice version
        getprice of (<symbol> <amount>)...

Commands:

    version               ❍ Show version info.
    of                    ❍ Gets the price of a given token.
    <symbol>              ❍ The symbol of a given token, eg ETH.
    <amount>              ❍ The amount of that given token to calculate the price of.

Options:

    --help                ❍ Show this message.
```

***

&nbsp;

### :page_with_curl: Notes:

It's currently only supporting very few crypto assets - mainly those that I give a hoot about.

&nbsp;

***

&nbsp;

### :guardsman: Tests

To run the tests simply run:

__`❍ cargo test`__

&nbsp;

***

&nbsp;

### :black_nib: To Do:
- [ ] Add more assets
