use bevy::{
    prelude::*,
    reflect::{FromReflect, Reflect},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Reflect, Serialize, Deserialize, FromReflect)]
#[reflect(Serialize, Deserialize)]
pub(crate) struct ClosePortalEvent {
    pub(crate) source: Entity,
}
