# ledger-getquote-blocktap

An implementation of [ledger-cli](https://ledger-cli.org) `getquote` that uses [Blocktap.io](https://blocktap.io) GraphQL to retrieve cryptocurrency pricing data.

## The base GraphQL

Run this in [Blocktap's playground](https://api.blocktap.io/graphiql):

```graphql
query ledger_pricedb {
  currencies(sort: { marketCapRank: ASC }){ # need to programatically add filters here for the currency passed into getquote.
    currencyName
    currencySymbol
    markets(aggregation: VWA, filter: { quoteSymbol_eq: "USD" }) {
      ticker {
        lastPrice
      }
    }
  }
}
```
