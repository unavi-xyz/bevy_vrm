//! Conversions from VRM 0.0 values to their VRM 1.0 equivalents.

use super::{
    BoneName,
    FirstPersonFlag,
};
use crate::vrm1::vrmc_vrm::{
    first_person::FirstPersonType,
    humanoid::HumanBoneName,
};

/// Thumb bones shift down one joint: 0.0 `ThumbProximal` is 1.0
/// `ThumbMetacarpal`, and 0.0 `ThumbIntermediate` is 1.0 `ThumbProximal`.
impl From<BoneName> for HumanBoneName {
    fn from(value: BoneName) -> Self {
        match value {
            BoneName::Hips => Self::Hips,
            BoneName::LeftUpperLeg => Self::LeftUpperLeg,
            BoneName::RightUpperLeg => Self::RightUpperLeg,
            BoneName::LeftLowerLeg => Self::LeftLowerLeg,
            BoneName::RightLowerLeg => Self::RightLowerLeg,
            BoneName::LeftFoot => Self::LeftFoot,
            BoneName::RightFoot => Self::RightFoot,
            BoneName::Spine => Self::Spine,
            BoneName::Chest => Self::Chest,
            BoneName::Neck => Self::Neck,
            BoneName::Head => Self::Head,
            BoneName::LeftShoulder => Self::LeftShoulder,
            BoneName::RightShoulder => Self::RightShoulder,
            BoneName::LeftUpperArm => Self::LeftUpperArm,
            BoneName::RightUpperArm => Self::RightUpperArm,
            BoneName::LeftLowerArm => Self::LeftLowerArm,
            BoneName::RightLowerArm => Self::RightLowerArm,
            BoneName::LeftHand => Self::LeftHand,
            BoneName::RightHand => Self::RightHand,
            BoneName::LeftToes => Self::LeftToes,
            BoneName::RightToes => Self::RightToes,
            BoneName::LeftEye => Self::LeftEye,
            BoneName::RightEye => Self::RightEye,
            BoneName::Jaw => Self::Jaw,
            BoneName::LeftThumbProximal => Self::LeftThumbMetacarpal,
            BoneName::LeftThumbIntermediate => Self::LeftThumbProximal,
            BoneName::LeftThumbDistal => Self::LeftThumbDistal,
            BoneName::LeftIndexProximal => Self::LeftIndexProximal,
            BoneName::LeftIndexIntermediate => Self::LeftIndexIntermediate,
            BoneName::LeftIndexDistal => Self::LeftIndexDistal,
            BoneName::LeftMiddleProximal => Self::LeftMiddleProximal,
            BoneName::LeftMiddleIntermediate => Self::LeftMiddleIntermediate,
            BoneName::LeftMiddleDistal => Self::LeftMiddleDistal,
            BoneName::LeftRingProximal => Self::LeftRingProximal,
            BoneName::LeftRingIntermediate => Self::LeftRingIntermediate,
            BoneName::LeftRingDistal => Self::LeftRingDistal,
            BoneName::LeftLittleProximal => Self::LeftLittleProximal,
            BoneName::LeftLittleIntermediate => Self::LeftLittleIntermediate,
            BoneName::LeftLittleDistal => Self::LeftLittleDistal,
            BoneName::RightThumbProximal => Self::RightThumbMetacarpal,
            BoneName::RightThumbIntermediate => Self::RightThumbProximal,
            BoneName::RightThumbDistal => Self::RightThumbDistal,
            BoneName::RightIndexProximal => Self::RightIndexProximal,
            BoneName::RightIndexIntermediate => Self::RightIndexIntermediate,
            BoneName::RightIndexDistal => Self::RightIndexDistal,
            BoneName::RightMiddleProximal => Self::RightMiddleProximal,
            BoneName::RightMiddleIntermediate => Self::RightMiddleIntermediate,
            BoneName::RightMiddleDistal => Self::RightMiddleDistal,
            BoneName::RightRingProximal => Self::RightRingProximal,
            BoneName::RightRingIntermediate => Self::RightRingIntermediate,
            BoneName::RightRingDistal => Self::RightRingDistal,
            BoneName::RightLittleProximal => Self::RightLittleProximal,
            BoneName::RightLittleIntermediate => Self::RightLittleIntermediate,
            BoneName::RightLittleDistal => Self::RightLittleDistal,
            BoneName::UpperChest => Self::UpperChest,
        }
    }
}

impl From<FirstPersonFlag> for FirstPersonType {
    fn from(value: FirstPersonFlag) -> Self {
        match value {
            FirstPersonFlag::Auto => Self::Auto,
            FirstPersonFlag::Both => Self::Both,
            FirstPersonFlag::FirstPersonOnly => Self::FirstPersonOnly,
            FirstPersonFlag::ThirdPersonOnly => Self::ThirdPersonOnly,
        }
    }
}
