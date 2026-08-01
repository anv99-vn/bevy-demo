use bevy::prelude::*;

#[derive(Component)]
pub struct Rotator;

#[derive(Component)]
pub struct GameWorld;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(2.0, 2.0, 2.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.25, 0.66, 0.94),
            ..default()
        })),
        Transform::default(),
        Rotator,
        GameWorld,
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(10.0, 0.4, 10.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.15, 0.15, 0.18),
            ..default()
        })),
        Transform::from_xyz(0.0, -2.0, 0.0),
        GameWorld,
    ));

    commands.spawn((
        PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
        GameWorld,
    ));
}

pub fn despawn(mut commands: Commands, query: Query<Entity, With<GameWorld>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

pub fn rotate(time: Res<Time>, mut query: Query<&mut Transform, With<Rotator>>) {
    for mut transform in &mut query {
        let delta = time.delta_secs();
        transform.rotate_x(0.6 * delta);
        transform.rotate_y(0.9 * delta);
    }
}
