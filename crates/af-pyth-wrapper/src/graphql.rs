use af_move_type::MoveInstance;
use af_sui_types::Address;
use sui_framework_sdk::object::ID;
use sui_gql_client::GraphQlClient;
use sui_gql_client::queries::{Error as QueryError, GraphQlClientExt as _};

type Key = crate::wrapper::PythPriceInfoId;

/// Error for [`GraphQlClientExt`].
#[derive(thiserror::Error, Debug)]
pub enum Error<C: std::error::Error> {
    #[error("Querying Owner DF content: {0}")]
    OwnerDfContent(QueryError<C>),

    #[error("BCS De/Ser: {0}")]
    Bcs(#[from] sui_sdk_types::bcs::Error),

    #[error(transparent)]
    FromRawType(#[from] af_move_type::FromRawTypeError),
}

#[trait_variant::make(Send)]
pub trait GraphQlClientExt: GraphQlClient + Sized {
    /// Get the ID of the Pyth `PriceInfoObject` for a `PriceFeed`.
    async fn price_info_object_id_for_feed(
        &self,
        pyth_wrapper_pkg: Address,
        price_feed: Address,
    ) -> Result<Address, Error<Self::Error>> {
        async move {
            let key = Key::new().move_instance(pyth_wrapper_pkg);
            let raw = self
                .owner_df_content(price_feed, key.try_into()?, None)
                .await
                .map_err(Error::OwnerDfContent)?;
            let pf: MoveInstance<ID> = raw.try_into()?;

            Ok(pf.value.bytes)
        }
    }
}

impl<T: GraphQlClient> GraphQlClientExt for T {}
