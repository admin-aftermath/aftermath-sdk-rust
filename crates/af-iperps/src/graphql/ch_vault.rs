use af_move_type::{FromRawStructError, MoveInstance};
use af_sui_types::Address;
use enum_as_inner::EnumAsInner;
use sui_gql_client::queries::Error as QueryError;
use sui_gql_client::queries::fragments::{DynamicFieldName, MoveValueRaw};
use sui_gql_client::queries::outputs::RawMoveStruct;
use sui_gql_client::{GraphQlClient, GraphQlResponseExt, schema};

use crate::keys;

#[derive(thiserror::Error, Debug)]
pub enum Error<C: std::error::Error> {
    #[error(transparent)]
    Query(#[from] QueryError<C>),
    #[error("Deserializing Vault instance: {0}")]
    ToVault(#[from] FromRawStructError),
}

type Vault = MoveInstance<crate::Vault>;

pub(super) async fn query<C: GraphQlClient>(
    client: &C,
    package: Address,
    ch: Address,
) -> Result<Vault, Error<C::Error>> {
    let raw = request(client, package, ch).await?;
    Ok(raw.try_into()?)
}

async fn request<C: GraphQlClient>(
    client: &C,
    package: Address,
    ch: Address,
) -> Result<RawMoveStruct, QueryError<C::Error>> {
    let vars = Variables {
        ch,
        vault: keys::MarketVault::new()
            .move_instance(package)
            .try_into()
            .expect("BCS-serializable"),
    };
    let data = client
        .query::<Query, _>(vars)
        .await
        .map_err(QueryError::Client)?
        .try_into_data()?;
    Ok(extract(data)?)
}

fn extract(data: Option<Query>) -> Result<RawMoveStruct, &'static str> {
    graphql_extract::extract!(data => {
        ch? {
            vault? {
                value? {
                    ... on VaultDfValue::MoveValue {
                        type_
                        bcs
                    }
                }
            }
        }
    });
    let move_value = MoveValueRaw { type_, bcs };
    Ok(move_value.try_into().expect("Vault is a struct"))
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
#[test]
fn gql_output() {
    use cynic::QueryBuilder as _;

    let package = Address::ZERO;
    let vars = Variables {
        ch: Address::ZERO,
        vault: keys::MarketVault::new()
            .move_instance(package)
            .try_into()
            .unwrap(),
    };
    let operation = Query::build(vars);
    insta::assert_snapshot!(operation.query, @r###"
    query Query($ch: SuiAddress!, $vault: DynamicFieldName!) {
      ch: object(address: $ch) {
        vault: dynamicField(name: $vault) {
          value {
            __typename
            ... on MoveValue {
              type {
                repr
              }
              bcs
            }
          }
        }
      }
    }
    "###);
}

#[derive(cynic::QueryVariables, Debug)]
struct Variables {
    ch: Address,
    vault: DynamicFieldName,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "Variables")]
struct Query {
    #[arguments(address: $ch)]
    #[cynic(alias, rename = "object")]
    ch: Option<ClearingHouseObject>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Object", variables = "Variables")]
struct ClearingHouseObject {
    #[arguments(name: $vault)]
    #[cynic(alias, rename = "dynamicField")]
    vault: Option<VaultDf>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "DynamicField")]
struct VaultDf {
    value: Option<VaultDfValue>,
}

#[derive(cynic::InlineFragments, Debug, EnumAsInner)]
#[cynic(graphql_type = "DynamicFieldValue")]
enum VaultDfValue {
    MoveValue(MoveValueRaw),
    #[cynic(fallback)]
    Unknown,
}
