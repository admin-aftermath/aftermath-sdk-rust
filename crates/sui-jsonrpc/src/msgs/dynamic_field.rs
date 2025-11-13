// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
use std::fmt::Display;

use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, IfIsHumanReadable, serde_as};
use sui_sdk_types::{Address, Digest, Identifier, StructTag, TypeTag, Version};

use super::Page;
use crate::serde::Base64orBase58;

const DYNAMIC_FIELD_MODULE_NAME: &str = "dynamic_field";
const DYNAMIC_FIELD_FIELD_STRUCT_NAME: &str = "Field";

const DYNAMIC_OBJECT_FIELD_MODULE_NAME: &str = "dynamic_object_field";
const DYNAMIC_OBJECT_FIELD_WRAPPER_STRUCT_NAME: &str = "Wrapper";

pub type DynamicFieldPage = Page<DynamicFieldInfo, Address>;

/// Originally `sui_types::dynamic_field::DynamicFieldName`.
#[serde_as]
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DynamicFieldName {
    #[serde_as(as = "IfIsHumanReadable<DisplayFromStr, _>")]
    pub type_: TypeTag,
    #[serde_as(as = "IfIsHumanReadable<_, DisplayFromStr>")]
    pub value: serde_json::Value,
}

impl Display for DynamicFieldName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.type_, self.value)
    }
}

/// Originally `sui_types::dynamic_field::DynamicFieldInfo`.
#[serde_as]
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DynamicFieldInfo {
    pub name: DynamicFieldName,
    #[serde_as(as = "IfIsHumanReadable<Base64orBase58, _>")]
    pub bcs_name: Vec<u8>,
    pub type_: DynamicFieldType,
    pub object_type: String,
    pub object_id: Address,
    pub version: Version,
    pub digest: Digest,
}

impl DynamicFieldInfo {
    pub fn is_dynamic_field(tag: &StructTag) -> bool {
        *tag.address() == Address::TWO
            && tag.module().as_str() == DYNAMIC_FIELD_MODULE_NAME
            && tag.name().as_str() == DYNAMIC_FIELD_FIELD_STRUCT_NAME
    }

    pub fn is_dynamic_object_field_wrapper(tag: &StructTag) -> bool {
        *tag.address() == Address::TWO
            && tag.module().as_str() == DYNAMIC_OBJECT_FIELD_MODULE_NAME
            && tag.name().as_str() == DYNAMIC_OBJECT_FIELD_WRAPPER_STRUCT_NAME
    }

    pub fn dynamic_field_type(key: TypeTag, value: TypeTag) -> StructTag {
        StructTag::new(
            Address::TWO,
            Identifier::from_static(DYNAMIC_FIELD_FIELD_STRUCT_NAME),
            Identifier::from_static(DYNAMIC_FIELD_MODULE_NAME),
            vec![key, value],
        )
    }

    pub fn dynamic_object_field_wrapper(key: TypeTag) -> StructTag {
        StructTag::new(
            Address::TWO,
            Identifier::from_static(DYNAMIC_OBJECT_FIELD_MODULE_NAME),
            Identifier::from_static(DYNAMIC_OBJECT_FIELD_WRAPPER_STRUCT_NAME),
            vec![key],
        )
    }
}

/// Originally `sui_types::dynamic_field::DynamicFieldType`.
#[derive(Clone, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum DynamicFieldType {
    #[serde(rename_all = "camelCase")]
    DynamicField,
    DynamicObject,
}

impl Display for DynamicFieldType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DynamicFieldType::DynamicField => write!(f, "DynamicField"),
            DynamicFieldType::DynamicObject => write!(f, "DynamicObject"),
        }
    }
}
