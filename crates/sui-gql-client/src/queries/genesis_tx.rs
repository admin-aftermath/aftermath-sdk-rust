use af_sui_types::Transaction;
use graphql_extract::extract;

use super::Error;
use crate::{GraphQlClient, GraphQlResponseExt as _, scalars, schema};

pub async fn query<C: GraphQlClient>(client: &C) -> Result<Transaction, Error<C::Error>> {
    let data = client
        .query::<Query, _>(Variables { id: Some(0) })
        .await
        .map_err(Error::Client)?
        .try_into_data()?;
    extract!(data => {
        epoch? {
            transaction_blocks {
                nodes[] {
                    bcs?
                }
            }
        }
    });
    Ok(nodes
        .into_iter()
        .next()
        .ok_or("No transactions in epoch")??
        .into_inner())
}

#[derive(cynic::QueryVariables, Clone, Debug)]
struct Variables {
    id: Option<af_sui_types::Version>,
}

#[derive(cynic::QueryFragment, Clone, Debug)]
#[cynic(graphql_type = "Query", variables = "Variables")]
struct Query {
    #[arguments(id: $id)]
    epoch: Option<Epoch>,
}

#[derive(cynic::QueryFragment, Clone, Debug)]
struct Epoch {
    #[arguments(first: 1)]
    transaction_blocks: TransactionBlockConnection,
}

#[derive(cynic::QueryFragment, Clone, Debug)]
struct TransactionBlockConnection {
    nodes: Vec<TransactionBlock>,
}

#[derive(cynic::QueryFragment, Clone, Debug)]
struct TransactionBlock {
    bcs: Option<scalars::Base64Bcs<Transaction>>,
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
#[test]
fn init_gql_output() {
    use cynic::QueryBuilder as _;
    let operation = Query::build(Variables { id: None });
    insta::assert_snapshot!(operation.query, @r###"
    query Query($id: UInt53) {
      epoch(id: $id) {
        transactionBlocks(first: 1) {
          nodes {
            bcs
          }
        }
      }
    }
    "###);
}
