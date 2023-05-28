use crate::level_instantiation::spawning::objects::portal::PortalEntities;
use crate::GameState;
use anyhow::Result;
use bevy::prelude::*;
use bevy_mod_sysfail::macros::*;
use serde::{Deserialize, Serialize};

use self::resources::ClosePortalEvent;

use super::interactions_ui::InteractionOpportunities;

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
    mut interaction_opportunities: ResMut<InteractionOpportunities>,
) -> Result<()> {
    for event in events.iter() {
        dbg!("Received ClosePortalEvent");
        let Ok(portal_entities) = close_portal_target_query.get(event.source) else {
            continue;
        };
        dbg!("Removing entities");
        for e in portal_entities.0.iter() {
            commands.entity(*e).despawn_recursive();
            interaction_opportunities.0.remove(e);
        }
    }
    Ok(())
}
