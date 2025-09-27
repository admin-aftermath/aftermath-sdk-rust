use af_sui_types::{Address, Version};

use super::Error;
use crate::{GraphQlClient, GraphQlResponseExt as _, schema};

pub(super) async fn query<C: GraphQlClient>(
    client: &C,
    package_id: Address,
) -> super::Result<(Address, Version), C> {
    let vars = Variables {
        address: package_id,
    };
    let data = client
        .query::<Query, _>(vars)
        .await
        .map_err(Error::Client)?
        .try_into_data()?;
    graphql_extract::extract!(data => {
        latest_package? {
            address
            version
        }
    });
    Ok((address, version))
}

#[derive(cynic::QueryVariables, Clone, Debug)]
struct Variables {
    address: Address,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(variables = "Variables")]
struct Query {
    #[arguments(address: $address)]
    latest_package: Option<MovePackage>,
}

#[derive(cynic::QueryFragment, Debug)]
struct MovePackage {
    address: Address,
    version: Version,
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
#[test]
fn gql_output() {
    use cynic::QueryBuilder as _;

    let vars = Variables {
        address: Address::ZERO,
    };
    let operation = Query::build(vars);
    insta::assert_snapshot!(operation.query, @r###"
    query Query($address: SuiAddress!) {
      latestPackage(address: $address) {
        address
        version
      }
    }
    "###);
}
