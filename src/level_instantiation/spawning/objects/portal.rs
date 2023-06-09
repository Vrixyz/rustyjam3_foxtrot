use crate::file_system_interaction::asset_loading::{AnimationAssets, SceneAssets};
use crate::level_instantiation::spawning::objects::GameCollisionGroup;
use crate::level_instantiation::spawning::GameObject;
use crate::movement::general_movement::{CharacterAnimations, Model};
use crate::player_control::camera::IngameCamera;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy_basic_portals::*;
use bevy_rapier3d::prelude::*;
use serde::{Deserialize, Serialize};
use std::f32::consts::TAU;

use super::util::MeshAssetsExt;

pub(crate) const HEIGHT: f32 = 0.4;
pub(crate) const RADIUS: f32 = 0.4;

#[derive(Component, Reflect)]
pub struct PortalEntities(pub Vec<Entity>);

#[derive(Component, Debug, Clone, Eq, PartialEq, Reflect, Serialize, Deserialize, FromReflect)]
#[reflect(Serialize, Deserialize)]
pub struct ClosePortalTarget;

fn get_or_add_mesh_portal_handle(mesh_assets: &mut Assets<Mesh>) -> Handle<Mesh> {
    const MESH_HANDLE: HandleUntyped =
        HandleUntyped::weak_from_u64(Mesh::TYPE_UUID, 0x1f40128bac02a9c);
    mesh_assets.get_or_add(MESH_HANDLE, || {
        Mesh::from(shape::Quad::new(Vec2::new(1., 2.)))
    })
}

pub(crate) fn spawn(
    In(transform): In<Transform>,
    time: Res<Time>,
    mut commands: Commands,
    animations: Res<AnimationAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    scene_handles: Res<SceneAssets>,
    camera_query: Query<Entity, With<IngameCamera>>,
) {
    let portal_die_time = time.elapsed_seconds() + 15f32;
    //let delay_destroy = DelayDestroy::new(portal_die_time);
    let portal_mesh = get_or_add_mesh_portal_handle(&mut meshes);
    let portal_position = Transform::from_xyz(
        transform.translation.x,
        transform.translation.y + 150.,
        transform.translation.z,
    );
    let portal_entity = commands
        .spawn((
            ClosePortalTarget,
            RigidBody::KinematicPositionBased,
            CreatePortalBundle {
                mesh: portal_mesh,
                portal_transform: transform,
                // This component will be deleted and things that are needed to create the portal will be created
                create_portal: CreatePortal {
                    destination: AsPortalDestination::Create(CreatePortalDestination {
                        transform: portal_position,
                        ..default()
                    }),
                    // Uncomment this to see the portal
                    // debug: Some(DebugPortal {
                    //     show_window: false,
                    //     ..default()
                    // }),
                    // I have to provide a camera because there are multiple cameras in foxtrot.
                    main_camera: Some(camera_query.iter().next().unwrap()),

                    ..default()
                },
                ..default()
            },
            //delay_destroy,
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("Portal Close Collider"),
                Collider::cylinder(HEIGHT, RADIUS * 5.),
                Sensor,
                ActiveEvents::COLLISION_EVENTS,
                CollisionGroups::new(
                    GameCollisionGroup::OTHER.into(),
                    GameCollisionGroup::PLAYER.into(),
                ),
            ));
        })
        .id();
    let mut inside_portal_pos = portal_position.translation;
    inside_portal_pos += Vec3::Z * -1.;
    // End remove

    // This is my attempt to create something more complex, but let's focus on the minimal above.

    //inside_portal_pos += Vec3::Z * -2.;

    let model_entity = commands
        .spawn((
            (
                PbrBundle {
                    transform: Transform::from_translation(inside_portal_pos),
                    ..default()
                },
                Name::new("Character model"),
                //Follower,
                CharacterAnimations {
                    idle: animations.character_idle.clone(),
                    walk: animations.character_walking.clone(),
                    aerial: animations.character_running.clone(),
                },
            ),
            //delay_destroy,
        ))
        .id();
    let scene_entity = commands
        .spawn((
            Model {
                target: model_entity,
            },
            SpatialBundle::default(),
            Name::new("Portal Model Parent"),
            //delay_destroy,
        ))
        .with_children(|parent| {
            parent.spawn((
                SceneBundle {
                    scene: scene_handles.character.clone(),
                    transform: Transform {
                        translation: Vec3::new(0., -HEIGHT / 2. - RADIUS, 0.),
                        scale: Vec3::splat(0.012),
                        ..default()
                    },
                    ..default()
                },
                Name::new("Portal Model"),
            ));
        })
        .id();

    commands.entity(portal_entity).insert(PortalEntities(vec![
        portal_entity,
        model_entity,
        scene_entity,
    ]));
}
