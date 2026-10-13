//! Animation targets for each VRM humanoid bone.

use std::{
    collections::HashMap,
    sync::LazyLock,
};

use bevy::animation::AnimationTargetId;
use serde_vrm::vrm1::vrmc_vrm::humanoid::HumanBoneName;

use crate::animations::target_chain::TargetChain;

macro_rules! push_bone {
    ($map:ident, $chain:ident, $name:ident) => {
        $map.insert(
            HumanBoneName::$name,
            $chain.push_target(HumanBoneName::$name.as_str().to_string()),
        );
    };
}

macro_rules! finger {
    ($map:ident, $chain:ident, $side:ident, $finger:ident) => {
        finger!($map, $chain, $side, $finger, Proximal, Intermediate, Distal);
    };
    (
        $map:ident,
        $chain:ident,
        $side:ident,
        $finger:ident,
        $first:ident,
        $second:ident,
        $third:ident
    ) => {
        paste::paste! {
            {
                let mut chain = $chain.clone();

                push_bone!($map, chain, [<$side $finger $first>]);
                push_bone!($map, chain, [<$side $finger $second>]);
                push_bone!($map, chain, [<$side $finger $third>]);
            }
        }
    };
}

macro_rules! arm {
    ($map:ident, $chain:ident, $side:ident) => {
        paste::paste! {
            {
                let mut chain = $chain.clone();

                push_bone!($map, chain, [<$side Shoulder>]);
                push_bone!($map, chain, [<$side UpperArm>]);
                push_bone!($map, chain, [<$side LowerArm>]);
                push_bone!($map, chain, [<$side Hand>]);

                finger!($map, chain, $side, Thumb, Metacarpal, Proximal, Distal);
                finger!($map, chain, $side, Index);
                finger!($map, chain, $side, Middle);
                finger!($map, chain, $side, Ring);
                finger!($map, chain, $side, Little);
            }
        }
    };
}

macro_rules! leg {
    ($map:ident, $chain:ident, $side:ident) => {
        paste::paste! {
            {
                let mut chain = $chain.clone();

                push_bone!($map, chain, [<$side UpperLeg>]);
                push_bone!($map, chain, [<$side LowerLeg>]);
                push_bone!($map, chain, [<$side Foot>]);
                push_bone!($map, chain, [<$side Toes>]);
            }
        }
    };
}

pub static VRM_ANIMATION_TARGETS: LazyLock<HashMap<HumanBoneName, AnimationTargetId>> =
    LazyLock::new(|| {
        let mut map = HashMap::default();
        let mut chain = TargetChain::default();

        push_bone!(map, chain, Hips);

        leg!(map, chain, Left);
        leg!(map, chain, Right);

        push_bone!(map, chain, Spine);
        push_bone!(map, chain, Chest);
        push_bone!(map, chain, UpperChest);

        arm!(map, chain, Left);
        arm!(map, chain, Right);

        push_bone!(map, chain, Neck);
        push_bone!(map, chain, Head);

        {
            let mut chain = chain.clone();
            push_bone!(map, chain, Jaw);
        }

        {
            let mut chain = chain.clone();
            push_bone!(map, chain, LeftEye);
        }

        {
            let mut chain = chain.clone();
            push_bone!(map, chain, RightEye);
        }

        map
    });
