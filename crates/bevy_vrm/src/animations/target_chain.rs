use bevy::{animation::AnimationTargetId, ecs::name::Name};

/// Utility for creating chains of [AnimationTargetId]s.
#[derive(Default, Clone)]
pub struct TargetChain(Vec<String>);

impl TargetChain {
    /// Add a name to the list and return the new [AnimationTargetId].
    pub fn push_target(&mut self, name: String) -> AnimationTargetId {
        self.0.push(name);
        self.target()
    }

    /// Get the [AnimationTargetId] for the current list of names.
    pub fn target(&self) -> AnimationTargetId {
        let names = self.0.iter().cloned().map(Name::new).collect::<Vec<_>>();
        AnimationTargetId::from_names(names.iter())
    }
}
