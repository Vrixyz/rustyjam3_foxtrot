use crate::level_instantiation::spawning::objects::portal::PortalEntities;
use crate::GameState;
use anyhow::Result;
use bevy::prelude::*;
use bevy_mod_sysfail::macros::*;
use serde::{Deserialize, Serialize};

use self::resources::ClosePortalEvent;

pub mod resources;

pub(crate) fn close_portal_plugin(app: &mut App) {
    app.add_event::<ClosePortalEvent>()
        .add_system(close_portal.in_set(OnUpdate(GameState::Playing)));
}

#[sysfail(log(level = "error"))]
fn close_portal(
    mut commands: Commands,
    mut events: EventReader<ClosePortalEvent>,
    close_portal_target_query: Query<&PortalEntities>,
) -> Result<()> {
    for event in events.iter() {
        let Ok(portal_entities) = close_portal_target_query.get(event.source) else {
            continue;
        };
        for e in portal_entities.0.iter() {
            commands.entity(*e).despawn_recursive();
        }
    }
    Ok(())
}
