use bevy::prelude::*;
use bevy_flycam::prelude::*;

#[derive(Component)]
struct Rotaty {
    t: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        // FlyCam
        .add_plugins(NoCameraPlayerPlugin)
        .insert_resource(MovementSettings {
            sensitivity: 0.00005,
            speed: 7.0,
        })
        .insert_resource(KeyBindings {
            move_ascend: KeyCode::E,
            move_descend: KeyCode::Q,
            ..Default::default()
        })
        // My Stuff
        .add_systems(Startup, (setup_camera, setup_cubes))
        .add_systems(Update, rotate)
        // Let's go
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(20., 15., 20.).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        FlyCam,
    ));
}

fn setup_cubes(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut material_assets: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = mesh_assets.add(shape::Box::new(1., 1., 1.).into());
    let material = material_assets.add(Color::rgb(1.0, 1.0, 1.0).into());

    for x in -5..5 {
        for y in -5..5 {
            for z in -5..5 {
                commands
                    .spawn(PbrBundle {
                        mesh: mesh.clone(),
                        material: material.clone(),

                        transform: Transform::from_xyz(
                            x as f32 * 1.75,
                            y as f32 * 1.75,
                            z as f32 * 1.75,
                        ),

                        ..default()
                    })
                    .insert(Rotaty { t: 0.0 });
            }
        }
    }
}

fn rotate(mut entities: Query<&mut Transform, With<Rotaty>>, time: Res<Time>) {
    let dt = time.delta_seconds();

    for mut transform in entities.iter_mut() {
        transform.rotate_y(1.0 * dt);
    }
}
