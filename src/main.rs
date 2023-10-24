use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    prelude::*,
};
use bevy_flycam::prelude::*;

#[derive(Component)]
struct Rotaty {
    t: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .insert_resource(bevy::pbr::DirectionalLightShadowMap { size: 4096 })
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 0.2,
        })
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
        .add_systems(Startup, (setup_camera, setup_lighting, setup_cubes))
        .add_systems(Update, rotate)
        // Let's go
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            tonemapping: Tonemapping::BlenderFilmic,
            transform: Transform::from_xyz(20., 15., 20.).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        BloomSettings::default(),
        FlyCam,
        FogSettings {
            color: Color::rgba(0.01, 0.01, 0.01, 0.9),
            falloff: FogFalloff::Linear {
                start: 15.0,
                end: 40.0,
            },
            ..default()
        },
    ));
}

fn setup_lighting(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 1000.0,
            ..default()
        },

        ..default()
    });
}

fn setup_cubes(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut material_assets: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = mesh_assets.add(shape::Box::new(1., 1., 1.).into());
    let material = material_assets.add(Color::rgb(15.0, 10.0, 1.0).into());

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
                    .insert(Rotaty {
                        t: (z as f32 * 10.0 + x as f32 + y as f32 * 0.15),
                    });
            }
        }
    }
}

fn rotate(mut entities: Query<(&mut Transform, &Rotaty)>, time: Res<Time>) {
    let time = time.elapsed_seconds();

    for (mut transform, rotaty) in entities.iter_mut() {
        let particle_time = time + rotaty.t;
        transform.rotation = Quat::from_euler(
            EulerRot::XYZ,
            particle_time.sin(),
            particle_time.cos(),
            particle_time.sin(),
        );
    }
}
