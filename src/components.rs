use bevy::prelude::*;


#[derive(Component, Deref, DerefMut)]
pub struct Seek {
    #[deref]
    target: Vec2,
    pub(crate) dist: f32,
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
    #[deref]
    target: Vec2,
} impl Flee {
    pub fn new (target: Vec2) -> Self {
        Flee {
            target
        }
    }
}

#[derive(Component)]
pub struct Evade {
    pub(crate) target: Entity,
    pub(crate) t_pos: Vec2,
    pub(crate) t_velocity: Vec2
} impl Evade {
    pub fn new (target: Entity) -> Self {
        Self {
            t_pos: Vec2::ZERO,
            t_velocity: Vec2::ZERO,
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