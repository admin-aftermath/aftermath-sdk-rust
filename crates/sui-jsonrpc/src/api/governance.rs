// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use jsonrpsee::proc_macros::rpc;
use sui_sdk_types::Address;

use crate::msgs::{DelegatedStake, SuiCommittee, SuiSystemStateSummary, ValidatorApys};
use crate::serde::BigInt;

#[rpc(client, namespace = "suix")]
pub trait GovernanceReadApi {
    /// Return one or more [DelegatedStake]. If a Stake was withdrawn its status will be Unstaked.
    #[method(name = "getStakesByIds")]
    async fn get_stakes_by_ids(
        &self,
        staked_sui_ids: Vec<Address>,
    ) -> RpcResult<Vec<DelegatedStake>>;

    /// Return all [DelegatedStake].
    #[method(name = "getStakes")]
    async fn get_stakes(&self, owner: Address) -> RpcResult<Vec<DelegatedStake>>;

    /// Return the committee information for the asked `epoch`.
    #[method(name = "getCommitteeInfo")]
    async fn get_committee_info(&self, epoch: Option<BigInt<u64>>) -> RpcResult<SuiCommittee>;

    /// Return the latest SUI system state object on-chain.
    #[method(name = "getLatestSuiSystemState")]
    async fn get_latest_sui_system_state(&self) -> RpcResult<SuiSystemStateSummary>;

    /// Return the reference gas price for the network
    #[method(name = "getReferenceGasPrice")]
    async fn get_reference_gas_price(&self) -> RpcResult<BigInt<u64>>;

    /// Return the validator APY
    #[method(name = "getValidatorsApy")]
    async fn get_validators_apy(&self) -> RpcResult<ValidatorApys>;
}
