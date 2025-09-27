use af_sui_types::{Address, Version};

use super::Error;
use super::fragments::PageInfoForward;
use crate::{GraphQlClient, Paged, PagedResponse, missing_data, schema};

pub async fn query<C: GraphQlClient>(
    client: &C,
    package_id: Address,
) -> Result<impl Iterator<Item = (Address, u64)> + use<C>, Error<C::Error>> {
    let vars = Variables {
        address: package_id,
        first: None,
        after: None,
    };

    let response: PagedResponse<Query> = client.query_paged(vars).await.map_err(Error::Client)?;
    let (init, pages) = response
        .try_into_data()?
        .ok_or_else(|| missing_data!("No data"))?;

    Ok(init
        .package_versions
        .nodes
        .into_iter()
        .chain(pages.into_iter().flat_map(|p| p.package_versions.nodes))
        .map(|o| (o.address, o.version)))
}

#[derive(cynic::QueryVariables, Clone, Debug)]
pub struct Variables {
    address: Address,
    after: Option<String>,
    first: Option<i32>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(variables = "Variables")]
pub struct Query {
    #[arguments(address: $address, first: $first, after: $after)]
    pub package_versions: MovePackageConnection,
}

impl Paged for Query {
    type Input = Variables;
    type NextPage = Self;
    type NextInput = Variables;

    fn next_variables(&self, mut prev_vars: Self::Input) -> Option<Self::NextInput> {
        let PageInfoForward {
            has_next_page,
            end_cursor,
        } = &self.package_versions.page_info;
        if *has_next_page {
            prev_vars.after.clone_from(end_cursor);
            Some(prev_vars)
        } else {
            None
        }
    }
}

#[derive(cynic::QueryFragment, Debug)]
pub struct MovePackageConnection {
    pub nodes: Vec<MovePackage>,
    page_info: PageInfoForward,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct MovePackage {
    pub address: Address,
    pub version: Version,
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
#[test]
fn gql_output() {
    use cynic::QueryBuilder as _;

    let vars = Variables {
        address: Address::new(rand::random()).into(),
        first: None,
        after: None,
    };
    let operation = Query::build(vars);
    insta::assert_snapshot!(operation.query, @r###"
    query Query($address: SuiAddress!, $after: String, $first: Int) {
      packageVersions(address: $address, first: $first, after: $after) {
        nodes {
          address
          version
        }
        pageInfo {
          hasNextPage
          endCursor
        }
      }
    }
    "###);
}
