use af_move_type::MoveInstance;
use af_sui_types::Address;
use sui_gql_client::GraphQlClient;
use sui_gql_client::queries::{Error as QueryError, GraphQlClientExt as _};

use crate::oracle::PriceFeed;

type Key = crate::keys::PriceFeedForSource;

pub(crate) async fn query<C>(
    client: &C,
    af_oracle_pkg: Address,
    price_feed_storage: Address,
    source_wrapper_id: Address,
) -> Result<Option<MoveInstance<PriceFeed>>, Error<C::Error>>
where
    C: GraphQlClient,
{
    let key = Key::new(source_wrapper_id.into()).move_instance(af_oracle_pkg);
    let raw_move_value = client
        .owner_df_content(price_feed_storage, key.try_into()?, None)
        .await;
    match raw_move_value {
        Ok(raw) => Ok(Some(raw.try_into()?)),
        Err(QueryError::MissingData(_)) => Ok(None),
        Err(err) => Err(Error::OwnerDfContent(err)),
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error<C: std::error::Error> {
    #[error("Querying Owner DF content: {0}")]
    OwnerDfContent(QueryError<C>),

    #[error("BCS De/Ser: {0}")]
    Bcs(#[from] bcs::Error),

    #[error(transparent)]
    FromRawType(#[from] af_move_type::FromRawTypeError),
}
