use bevy::{animation::AnimationTargetId, core::Name};

/// Utility for creating chains of [AnimationTargetId]s.
#[derive(Default, Clone)]
pub struct TargetChain<'a> {
    pub names: Vec<String>,
    /// Prefix added before each added name, when using [TargetChain::push_target].
    pub prefix: &'a str,
}

impl<'a> TargetChain<'a> {
    /// Add a name to the list and return the new [AnimationTargetId].
    pub fn push_target(&mut self, name: &str) -> AnimationTargetId {
        let value = format!("{}{}", self.prefix, name);
        self.names.push(value);
        self.target()
    }

    /// Get the [AnimationTargetId] for the current list of names.
    pub fn target(&self) -> AnimationTargetId {
        let names = self
            .names
            .clone()
            .into_iter()
            .map(Name::new)
            .collect::<Vec<_>>();
        AnimationTargetId::from_names(names.iter())
    }
}
