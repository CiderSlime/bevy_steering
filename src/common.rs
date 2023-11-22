use bevy::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum MovementSet {
    GetEntitiesTransform,
    PrepareContext,
    Move,
    Finalize
}