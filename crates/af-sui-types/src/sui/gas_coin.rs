// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::{GAS_MODULE_NAME, GAS_STRUCT_NAME, SUI_FRAMEWORK_ADDRESS, StructTag, TypeTag};

/// One-time-witness representation.
pub struct Gas;

impl Gas {
    pub fn type_() -> StructTag {
        StructTag::new(
            SUI_FRAMEWORK_ADDRESS,
            GAS_STRUCT_NAME.to_owned(),
            GAS_MODULE_NAME.to_owned(),
            Vec::new(),
        )
    }

    pub fn type_tag() -> TypeTag {
        TypeTag::Struct(Box::new(Self::type_()))
    }

    pub fn is_gas(other: &StructTag) -> bool {
        &Self::type_() == other
    }

    pub fn is_gas_type(other: &TypeTag) -> bool {
        match other {
            TypeTag::Struct(s) => Self::is_gas(s),
            _ => false,
        }
    }
}

/// Return `true` if `s` is the type of a gas coin (i.e., 0x2::coin::Coin<0x2::sui::SUI>)
pub fn is_gas_coin(s: &StructTag) -> bool {
    let Some(coin_type) = s.is_coin() else {
        return false;
    };
    Gas::is_gas_type(coin_type)
}
