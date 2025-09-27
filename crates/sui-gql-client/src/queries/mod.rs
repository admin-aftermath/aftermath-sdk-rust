use std::collections::HashMap;

use af_sui_types::{
    Address as SuiAddress,
    Address,
    Object,
    ObjectArg,
    ObjectRef,
    StructTag,
    TransactionData,
    Version,
};
// For `object_args!` macro only
#[doc(hidden)]
pub use bimap::BiMap;
use futures::Stream;
use outputs::{DynamicField, ObjectKey, RawMoveStruct, RawMoveValue};

use crate::{GraphQlClient, GraphQlErrors};

mod coin_metadata;
mod current_epoch_id;
mod epoch_final_checkpoint_num;
mod events_backward;
mod filtered_full_objects;
pub mod fragments;
mod full_object;
mod full_objects;
mod gas_payment;
mod genesis_tx;
mod latest_checkpoint;
mod latest_object_version;
mod latest_objects_version;
mod latest_package;
mod latest_version_at_checkpoint_v2;
mod max_page_size;
mod multi_get_objects;
mod object_arg;
mod object_args;
mod object_args_and_content;
mod object_content;
mod object_type;
mod objects_content;
mod objects_flat;
pub mod outputs;
mod owner_df_content;
mod owner_df_contents;
mod owner_df_contents_stream;
mod owner_dof_content;
mod package_at_version;
mod packages_from_original;
mod packages_published_epoch;
mod reference_gas_price;
pub(crate) mod stream;
mod transaction_blocks_status;

pub use self::events_backward::{EventEdge, EventFilter};
pub use self::gas_payment::Error as GasPaymentError;
pub use self::latest_version_at_checkpoint_v2::Error as LatestVersionAtCheckpointError;
pub use self::object_args::Error as ObjectArgsError;
pub use self::object_args_and_content::Error as ObjectArgsAndContentError;

/// Standard query result type to aid in adding new queries.
type Result<T, C> = std::result::Result<T, Error<<C as GraphQlClient>::Error>>;

/// Extension trait to [`GraphQlClient`] collecting all defined queries in one place.
#[trait_variant::make(Send)]
pub trait GraphQlClientExt: GraphQlClient + Sized {
    // NOTE: `.await` is not used in the implementations below because `trait_variant` de-sugars the
    // method definitions removing their `async` prefixes

    /// The latest epoch id.
    async fn current_epoch_id(&self) -> Result<u64, Self> {
        current_epoch_id::query(self)
    }

    /// The last checkpoint number of an epoch.
    async fn epoch_final_checkpoint_num(&self, epoch_id: u64) -> Result<u64, Self> {
        epoch_final_checkpoint_num::query(self, epoch_id)
    }

    /// Return a single page of events + cursors and a flag indicating if there's a previous page.
    ///
    /// If `page_size` is left `None`, the server decides the size of the page.
    ///
    /// The edges are returned in reverse order of which they where returned by the server
    async fn events_backward(
        &self,
        filter: Option<EventFilter>,
        cursor: Option<String>,
        page_size: Option<u32>,
    ) -> Result<(Vec<EventEdge>, bool), Self> {
        events_backward::query(self, filter, cursor, page_size)
    }

    /// The latest full [`Object`] contents with the possibility to filter by owner or object type.
    fn filtered_full_objects(
        &self,
        owner: Option<SuiAddress>,
        type_: Option<String>,
        page_size: Option<u32>,
    ) -> impl Stream<Item = Result<Object, Self>> + '_ {
        filtered_full_objects::query(self, owner, type_, page_size)
    }

    /// The full [`Object`] contents at a certain version or the latest if not specified.
    async fn full_object(&self, object_id: Address, version: Option<u64>) -> Result<Object, Self> {
        full_object::query(self, object_id, version)
    }

    /// The full [`Object`] contents at their latest versions.
    ///
    /// Fails if any requested object id is not in the final map.
    ///
    /// # Note
    ///
    /// The check for returned object ids is just so that the caller can safely do `map[object_id]`
    /// on the returned map. Keep in mind that the result if an object id is repeated in `objects`
    /// is undefined. Avoid doing so.
    async fn latest_full_objects(
        &self,
        objects: impl IntoIterator<Item = Address> + Send,
        page_size: Option<u32>,
    ) -> Result<HashMap<Address, Object>, Self> {
        full_objects::query(self, objects, page_size)
    }

    /// The full [`Object`] contents at specific versions.
    ///
    /// Duplicate object keys are automatically discarded.
    async fn multi_get_objects(
        &self,
        keys: impl IntoIterator<Item = (Address, Version)> + Send,
    ) -> Result<Vec<Object>, Self> {
        self::multi_get_objects::query(self, keys)
    }

    /// Genesis transaction of the Sui network instance.
    async fn genesis_tx(&self) -> Result<TransactionData, Self> {
        genesis_tx::query(self)
    }

    /// Latest checkpoint number.
    async fn latest_checkpoint(&self) -> Result<u64, Self> {
        latest_checkpoint::query(self)
    }

    /// The latest checkpoint number and object version of an object.
    async fn latest_object_version(&self, object_id: Address) -> Result<(u64, u64), Self> {
        latest_object_version::query(self, object_id)
    }

    /// The latest checkpoint number and the map of object ids to the their version at that
    /// checkpoint.
    ///
    /// Fails if the server doesn't return the version for any of the requested objects.
    async fn latest_objects_version(
        &self,
        object_ids: &[Address],
    ) -> Result<(u64, HashMap<Address, u64>), Self> {
        latest_objects_version::query(self, object_ids)
    }

    /// Version of the object at this checkpoint.
    async fn latest_version_at_checkpoint(
        &self,
        id: Address,
        ckpt_num: u64,
    ) -> std::result::Result<u64, LatestVersionAtCheckpointError<Self::Error>> {
        latest_version_at_checkpoint_v2::query(self, id, ckpt_num)
    }

    /// Get the object argument for a programmable transaction.
    async fn object_arg(&self, id: Address) -> Result<ObjectArg, Self> {
        object_arg::query(self, id)
    }

    /// Turn a bijective map of names and object ids into one of names and object args.
    ///
    /// Fails if the query response does not have the necessary data for the input map.
    async fn object_args(
        &self,
        names: BiMap<String, Address>,
        page_size: Option<u32>,
    ) -> std::result::Result<BiMap<String, ObjectArg>, ObjectArgsError<Self::Error>> {
        object_args::query(self, names, page_size)
    }

    /// Get a sequence of object args and contents corresponding to `object_ids`, but not
    /// necessarily in the same order.
    ///
    /// **NOTE**: prefer [`GraphQlClientExt::latest_full_objects`] instead and call `Object::object_arg`
    /// on each returned object.
    ///
    /// The `mutable` argument controls whether we want to create mutable [`ObjectArg`]s, if they
    /// are of the [`ObjectArg::SharedObject`] variant.
    ///
    /// Fails if any object in the response is missing data.
    async fn object_args_and_content(
        &self,
        object_ids: impl IntoIterator<Item = Address> + Send,
        mutable: bool,
        page_size: Option<u32>,
    ) -> std::result::Result<Vec<(ObjectArg, RawMoveStruct)>, ObjectArgsAndContentError<Self::Error>>
    {
        object_args_and_content::query(self, object_ids, mutable, page_size)
    }

    /// Get the raw Move struct of an object's content.
    async fn object_content(
        &self,
        object_id: Address,
        version: Option<u64>,
    ) -> Result<RawMoveStruct, Self> {
        object_content::query(self, object_id, version)
    }

    async fn objects_content(
        &self,
        object_ids: Vec<Address>,
    ) -> Result<HashMap<Address, RawMoveStruct>, Self> {
        objects_content::query(self, object_ids)
    }

    /// Get the raw Move value of a dynamic field's value.
    async fn owner_df_content(
        &self,
        address: SuiAddress,
        raw_move_value: RawMoveValue,
        root_version: Option<u64>,
    ) -> Result<RawMoveValue, Self> {
        owner_df_content::query(self, address, raw_move_value, root_version)
    }

    /// Map of all keys to dynamic field values: [`RawMoveValue`] -> [`DynamicField`].
    async fn owner_df_contents(
        &self,
        address: SuiAddress,
        root_version: Option<u64>,
        first: Option<i32>,
        after: Option<String>,
    ) -> Result<(HashMap<RawMoveValue, DynamicField>, Option<String>), Self> {
        owner_df_contents::query(self, address, root_version, first, after)
    }

    /// **Streamed** map of all keys to dynamic field values: [`RawMoveValue`] -> [`DynamicField`].
    async fn owner_df_contents_stream(
        &self,
        address: SuiAddress,
        root_version: Option<u64>,
        page_size: Option<i32>,
    ) -> impl Stream<Item = Result<(RawMoveValue, DynamicField), Self>> + '_ {
        owner_df_contents_stream::query(self, address, root_version, page_size)
    }

    /// Get the raw Move struct of a dynamic object field's value.
    async fn owner_dof_content(
        &self,
        address: SuiAddress,
        raw_move_value: RawMoveValue,
        root_version: Option<u64>,
    ) -> Result<(ObjectKey, RawMoveStruct), Self> {
        owner_dof_content::query(self, address, raw_move_value, root_version)
    }

    /// Get the latest version of a package.
    ///
    /// Returns both its ID and version number.
    async fn latest_package(&self, package_id: Address) -> Result<(Address, Version), Self> {
        latest_package::query(self, package_id)
    }

    /// Get the address of a package at a specific version.
    async fn package_at_version(
        &self,
        package_id: Address,
        version: Version,
    ) -> Result<Address, Self> {
        package_at_version::query(self, package_id, version)
    }

    /// Get all the package ids and versions given the original package id.
    async fn packages_from_original(
        &self,
        package_id: Address,
    ) -> Result<impl Iterator<Item = (Address, u64)>, Self> {
        packages_from_original::query(self, package_id)
    }

    /// The epoch and checkpoint number (in this order) for each package id.
    async fn packages_published_epoch(
        &self,
        package_ids: Vec<Address>,
    ) -> Result<impl Iterator<Item = (Address, u64, u64)>, Self> {
        packages_published_epoch::query(self, package_ids)
    }

    /// The reference gas price for the latest epoch.
    async fn reference_gas_price(&self) -> Result<u64, Self> {
        reference_gas_price::query(self)
    }

    /// Get execution status for the input transaction digests
    #[expect(deprecated, reason = "Internal module deprecation")]
    async fn transaction_blocks_status(
        &self,
        transaction_digests: Vec<String>,
    ) -> Result<impl Iterator<Item = crate::extract::Result<(String, bool)>>, Self> {
        transaction_blocks_status::query(self, transaction_digests)
    }

    /// Gas coins to satisfy the budget, excluding some object ids.
    ///
    /// The `exclude` is here because it can be useful if a SUI coin is already being used in the
    /// PTB itself. However in such a scenario one can use [`Argument::Gas`] instead.
    ///
    /// [`Argument::Gas`]: af_sui_types::Argument::Gas
    async fn gas_payment(
        &self,
        sponsor: SuiAddress,
        budget: u64,
        exclude: Vec<Address>,
    ) -> std::result::Result<Vec<ObjectRef>, GasPaymentError<Self::Error>> {
        gas_payment::query(self, sponsor, budget, exclude)
    }

    /// The maximum size for pagination allowed by the server.
    async fn max_page_size(&self) -> Result<i32, Self> {
        max_page_size::query(self)
    }

    /// Struct type of an object given its ID.
    async fn object_type(&self, id: Address) -> Result<StructTag, Self> {
        object_type::query(self, id)
    }

    /// Fetches metadata for the given coin type
    ///
    /// Returns a tuple containing (decimals, name, symbol)
    async fn coin_metadata(
        &self,
        type_: &str,
    ) -> Result<(Option<u8>, Option<String>, Option<String>), Self> {
        coin_metadata::query(self, type_)
    }
}

impl<T: GraphQlClient> GraphQlClientExt for T {}

/// Generic error type for queries.
#[derive(thiserror::Error, Clone, Debug)]
pub enum Error<C: std::error::Error> {
    #[error("Client error: {0:?}")]
    Client(C),
    #[error("In server response: {0}")]
    Server(#[from] GraphQlErrors),
    #[error("Missing data in response: {0}")]
    MissingData(String),
}

#[expect(deprecated, reason = "Internal module deprecation")]
impl<C: std::error::Error> From<crate::extract::Error> for Error<C> {
    fn from(value: crate::extract::Error) -> Self {
        Self::MissingData(value.0)
    }
}

impl<C: std::error::Error> From<&'static str> for Error<C> {
    fn from(value: &'static str) -> Self {
        Self::MissingData(value.into())
    }
}

/// Helper to generate [`Error::MissingData`].
///
/// Works very much like an `anyhow!`/`eyre!` macro, but intended for the case when trying to
/// extract some data from the query.
#[macro_export]
macro_rules! missing_data {
    ($($msg:tt)*) => {
        $crate::queries::Error::MissingData(format!($($msg)*))
    };
}
