// Copyright (c) The Diem Core Contributors
// Copyright (c) The Move Contributors
// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
//
// Most of the code here is highly adapted from
// https://github.com/MystenLabs/sui/tree/main/external-crates/move/crates/move-core-types
#![allow(clippy::use_self)]
#![allow(clippy::option_if_let_else)]

#[expect(clippy::redundant_pub_crate, reason = "Want to keep it explicit")]
pub(crate) mod identifier;
#[cfg(feature = "u256")]
pub mod u256;

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use std::mem;
    use std::str::FromStr as _;

    use crate::{IdentStr, StructTag, TypeTag};

    #[test]
    fn test_type_tag_serde() {
        let a = TypeTag::Struct(Box::new(StructTag::new(
            "0x1".parse().expect("Parses short hex reprs"),
            IdentStr::cast("abc").to_owned(),
            IdentStr::cast("abc").to_owned(),
            vec![TypeTag::U8],
        )));
        let b = serde_json::to_string(&a).unwrap();
        let c: TypeTag = serde_json::from_str(&b).unwrap();
        assert!(a.eq(&c), "Typetag serde error");
        assert_eq!(mem::size_of::<TypeTag>(), 16);
    }

    #[test]
    fn type_tag_with_leading_underscores() {
        // From: https://suiscan.xyz/mainnet/object/0x9a84c763d68f07811c90fd989ca306cc7ff67916881de3da37e5fc811bbc5a8d
        let display = "0xb24b6789e088b876afabca733bed2299fbc9e2d6369be4d1acfa17d8145454d9::swap::Pool<0x2::sui::SUI, 0x1687f9a0321d0e643ba47ba1c8fc9226e4e28b55fe176a5a2675691a458dee9d::_jeet::_JEET>";
        let _: TypeTag = display.parse().unwrap();
    }

    #[test]
    fn struct_tag_with_leading_underscores() {
        // From: https://suiscan.xyz/mainnet/object/0x9a84c763d68f07811c90fd989ca306cc7ff67916881de3da37e5fc811bbc5a8d
        let display = "0xb24b6789e088b876afabca733bed2299fbc9e2d6369be4d1acfa17d8145454d9::swap::Pool<0x2::sui::SUI, 0x1687f9a0321d0e643ba47ba1c8fc9226e4e28b55fe176a5a2675691a458dee9d::_jeet::_JEET>";
        let _: StructTag = display.parse().unwrap();
    }

    #[test]
    fn test_type_tag() {
        for s in &[
            "u64",
            "bool",
            "vector<u8>",
            "vector<vector<u64>>",
            "vector<u16>",
            "vector<vector<u16>>",
            "vector<u32>",
            "vector<vector<u32>>",
            "vector<u128>",
            "vector<vector<u128>>",
            "vector<u256>",
            "vector<vector<u256>>",
            "signer",
            "0x1::M::S",
            "0x2::M::S_",
            "0x3::M_::S",
            "0x4::M_::S_",
            "0x00000000004::M::S",
            "0x1::M::S<u64>",
            "0x1::M::S<u16>",
            "0x1::M::S<u32>",
            "0x1::M::S<u256>",
            "0x1::M::S<0x2::P::Q>",
            "vector<0x1::M::S>",
            "vector<0x1::M_::S_>",
            "vector<vector<0x1::M_::S_>>",
            "0x1::M::S<vector<u8>>",
            "0x1::M::S<vector<u16>>",
            "0x1::M::S<vector<u32>>",
            "0x1::M::S<vector<u64>>",
            "0x1::M::S<vector<u128>>",
            "0x1::M::S<vector<u256>>",
        ] {
            assert!(s.parse::<TypeTag>().is_ok(), "Failed to parse tag {}", s);
        }
    }

    #[test]
    fn test_parse_valid_struct_tag() {
        let valid = vec![
            "0x1::Diem::Diem",
            "0x1::Diem_Type::Diem",
            "0x1::Diem_::Diem",
            "0x1::X_123::X32_",
            "0x1::Diem::Diem_Type",
            "0x1::Diem::Diem<0x1::XDX::XDX>",
            "0x1::Diem::Diem<0x1::XDX::XDX_Type>",
            "0x1::Diem::Diem<u8>",
            "0x1::Diem::Diem<u64>",
            "0x1::Diem::Diem<u128>",
            "0x1::Diem::Diem<u16>",
            "0x1::Diem::Diem<u32>",
            "0x1::Diem::Diem<u256>",
            "0x1::Diem::Diem<bool>",
            "0x1::Diem::Diem<address>",
            "0x1::Diem::Diem<signer>",
            "0x1::Diem::Diem<vector<0x1::XDX::XDX>>",
            "0x1::Diem::Diem<u8,bool>",
            "0x1::Diem::Diem<u8,   bool>",
            "0x1::Diem::Diem<u16,bool>",
            "0x1::Diem::Diem<u32,   bool>",
            "0x1::Diem::Diem<u128,bool>",
            "0x1::Diem::Diem<u256,   bool>",
            "0x1::Diem::Diem<u8  ,bool>",
            "0x1::Diem::Diem<u8 , bool  ,    vector<u8>,address,signer>",
            "0x1::Diem::Diem<vector<0x1::Diem::Struct<0x1::XUS::XUS>>>",
            "0x1::Diem::Diem<0x1::Diem::Struct<vector<0x1::XUS::XUS>, 0x1::Diem::Diem<vector<0x1::Diem::Struct<0x1::XUS::XUS>>>>>",
        ];
        for (i, text) in valid.into_iter().enumerate() {
            let st: StructTag = text.parse().expect("valid StructTag");
            insta::assert_snapshot!(
                format!("valid_struct_tag_{i}"),
                st,
                &format!("\"{text}\".parse()?")
            );
        }
    }

    #[test]
    fn test_parse_struct_tag_with_type_names() {
        let names = vec![
            "address", "vector", "u128", "u256", "u64", "u32", "u16", "u8", "bool", "signer",
        ];

        let mut tests = vec![];
        for name in &names {
            for name_type in &names {
                tests.push(format!("0x1::{name}::{name_type}"))
            }
        }

        let mut instantiations = vec![];
        for ty in &tests {
            for other_ty in &tests {
                instantiations.push(format!("{ty}<{other_ty}>"))
            }
        }

        for text in tests.iter().chain(&instantiations) {
            text.parse::<StructTag>().expect("valid StructTag");
        }
    }

    #[test]
    fn test_parse_struct_tag_short_account_addr() {
        let result: StructTag = "0x2::sui::SUI".parse().expect("should not error");
        insta::assert_snapshot!(
            result.to_string(),
            @"0x0000000000000000000000000000000000000000000000000000000000000002::sui::SUI"
        );
    }

    #[test]
    fn test_parse_struct_tag_long_account_addr() {
        let result: StructTag =
            "0x0000000000000000000000000000000000000000000000000000000000000002::sui::SUI"
                .parse()
                .expect("should not error");
        insta::assert_snapshot!(
            result.to_string(),
            @"0x0000000000000000000000000000000000000000000000000000000000000002::sui::SUI"
        );
    }

    #[test]
    fn test_parse_struct_with_type_param_short_addr() {
        let result =
            StructTag::from_str("0x2::coin::COIN<0x2::sui::SUI>").expect("should not error");
        insta::assert_snapshot!(
            result.to_string(),
            @"0x0000000000000000000000000000000000000000000000000000000000000002::coin::COIN<0x0000000000000000000000000000000000000000000000000000000000000002::sui::SUI>"
        );
    }

    #[test]
    fn test_parse_struct_with_type_param_long_addr() {
        let result = StructTag::from_str("0x0000000000000000000000000000000000000000000000000000000000000002::coin::COIN<0x0000000000000000000000000000000000000000000000000000000000000002::sui::SUI>")
            .expect("should not error");
        insta::assert_snapshot!(
            result.to_string(),
            @"0x0000000000000000000000000000000000000000000000000000000000000002::coin::COIN<0x0000000000000000000000000000000000000000000000000000000000000002::sui::SUI>"
        );
    }

    #[test]
    fn test_complex_struct_tag_with_short_addr() {
        let result =
            StructTag::from_str("0xe7::vec_coin::VecCoin<vector<0x2::coin::Coin<0x2::sui::SUI>>>")
                .expect("should not error");
        insta::assert_snapshot!(
            result.to_string(),
            @"0x00000000000000000000000000000000000000000000000000000000000000e7::vec_coin::VecCoin<vector<0x0000000000000000000000000000000000000000000000000000000000000002::coin::Coin<0x0000000000000000000000000000000000000000000000000000000000000002::sui::SUI>>>"
        );
    }

    #[test]
    fn test_complex_struct_tag_with_long_addr() {
        let result = StructTag::from_str("0x00000000000000000000000000000000000000000000000000000000000000e7::vec_coin::VecCoin<vector<0x0000000000000000000000000000000000000000000000000000000000000002::coin::Coin<0x0000000000000000000000000000000000000000000000000000000000000002::sui::SUI>>>")
            .expect("should not error");
        insta::assert_snapshot!(
            result.to_string(),
            @"0x00000000000000000000000000000000000000000000000000000000000000e7::vec_coin::VecCoin<vector<0x0000000000000000000000000000000000000000000000000000000000000002::coin::Coin<0x0000000000000000000000000000000000000000000000000000000000000002::sui::SUI>>>"
        );
    }

    #[test]
    fn test_dynamic_field_short_addr() {
        let result = StructTag::from_str(
            "0x2::dynamic_field::Field<address, 0xdee9::custodian_v2::Account<0x234::coin::COIN>>",
        )
        .expect("should not error");
        insta::assert_snapshot!(
            result.to_string(),
            @"0x0000000000000000000000000000000000000000000000000000000000000002::dynamic_field::Field<address, 0x000000000000000000000000000000000000000000000000000000000000dee9::custodian_v2::Account<0x0000000000000000000000000000000000000000000000000000000000000234::coin::COIN>>"
        );
    }

    #[test]
    fn test_dynamic_field_long_addr() {
        let result = StructTag::from_str(
            "0x2::dynamic_field::Field<address, 0xdee9::custodian_v2::Account<0x234::coin::COIN>>",
        )
        .expect("should not error");
        insta::assert_snapshot!(
            result.to_string(),
            @"0x0000000000000000000000000000000000000000000000000000000000000002::dynamic_field::Field<address, 0x000000000000000000000000000000000000000000000000000000000000dee9::custodian_v2::Account<0x0000000000000000000000000000000000000000000000000000000000000234::coin::COIN>>"
        );
    }
}
