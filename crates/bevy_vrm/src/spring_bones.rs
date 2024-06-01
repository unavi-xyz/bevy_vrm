use bevy::{prelude::*};

use crate::{SpringBone, SpringBoneLogicState};

pub struct SpringBonePlugin;
impl Plugin for SpringBonePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, do_springbone_logic);
        app.register_type::<SpringBone>();
        app.register_type::<SpringBoneLogicState>();
    }
}

pub fn do_springbone_logic() {}
