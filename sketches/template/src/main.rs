use bevy::{
    prelude::*, DefaultPlugins, render::camera::ScalingMode,
};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

#[derive(Resource)]
struct Rand(ChaCha8Rng);

fn main() {
    App::new()
        .insert_resource(Rand(ChaCha8Rng::seed_from_u64(3)))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_window))
        .add_systems(Update, update)
        .run();
}

fn setup_window(
    mut commands: Commands,
    mut clear_color: ResMut<ClearColor>,
    mut windows: Query<&mut Window>,
) {

    let origin = Vec3::new(0.0, 25.0, 0.0);
    let dist: f32 = 50.0;
    let height: f32 = dist * 0.8534 + origin.y;

    // setup the camera
    commands.spawn((Camera3dBundle {
        camera: Camera {
            hdr: true,
            ..default()
        },
        projection: OrthographicProjection {
            scale: 1.0,
            scaling_mode: ScalingMode::FixedVertical(100.0),
            ..default()
        }.into(),
        transform: Transform::from_xyz(-dist, height, dist).looking_at(origin, Vec3::Y),
        ..default()
    },
    ));

    let mut window = windows.single_mut();
    window.resolution.set(500.0, 500.0);
    clear_color.0 = Color::BLACK;
}

fn update(
    time: Res<Time>,
    mut rng: ResMut<Rand>,
) {
}
