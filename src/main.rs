extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate graphql_client;

extern crate failure;
extern crate reqwest;

use graphql_client::{GraphQLQuery, Response};

extern crate argparse;
use argparse::{ArgumentParser, Store, StoreTrue};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema/blocktap.json",
    query_path = "schema/single.graphql",
    response_derives = "Debug"
)]
pub struct SingleCurrency;

const BLOCKTAP_GRAPHQL_ENDPOINT: &str = "https://api.blocktap.io/graphql";

fn main() {
    let mut symbol = String::from("");
    {
        let mut parser = ArgumentParser::new();
        parser.set_description("Retrieves last price for a currency from Blocktap.io");
        parser
            .refer(&mut symbol)
            .add_argument("symbol", Store, "Symbol of the currency");
        parser.parse_args_or_exit();
    }
    eprintln!("Accessing {}…", BLOCKTAP_GRAPHQL_ENDPOINT);
    let response_body =
        do_query(single_currency::Variables { symbol: symbol }).expect("Unable to query");
    let price: String = extract_price(response_body);
    println!("{}", price)
}

use single_currency::SingleCurrencyMarkets;
fn extract_price(response: Response<single_currency::ResponseData>) -> String {
    let markets: Vec<Option<SingleCurrencyMarkets>> = response
        .data
        .expect("No data")
        .currency
        .expect("No currency")
        .markets
        .expect("No markets");
    let first: &Option<SingleCurrencyMarkets> = markets.first().expect("Markets is empty");
    let markets: &SingleCurrencyMarkets = first.as_ref().expect("sdf");

    let ticker = markets.ticker.as_ref().expect("No ticker");
    let price = ticker.last_price.as_ref().expect("No last_price");

    price.to_string()
}

fn do_query(
    variables: single_currency::Variables,
) -> Result<Response<single_currency::ResponseData>, failure::Error> {
    let request_body = SingleCurrency::build_query(variables);
    let client = reqwest::Client::new();
    let mut resp = client
        .post(BLOCKTAP_GRAPHQL_ENDPOINT)
        .json(&request_body)
        .send()?;
    let response_body: Response<single_currency::ResponseData> = resp.json()?;
    Ok(response_body)
}
