use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_camera, setup_cubes))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(20., 20., 20.).looking_at(Vec3::ZERO, Vec3::Y),

        ..default()
    });
}

fn setup_cubes(mut commands: Commands, mut mesh_assets: ResMut<Assets<Mesh>>) {
    let mesh = mesh_assets.add(shape::Box::new(1., 1., 1.).into());

    for x in -5..5 {
        for y in -5..5 {
            for z in -5..5 {
                commands.spawn(PbrBundle {
                    mesh: mesh.clone(),
                    transform: Transform::from_xyz(
                        x as f32 * 1.75,
                        y as f32 * 1.75,
                        z as f32 * 1.75,
                    ),
                    ..default()
                });
            }
        }
    }
}
