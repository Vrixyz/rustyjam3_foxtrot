use crate::file_system_interaction::asset_loading::{AnimationAssets, SceneAssets};
use crate::level_instantiation::spawning::objects::GameCollisionGroup;
use crate::level_instantiation::spawning::GameObject;
use crate::movement::general_movement::{CharacterAnimations, CharacterControllerBundle, Model};
use crate::player_control::camera::IngameCamera;
use crate::world_interaction::dialog::{DialogId, DialogTarget};
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy_basic_portals::*;
use bevy_rapier3d::prelude::*;
use std::f32::consts::TAU;

use super::util::MeshAssetsExt;

pub(crate) const HEIGHT: f32 = 0.4;
pub(crate) const RADIUS: f32 = 0.4;

#[derive(Component)]
pub(crate) struct PortalSpawnPoint(pub Option<CreatePortalBundle>);

fn get_or_add_mesh_handle(mesh_assets: &mut Assets<Mesh>) -> Handle<Mesh> {
    const MESH_HANDLE: HandleUntyped =
        HandleUntyped::weak_from_u64(Mesh::TYPE_UUID, 0x1f40128bac02a9c);
    mesh_assets.get_or_add(MESH_HANDLE, || {
        Mesh::from(shape::Quad::new(Vec2::new(5., 5.)))
    })
}

pub(crate) fn spawn(
    In(transform): In<Transform>,
    mut commands: Commands,
    animations: Res<AnimationAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    scene_handles: Res<SceneAssets>,
    camera_query: Query<Entity, With<IngameCamera>>,
) {
    // TODO: I'm trying to input the basic example from https://github.com/Selene-Amanita/bevy_basic_portals/blob/main/examples/basic/main.rs there.
    // It's failing with a horrible error message :'(

    let portal_mesh = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(10., 10.))));
    commands.spawn(CreatePortalBundle {
        mesh: portal_mesh,
        // This component will be deleted and things that are needed to create the portal will be created
        create_portal: CreatePortal {
            destination: AsPortalDestination::Create(CreatePortalDestination {
                transform: Transform::from_xyz(20., 0., 0.),
                ..default()
            }),
            // Uncomment this to see the portal
            /*debug: Some(DebugPortal {
                show_window: false,
                ..default()
            }),*/
            // I have to provide a camera because there are multiple cameras in foxtrot.
            main_camera: None,
            ..default()
        },
        ..default()
    });

    let sphere_mesh = meshes.add(Mesh::from(shape::UVSphere {
        radius: 2.,
        ..default()
    }));
    commands.spawn(PbrBundle {
        mesh: sphere_mesh,
        transform: Transform::from_xyz(20., 0., -5.),
        ..default()
    });
    // End remove

    // This is my attempt to create something more complex, but let's focus on the minimal above.

    /*
    let mesh_handle = get_or_add_mesh_handle(&mut meshes);
    /*commands.spawn(CreatePortalBundle {
        mesh: mesh_handle,
        portal_transform: transform,
        // This component will be deleted and things that are needed to create the portal will be created
        create_portal: CreatePortal {
            destination: AsPortalDestination::Create(CreatePortalDestination {
                transform: Transform::from_xyz(
                    transform.translation.x,
                    150.,
                    transform.translation.z - 5.,
                ),
            }),
            // Uncomment this to see the portal
            debug: Some(DebugPortal {
                show_window: false,
                ..default()
            }),
            main_camera: Some(camera_query.iter().next().unwrap()),
            ..default()
        },
        ..default()
    });*/

    let entity = commands
        .spawn((
            PbrBundle {
                transform: Transform::from_xyz(
                    transform.translation.x,
                    2., //150.,
                    transform.translation.z,
                ),
                ..default()
            },
            Name::new("Portal"),
            CharacterControllerBundle::capsule(HEIGHT, RADIUS),
            //Follower,
            CharacterAnimations {
                idle: animations.character_idle.clone(),
                walk: animations.character_walking.clone(),
                aerial: animations.character_running.clone(),
            },
            DialogTarget {
                dialog_id: DialogId::new("follower"),
            },
            GameObject::Npc,
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("NPC Dialog Collider"),
                Collider::cylinder(HEIGHT / 2., RADIUS * 5.),
                Sensor,
                ActiveEvents::COLLISION_EVENTS,
                ActiveCollisionTypes::DYNAMIC_DYNAMIC,
                CollisionGroups::new(
                    GameCollisionGroup::OTHER.into(),
                    GameCollisionGroup::PLAYER.into(),
                ),
            ));
        })
        .id();
    commands
        .spawn((
            Model { target: entity },
            SpatialBundle::default(),
            Name::new("Portal Model Parent"),
        ))
        .with_children(|parent| {
            parent.spawn((
                SceneBundle {
                    scene: scene_handles.character.clone(),
                    transform: Transform {
                        translation: Vec3::new(0., -HEIGHT / 2. - RADIUS, 0.),
                        scale: Vec3::splat(0.012),
                        rotation: Quat::from_rotation_y(TAU / 2.),
                    },
                    ..default()
                },
                Name::new("Portal Model"),
            ));
        });*/
}
