//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::{input::keyboard::KeyboardInput, prelude::*};
use bevy_tween::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, handle_keyboard_events)
        .run();
}

#[derive(Component)]
struct Player;

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // circular base
    commands.spawn(PbrBundle {
        mesh: meshes.add(Circle::new(4.0)),
        material: materials.add(Color::WHITE),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });
    // cube
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material: materials.add(Color::rgb_u8(124, 144, 255)),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        })
        .insert(Player);
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn handle_keyboard_events(
    keys: Res<ButtonInput<KeyCode>>,
    player_query: Query<(Entity, &Transform), With<Player>>,
    mut commands: Commands,
    mut keyboard_event_reader: EventReader<KeyboardInput>,
) {
    for _ in keyboard_event_reader.read() {
        if keys.just_pressed(KeyCode::Space) {
            let (entity, transform) = player_query.single();
            let start = transform.translation.clone();
            let mut end = start.clone();

            end.x += 1.0;

            info!("add tween: {:?} -> {:?}", start, end);

            commands.entity(entity).insert((
                SpanTweenerBundle::new(Duration::from_millis(500)).tween_here(),
                EaseFunction::BackIn,
                ComponentTween::new(interpolate::Translation { start, end }),
            ));
        }
    }
}
