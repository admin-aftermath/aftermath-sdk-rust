//! JSON-RPC methods for querying Pyth on-chain data.
use af_ptbuilder::ptb;
use af_sui_types::{Address as SuiAddress, ObjectArg, TransactionKind};
use sui_framework_sdk::object::ID;
use sui_jsonrpc::api::WriteApiClient;
use sui_sdk_types::bcs::{FromBcs, ToBcs};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    JsonRpcClient(#[from] sui_jsonrpc::error::JsonRpcClientError),
    #[error("In ProgrammableTransactionBuilder: {0}")]
    PtBuilder(#[from] af_ptbuilder::Error),
    #[error(transparent)]
    FromHex(#[from] hex::FromHexError),
    #[error("Serializing to BCS: {0}")]
    Bcs(#[from] sui_sdk_types::bcs::Error),
    #[error("DevInspectResults.results is None")]
    DevInspectResults,
}

/// Performs a dev-inspect with a client implementation to return the object ID for an off-chain
/// price identifier.
///
/// Price identifiers can be found in <https://www.pyth.network/developers/price-feed-ids>
pub async fn get_price_info_object_id_from_pyth_state<C>(
    client: &C,
    package: SuiAddress,
    price_identifier_hex: String,
    pyth_state: ObjectArg,
) -> Result<SuiAddress>
where
    C: WriteApiClient + Sync,
{
    let price_identifier_bytes = &hex::decode(price_identifier_hex.replace("0x", ""))?;

    let inspect_tx = ptb!(
        package pyth: package;

        input obj pyth_state;
        input pure price_identifier_bytes;

        pyth::state::get_price_info_object_id(pyth_state, price_identifier_bytes);
    );

    let mut results = {
        let tx_bytes = TransactionKind::ProgrammableTransaction(inspect_tx).to_bcs_base64()?;
        let resp = client
            .dev_inspect_transaction_block(
                SuiAddress::ZERO, // doesn't matter
                tx_bytes,
                None,
                None,
                None,
            )
            .await?;
        resp.results.ok_or(Error::DevInspectResults)?
    };
    let sui_exec_result = results.swap_remove(0);
    let mut return_values = sui_exec_result.return_values;
    let (bytes, _sui_type_tag) = return_values.swap_remove(0);
    let id = ID::from_bcs(&bytes)?;
    Ok(id.bytes)
}
