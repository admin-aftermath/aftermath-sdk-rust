// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use jsonrpsee::proc_macros::rpc;
use sui_sdk_types::Address;

use crate::msgs::{Balance, CoinPage, SuiCoinMetadata, Supply};

#[rpc(client, namespace = "suix")]
pub trait CoinReadApi {
    /// Return all Coin<`coin_type`> objects owned by an address.
    #[method(name = "getCoins")]
    async fn get_coins(
        &self,
        owner: Address,
        coin_type: Option<String>,
        cursor: Option<String>,
        limit: Option<usize>,
    ) -> RpcResult<CoinPage>;

    /// Return all Coin objects owned by an address.
    #[method(name = "getAllCoins")]
    async fn get_all_coins(
        &self,
        owner: Address,
        cursor: Option<String>,
        limit: Option<usize>,
    ) -> RpcResult<CoinPage>;

    /// Return the total coin balance for one coin type, owned by the address owner.
    #[method(name = "getBalance")]
    async fn get_balance(&self, owner: Address, coin_type: Option<String>) -> RpcResult<Balance>;

    /// Return the total coin balance for all coin type, owned by the address owner.
    #[method(name = "getAllBalances")]
    async fn get_all_balances(&self, owner: Address) -> RpcResult<Vec<Balance>>;

    /// Return metadata (e.g., symbol, decimals) for a coin.
    ///
    /// Note that if the coin's metadata was
    /// wrapped in the transaction that published its marker type, or the latest version of the
    /// metadata object is wrapped or deleted, it will not be found.
    #[method(name = "getCoinMetadata")]
    async fn get_coin_metadata(&self, coin_type: String) -> RpcResult<Option<SuiCoinMetadata>>;

    /// Return total supply for a coin
    #[method(name = "getTotalSupply")]
    async fn get_total_supply(&self, coin_type: String) -> RpcResult<Supply>;
}
