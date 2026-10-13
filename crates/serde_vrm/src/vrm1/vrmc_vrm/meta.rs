//! Avatar metadata and license terms.

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
#[expect(clippy::struct_excessive_bools, reason = "mirrors the schema")]
pub struct Meta {
    pub name:                               String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version:                            Option<String>,
    pub authors:                            Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copyright_information:              Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_information:                Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub references:                         Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub third_party_licenses:               Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_image:                    Option<u32>,
    pub license_url:                        String,
    #[serde(default)]
    pub avatar_permission:                  AvatarPermission,
    #[serde(default)]
    pub allow_excessively_violent_usage:    bool,
    #[serde(default)]
    pub allow_excessively_sexual_usage:     bool,
    #[serde(default)]
    pub commercial_usage:                   CommercialUsage,
    #[serde(default)]
    pub allow_political_or_religious_usage: bool,
    #[serde(default)]
    pub allow_antisocial_or_hate_usage:     bool,
    #[serde(default)]
    pub credit_notation:                    CreditNotation,
    #[serde(default)]
    pub allow_redistribution:               bool,
    #[serde(default)]
    pub modification:                       Modification,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_license_url:                  Option<String>,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum AvatarPermission {
    #[default]
    OnlyAuthor,
    OnlySeparatelyLicensedPerson,
    Everyone,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum CommercialUsage {
    #[default]
    PersonalNonProfit,
    PersonalProfit,
    Corporation,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum CreditNotation {
    #[default]
    Required,
    Unnecessary,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Modification {
    #[default]
    Prohibited,
    AllowModification,
    AllowModificationRedistribution,
}
