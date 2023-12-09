use nanoserde::{DeJson, SerJson};

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct VrmcVrm {
    #[nserde(rename = "specVersion")]
    pub spec_version: String,
    pub meta: Meta,
    pub humanoid: Humanoid,
    #[nserde(rename = "firstPerson")]
    pub first_person: Option<FirstPerson>,
    #[nserde(rename = "lookAt")]
    pub look_at: Option<LookAt>,
    pub expressions: Option<Expressions>,
}

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct Meta {
    pub name: String,
    pub version: Option<String>,
    pub authors: Vec<String>,
    #[nserde(rename = "copyrightInformation")]
    pub copy_right_information: Option<String>,
    #[nserde(rename = "contactInformation")]
    pub contact_information: Option<String>,
    pub reference: Option<Vec<String>>,
    #[nserde(rename = "thirdPartyLicenses")]
    pub third_party_licenses: Option<String>,
    #[nserde(rename = "thumbnailImage")]
    pub thumbnail_image: Option<u32>,
    #[nserde(rename = "licenseUrl")]
    pub license_url: String,
    #[nserde(rename = "avatarPermission")]
    pub avatar_permission: String,
    #[nserde(rename = "allowExcessivelyViolentUsage")]
    pub allow_excessively_violent_usage: Option<bool>,
    #[nserde(rename = "allowExcessivelySexualUsage")]
    pub allow_excessively_sexual_usage: Option<bool>,
    #[nserde(rename = "commercialUsage")]
    pub commercial_usage: Option<String>,
    #[nserde(rename = "allowPoliticalOrReligiousUsage")]
    pub allow_political_or_religious_usage: Option<bool>,
    #[nserde(rename = "allowAntisocialOrHateUsage")]
    pub allow_antisocial_or_hate_usage: Option<bool>,
    #[nserde(rename = "creditNotation")]
    pub credit_notation: Option<String>,
    #[nserde(rename = "allowRedistribution")]
    pub allow_redistribution: Option<bool>,
    pub modification: Option<String>,
    #[nserde(rename = "otherLicenseUrl")]
    pub other_license_url: Option<String>,
}

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct Humanoid {}

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct FirstPerson {}

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct LookAt {}

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct Expressions {}
