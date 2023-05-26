use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
pub struct DelayDestroy {
    pub elapsed_seconds_to_destroy: f32,
}

impl DelayDestroy {
    pub fn new(elapsed_seconds_to_destroy: f32) -> Self {
        Self {
            elapsed_seconds_to_destroy,
        }
    }
}

pub fn delay_destroy(
    mut commands: Commands,
    time: Res<Time>,
    q_to_destroy: Query<(Entity, &DelayDestroy)>,
) {
    for (e, d) in q_to_destroy.iter() {
        if d.elapsed_seconds_to_destroy < time.elapsed_seconds() {
            commands.entity(e).despawn_recursive();
        }
    }
}
