extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate graphql_client;

extern crate failure;
extern crate reqwest;

use graphql_client::{GraphQLQuery, Response};

extern crate argparse;
use argparse::{ArgumentParser, Store};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema/blocktap.json",
    query_path = "schema/single.graphql",
    response_derives = "Debug"
)]
pub struct SingleAsset;

const BLOCKTAP_GRAPHQL_ENDPOINT: &str = "https://api.blocktap.io/graphql";

fn main() {
    let mut symbol = String::from("");
    {
        let mut parser = ArgumentParser::new();
        parser.set_description("Retrieves last price for a asset from Blocktap.io");
        parser
            .refer(&mut symbol)
            .add_argument("symbol", Store, "Symbol of the asset");
        parser.parse_args_or_exit();
    }
    eprintln!("Accessing {}…", BLOCKTAP_GRAPHQL_ENDPOINT);
    let response_body =
        do_query(single_asset::Variables { symbol: symbol }).expect("Unable to query");
    let price: String = extract_price(response_body);
    println!("{}", price)
}

use single_asset::SingleAssetMarkets;
fn extract_price(response: Response<single_asset::ResponseData>) -> String {
    let markets: Vec<Option<SingleAssetMarkets>> = response
        .data
        .expect("No data")
        .asset
        .expect("No asset")
        .markets
        .expect("No markets");
    let first: &Option<SingleAssetMarkets> = markets.first().expect("Markets is empty");
    let markets: &SingleAssetMarkets = first.as_ref().expect("sdf");

    let ticker = markets.ticker.as_ref().expect("No ticker");
    let price = ticker.last_price.as_ref().expect("No last_price");

    price.to_string()
}

fn do_query(
    variables: single_asset::Variables,
) -> Result<Response<single_asset::ResponseData>, failure::Error> {
    let request_body = SingleAsset::build_query(variables);
    let client = reqwest::Client::new();
    let mut resp = client
        .post(BLOCKTAP_GRAPHQL_ENDPOINT)
        .json(&request_body)
        .send()?;
    let response_body: Response<single_asset::ResponseData> = resp.json()?;
    Ok(response_body)
}
