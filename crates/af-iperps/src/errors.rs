// Copyright (c) Aftermath Technologies, Inc.
// SPDX-License-Identifier: Apache-2.0

#![expect(non_upper_case_globals, reason = "Copied from Move")]

macro_rules! move_aborts {
    (module $_:ident::$module:ident {$(
        $(#[$meta:meta])*
        const $Error:ident: u64 = $num:literal;
    )*}) => {
        $(
            $(#[$meta])*
            pub const $Error: u64 = $num;
        )*
        #[derive(
            Debug,
            PartialEq,
            Eq,
            Hash,
            num_enum::IntoPrimitive,
            num_enum::TryFromPrimitive,
            strum::Display,
            strum::EnumIs,
            strum::EnumMessage,
            strum::IntoStaticStr,
        )]
        #[repr(u64)]
        pub enum MoveAbort {$(
            $(#[$meta])*
            $Error = $num,
        )*}
    };
}

move_aborts! {
module perpetuals::errors {
    // ClearingHouse ---------------------------------------------------------------

    /// Cannot deposit/withdraw zero coins to/from the account's collateral.
    const DepositOrWithdrawAmountZero: u64 = 0;
    /// Size to place is 0. Raised also when there is no open position and
    /// an order with `reduce_only` is passed.
    const SizeOrPositionZero: u64 = 1;
    /// Index price returned from oracle is 0 or invalid value
    const BadIndexPrice: u64 = 2;
    /// Price is either 0 or greater than 0x8000_0000_0000_0000
    const InvalidPrice: u64 = 3;
    /// Order value in USD is too low
    const OrderUsdValueTooLow: u64 = 4;
    /// Passed a vector of invalid order ids to perform force cancellation
    /// during liquidation
    const InvalidForceCancelIds: u64 = 5;
    /// Liquidate must be the first operation of the session, if performed.
    const LiquidateNotFirstOperation: u64 = 6;
    /// Passed a vector of invalid order ids to cancel
    const InvalidCancelOrderIds: u64 = 7;
    /// Ticket has already passed `expire_timestamp` and can only be cancelled
    const StopOrderTicketExpired: u64 = 8;
    /// Index price is not at correct value to satisfy stop order conditions
    const StopOrderConditionsViolated: u64 = 9;
    /// Index price is not at correct value to satisfy stop order conditions
    const WrongOrderDetails: u64 = 10;
    /// Invalid base price feed storage for the clearing house
    const InvalidBasePriceFeedStorage: u64 = 11;
    /// Same liquidator and liqee account ids
    const SelfLiquidation: u64 = 12;
    /// User trying to access the account used the wrong account cap
    const InvalidAccountCap: u64 = 13;
    /// Raised when passing an integrator taker fee that is greater than
    /// the `max_integrator_taker_fee` set by the user in its account
    const InvalidIntegratorTakerFee: u64 = 14;
    /// Raised when trying to call a function with the wrong package's version
    const WrongVersion: u64 = 16;
    /// Raised when trying to have a session composed by only `start_session` and `end_session`
    const EmptySession: u64 = 17;
    /// Market already registered in the registry
    const MarketAlreadyRegistered: u64 = 18;
    /// Collateral is not registered in the registry
    const CollateralIsNotRegistered: u64 = 19;
    /// Market is not registered in the registry
    const MarketIsNotRegistered: u64 = 20;
    /// Invalid collateral price feed storage for the clearing house
    const InvalidCollateralPriceFeedStorage: u64 = 21;
    /// Fees accrued are negative
    const NegativeFeesAccrued: u64 = 22;
    /// Passed a timestamp older than current Clock's one
    const InvalidExpirationTimestamp: u64 = 23;
    /// Stop order gas cost provided is not enough
    const NotEnoughGasForStopOrder: u64 = 24;
    /// Invalid account trying to perform an action on a StopOrderTicket
    const InvalidAccountForStopOrder: u64 = 26;
    /// Invalid executor trying to execute the StopOrderTicket
    const InvalidExecutorForStopOrder: u64 = 27;
    /// Raised when the market's max open interest is surpassed as a result of
    /// the session's actions
    const MaxOpenInterestSurpassed: u64 = 28;
    /// Raised when a position's would get a base amount higher than the
    /// allowed percentage of open interest
    const MaxOpenInterestPositionPercentSurpassed: u64 = 29;
    /// Raised processing a session that requires a collateral allocation,
    /// but not enough collateral is available in the account
    const NotEnoughCollateralToAllocateForSession: u64 = 30;
    /// Raised processing a session that requires a collateral allocation
    /// and a wrong account is being used to fund it
    const WrongAccountIdForAllocation: u64 = 31;
    /// Raised when trying to create an integrator vault for an address
    /// that already has one
    const IntegratorVaultAlreadyExists: u64 = 32;
    /// Raised when trying to access an integrator vault that does not exist
    const IntegratorVaultDoesNotExist: u64 = 33;
    /// Raised when trying to place an order passing a `size` that is not a multiple
    /// of market's lot size
    const SizeNotMultipleOfLotSize: u64 = 34;
    /// Raised when trying to place an order passing a `price` that is not a multiple
    /// of market's tick size
    const PriceNotMultipleOfTickSize: u64 = 35;

    // Market ---------------------------------------------------------------

    /// While creating ordered map with invalid parameters,
    /// or changing them improperly for an existent map.
    const InvalidMarketParameters: u64 = 1000;
    /// Tried to call `update_funding` before enough time has passed since the
    /// last update.
    const UpdatingFundingTooEarly: u64 = 1001;
    /// Margin ratio update proposal already exists for market
    const ProposalAlreadyExists: u64 = 1002;
    /// Margin ratio update proposal cannot be commited too early
    const PrematureProposal: u64 = 1003;
    /// Margin ratio update proposal delay is outside the valid range
    const InvalidProposalDelay: u64 = 1004;
    /// Margin ratio update proposal does not exist for market
    const ProposalDoesNotExist: u64 = 1005;
    /// Exchange has no available fees to withdraw
    const NoFeesAccrued: u64 = 1006;
    /// Tried to withdraw more insurance funds than the allowed amount
    const InsufficientInsuranceSurplus: u64 = 1007;
    /// Cannot create a market for which a price feed does not exist
    const NoPriceFeedForMarket: u64 = 1008;
    /// Cannot delete a proposal that already matured. It can only be committed.
    const ProposalAlreadyMatured: u64 = 1009;
    /// Raised when an operation is performed while the market is paused
    const MarketIsPaused: u64 = 1010;
    /// Raised when trying to call `close_position_at_settlement_prices` while
    /// the market is not paused
    const MarketIsNotPaused: u64 = 1011;
    /// Raised when trying to call `close_position_at_settlement_prices` while
    /// before `close_market` has been called by admin
    const MarketIsNotClosed: u64 = 1012;
    /// Raised when Admin tries to resume a closed market
    const MarketIsClosed: u64 = 1013;

    // Position  ---------------------------------------------------------------

    /// Tried placing a new pending order when the position already has the maximum
    /// allowed number of pending orders.
    const MaxPendingOrdersExceeded: u64 = 2000;
    /// Used for checking both liqee and liqor positions during liquidation
    const PositionBelowIMR: u64 = 2001;
    /// When leaving liqee's position with a margin ratio above tolerance,
    /// meaning that liqor has overbought position
    const PositionAboveTolerance: u64 = 2002;
    /// An operation brought an account below initial margin requirements.
    const InitialMarginRequirementViolated: u64 = 2003;
    /// Position is above MMR, so can't be liquidated.
    const PositionAboveMMR: u64 = 2004;
    /// Cannot realize bad debt via means other than calling 'liquidate'.
    const PositionBadDebt: u64 = 2005;
    /// Cannot withdraw more than the account's free collateral.
    const InsufficientFreeCollateral: u64 = 2006;
    /// Cannot have more than 1 position in a market.
    const PositionAlreadyExists: u64 = 2007;
    /// Cannot compute deallocate amount for a target MR < IMR.
    const DeallocateTargetMrTooLow: u64 = 2008;
    /// Raised when trying to set a position's IMR lower than market's IMR or higher than 1
    const InvalidPositionIMR: u64 = 2009;
    /// Invalid stop order type
    const InvalidStopOrderType: u64 = 2010;
    /// Invalid position' status for placing a SLTP order
    const InvalidPositionForSLTP: u64 = 2011;

    // Orderbook & OrderedMap -------------------------------------------------------

    /// While creating ordered map with wrong parameters.
    const InvalidMapParameters: u64 = 3000;
    /// While searching for a key, but it doesn't exist.
    const KeyNotExist: u64 = 3001;
    /// While inserting already existing key.
    const KeyAlreadyExists: u64 = 3002;
    /// When attempting to destroy a non-empty map
    const DestroyNotEmpty: u64 = 3003;
    /// Invalid user tries to modify an order
    const InvalidUserForOrder: u64 = 3004;
    /// Orderbook flag requirements violated
    const FlagRequirementsViolated: u64 = 3005;
    /// Minimum size matched not reached
    const NotEnoughLiquidity: u64 = 3006;
    /// When trying to change a map configuration, but the map has
    /// length less than 4
    const MapTooSmall: u64 = 3007;
    /// When taker matches its own order
    const SelfTrading: u64 = 3008;
}
}

#[cfg(test)]
mod tests {
    use super::MoveAbort;

    #[test]
    fn variant_to_code() {
        assert_eq!(MoveAbort::MaxPendingOrdersExceeded as u64, 2000);
        assert_eq!(MoveAbort::SelfTrading as u64, 3008);
        assert_eq!(Ok(MoveAbort::MaxPendingOrdersExceeded), 2000_u64.try_into());
        assert_eq!(Ok(MoveAbort::SelfTrading), 3008_u64.try_into());
    }
}
