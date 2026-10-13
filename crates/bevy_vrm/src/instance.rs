//! Spawning of loaded VRM scenes into the world.

use bevy::prelude::*;

use crate::loader::Vrm;

#[derive(Component, Default)]
pub struct VrmInstance(pub Handle<Vrm>);

/// Marks a [`VrmInstance`] whose scene has been spawned.
#[derive(Component)]
pub struct VrmInstanceReady;

pub(crate) fn spawn_vrm_scenes(
    mut commands: Commands,
    to_spawn: Query<(Entity, &VrmInstance), Without<VrmInstanceReady>>,
    vrms: Res<Assets<Vrm>>,
) {
    for (entity, vrm_handle) in to_spawn.iter() {
        let Some(vrm) = vrms.get(&vrm_handle.0) else {
            continue;
        };

        let scene = match &vrm.gltf.default_scene {
            Some(handle) => handle.clone(),
            None => match vrm.gltf.scenes.first() {
                Some(handle) => handle.clone(),
                None => continue,
            },
        };

        commands
            .entity(entity)
            .insert((WorldAssetRoot(scene), VrmInstanceReady));
    }
}
