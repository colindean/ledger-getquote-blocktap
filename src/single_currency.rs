#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema/blocktap.json",
    query_path = "schema/single.graphql"
)]
pub struct SingleCurrency;
