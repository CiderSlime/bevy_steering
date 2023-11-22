use bevy::prelude::*;


#[derive(Component, Deref, DerefMut)]
pub struct Seek {
    #[deref]
    target: Vec2,
    pub(crate) dist: f32
} impl Seek {
    pub fn new(target: Vec2) -> Self {
        Seek {
            dist: 45.,
            target
        }
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct Flee {
    pub(crate) target: Entity,
    #[deref]
    pos: Vec2
} impl Flee {
    pub fn new (target: Entity) -> Self {
        Flee {
            pos: Vec2::ZERO,
            target
        }
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct Wander(pub f32);

#[derive(Component, Deref, DerefMut)]
pub struct Velocity{
    #[deref]
    current: Vec2,
    pub(crate) desired: Vec2,
}
impl Velocity {
    pub fn new() -> Self {
        Self {
            current: Vec2::ZERO,
            desired: Vec2::ZERO
        }
    }
    fn is_some(&self) -> bool {
        self.current != Vec2::ZERO
    }
}