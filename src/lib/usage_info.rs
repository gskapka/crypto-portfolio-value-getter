pub const USAGE_INFO: &str = "
❍ Crypto Price Getter ❍

    Copyright Greg Kapka 2020
    Questions: greg@kapka.co.uk

❍ Info ❍

A simple CLI for getting the price of various crypto assets.

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
";
/*
pub const USAGE_INFO: &str = "
❍ Bitcoff ❍

    Copyright Greg Kapka 2019
    Questions: greg@kapka.co.uk

❍ Info ❍

An on or offline BTC transaction signer!

❍ Usage ❍

Usage:  bitcoff --help
        bitcoff version
        bitcoff createKey [--network=<string>] [--outputPath=<path>] [--logLevel=<level>]
        bitcoff getUtxos [--keyfile=<path>] [--network=<string>] [--outputPath=<path>] [--logLevel=<level>]
        bitcoff getUtxosForAddress <btcAddress> [--network=<string>] [--outputPath=<path>] [--logLevel=<level>]
        bitcoff makeOnlineSweepTx <sweepTo> [--keyfile=<path>] [--network=<string>] [--fee=<uint>] [--outputPath=<path>] [--logLevel=<level>]
        bitcoff makeOfflineSweepTx <sweepTo> (--utxoFile=<path> | <utxos>) [--keyfile=<path>] [--network=<string>] [--fee=<uint>] [--outputPath=<path>] [--logLevel=<level>]
        bitcoff makeOnlineTx (<to> <amount>)... [--keyfile=<path>] [--network=<string>] [--fee=<uint>] [--change=<string>] [--outputPath=<path>] [--logLevel=<level>]
        bitcoff makeOfflineTx (<to> <amount>)... (--utxoFile=<path> | <utxos>) [--keyfile=<path>] [--network=<string>] [--fee=<uint>] [--change=<string>] [--outputPath=<path>] [--logLevel=<level>]
        bitcoff makeOnlineOpReturnTx (<to> <amount>)... <data> [--keyfile=<path>] [--network=<string>] [--fee=<uint>] [--change=<string>] [--outputPath=<path>] [--logLevel=<level>]
        bitcoff makeOfflineOpReturnTx (<to> <amount>)... <data> (--utxoFile=<path> | <utxos>) [--keyfile=<path>] [--network=<string>] [--fee=<uint>] [--change=<string>] [--outputPath=<path>] [--logLevel=<level>]

Commands:

    version               ❍ Show version info.
    createKey             ❍ Creates a BTC private key and public address combo.
    getUtxos              ❍ Makes API call to get all UTXOs associated with address derived from the encrypted private
                            key. UTXOs are presented in the following JSON format:
                            [{ serialized_utxo: <0x...>, value: <value-in-Satoshis> },...]
    getUtxosForAddress    ❍ Makes API call to get all UTXOs associated with supplied BTC address UTXOs are presented in
                            the following JSON format:
                            [{ serialized_utxo: <0x...>, value: <value-in-Satoshis> },...]
    makeOnlineTx          ❍ Create a simple BTC p2pkh transaction to one or more addresses. This online version will
                            grab the UTXO set for the private key you provide via an API call.
    makeOfflineTx         ❍ Create a simple BTC transaction to one or more addresses. In this offline version, the
                            UTXOs must be passed in via as either a JSON string, or from a file, both of which must use
                            the JSON format:
                            [{ serialized_utxo: <0x...>, value: <value-in-Satoshis> },...]
    makeOnlineOpReturnTx  ❍ Create an `OP_RETURN` transaction, pay the `to` address via a `p2pkh` transaction and where
                            the `OP_RETURN` output contains the <data> supplied. In this online version, available UTXOs
                            for the address of the private-key supplied are pulled from a block explorer.
    makeOfflineOpReturnTx ❍ Create an `OP_RETURN` transaction, pay the `to` address via a `p2pkh` transaction and where
                            the `OP_RETURN` output contains the <data> supplied. In this offline version, the UTXOs
                            required must be passed in via as either a JSON string, or from a file, both of which must
                            use the same JSON format as the above `getUtxos` command returns:
                            [{ serialized_utxo: <0x...>, value: <value-in-Satoshis> },...]
    <to>                  ❍ Address to send the transaction to.
    <sweepTo>             ❍ Address to send sweep the entire UTXO balance to.
    <amount>              ❍ Amount to send (in Satoshis).
    <data>                ❍ The hex data for the `OP_RETURN` output.
    <btcAddress>          ❍ A bitcoin address.
    <utxos>               ❍ The UTXOs required for a BTC transaction, as a
                            valid JSON string in the form:
                            [{ serialized_utxo: <0x...>, value: <value-in-Satoshis> },...]

Options:

    --help                ❍ Show this message.
    --outputPath=<path>   ❍ Save the tool's output to given path.
    --fee=<uint>          ❍ Fee to pay in Satoshis-per-byte. [default: 23]
    --network=<string>    ❍ Btc network: Either `Bitcoin` or `Testnet`. [default: Bitcoin]
    --logLevel=<level>    ❍ Define the level of logging in the tool's output as one of: `none`, `info`, `debug`, `trace`
                            or `error` [default: none]
    --keyfile=<path>      ❍ Path to GPG-encrypted BTC private key in wallet import format (`WIF`).
                            [default: ./encrypted-btc-private-key.gpg]
    --nonce=<uint>        ❍ A nonce to be combined with the ETH address before hashing. A nonce of '0' will use a unix
                            timestamp instead. [default: 0]
    --change=<string>     ❍ Address to send any change to. Defaults to address of the private key used for the
                            transaction. [default: signer]
    --utxoFile=<path>     ❍ Path to a file containing a valid JSON array of BTC UTXOs in the format:
                            [{ serialized_utxo: <0x...>, value: <value-in-Satoshis> },...]
";
*/
