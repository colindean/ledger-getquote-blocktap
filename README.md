# ledger-getquote-blocktap

![License: MIT](https://img.shields.io/github/license/colindean/ledger-getquote-blocktap.svg)
[![Linux and macOS build Status](https://travis-ci.org/colindean/ledger-getquote-blocktap.svg?branch=master)](https://travis-ci.org/colindean/ledger-getquote-blocktap)
[![Windows build status](https://ci.appveyor.com/api/projects/status/o6y1sq18anxv0oh4?svg=true)](https://ci.appveyor.com/project/colindean/ledger-getquote-blocktap)


An implementation of [ledger-cli](https://ledger-cli.org) `getquote` that uses [Blocktap.io](https://blocktap.io) GraphQL to retrieve cryptoasset pricing data.

## Usage

    getquote-blocktap <asset symbol>

Example:

    $ getquote-blocktap BTC
    3680.000000000

## Installation

Homebrew is the preferred installation method:

    brew tap colindean/ledger-getquote-blocktap https://github.com/colindean/ledger-getquote-blocktap.git
    brew install ledger-getquote-blocktap-bin

Alternatively, download the [latest release here](https://github.com/colindean/ledger-getquote-blocktap/releases/latest).

For Windows, you can use the MSVC version if you already have the 2015 Visual
C++ Runtime already installed. Otherwise, try the GNU version.

## Releasing

This repo is configured to release on annotated tags on push.

    cargo release --dry-run
    # once satisfied
    cargo release

## Communicating with Blocktap.io

### The base GraphQL

Run this in [Blocktap's playground](https://api.blocktap.io/graphiql):

```graphql
query ledger_pricedb {
  assets(sort: { marketCapRank: ASC }){ # need to programatically add filters here for the asset passed into getquote.
    assetName
    assetSymbol
    markets(aggregation: VWA, filter: { quoteSymbol_eq: "USD" }) {
      ticker {
        lastPrice
      }
    }
  }
}
```
### Retrieve Blocktap's JSON schema

Use the fantastic [graphql-rust project's client tool](https://github.com/graphql-rust/graphql-client):

    cargo install graphql_client_cli

Get the latest version of the JSON schema:

    graphql-client introspect-schema https://api.blocktap.io/graphql --output /tmp/blocktap.json
