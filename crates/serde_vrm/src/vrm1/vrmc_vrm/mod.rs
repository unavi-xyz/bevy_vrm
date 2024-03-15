use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VrmcVrm {
    #[serde(rename = "specVersion")]
    pub spec_version: String,
    pub meta: Meta,
    pub humanoid: Humanoid,
    #[serde(rename = "firstPerson")]
    pub first_person: Option<FirstPerson>,
    #[serde(rename = "lookAt")]
    pub look_at: Option<LookAt>,
    pub expressions: Option<Expressions>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Meta {
    pub name: String,
    pub version: Option<String>,
    pub authors: Vec<String>,
    #[serde(rename = "copyrightInformation")]
    pub copy_right_information: Option<String>,
    #[serde(rename = "contactInformation")]
    pub contact_information: Option<String>,
    pub reference: Option<Vec<String>>,
    #[serde(rename = "thirdPartyLicenses")]
    pub third_party_licenses: Option<String>,
    #[serde(rename = "thumbnailImage")]
    pub thumbnail_image: Option<u32>,
    #[serde(rename = "licenseUrl")]
    pub license_url: String,
    #[serde(rename = "avatarPermission")]
    pub avatar_permission: String,
    #[serde(rename = "allowExcessivelyViolentUsage")]
    pub allow_excessively_violent_usage: Option<bool>,
    #[serde(rename = "allowExcessivelySexualUsage")]
    pub allow_excessively_sexual_usage: Option<bool>,
    #[serde(rename = "commercialUsage")]
    pub commercial_usage: Option<String>,
    #[serde(rename = "allowPoliticalOrReligiousUsage")]
    pub allow_political_or_religious_usage: Option<bool>,
    #[serde(rename = "allowAntisocialOrHateUsage")]
    pub allow_antisocial_or_hate_usage: Option<bool>,
    #[serde(rename = "creditNotation")]
    pub credit_notation: Option<String>,
    #[serde(rename = "allowRedistribution")]
    pub allow_redistribution: Option<bool>,
    pub modification: Option<String>,
    #[serde(rename = "otherLicenseUrl")]
    pub other_license_url: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Humanoid {}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct FirstPerson {}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct LookAt {}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Expressions {}
