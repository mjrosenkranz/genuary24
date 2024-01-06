use bevy::{prelude::*, DefaultPlugins, render::camera::ScalingMode};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::f32::consts::TAU;

const STEP: f32 = 0.01;
const MAX_RADIUS: f32 = 50.0;
const MIN_RADIUS: f32 = 0.01;

#[derive(Component)]
struct Particle {
    theta: f32,
    radius: f32,
}

impl Particle {
    fn new(random: f32) -> Self {
        let theta: f32 = TAU * random / STEP;
        let radius = random * MAX_RADIUS;
        Self { theta, radius }
    }

    fn pos(&self) -> Vec3 {
        let height = (self.radius * self.radius - 20.0).log2() * 2.0;
        Vec3::new(
            self.radius * self.theta.cos(),
            height,
            self.radius * self.theta.sin(),
        )
    }
}

#[derive(Resource)]
struct Rand(ChaCha8Rng);

fn main() {
    App::new()
        .insert_resource(Rand(ChaCha8Rng::seed_from_u64(3)))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_window, setup_particles))
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
    commands.spawn(Camera3dBundle {
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
    });

    let mut window = windows.single_mut();
    window.resolution.set(500.0, 500.0);

    clear_color.0 = Color::hex("#2E3961").unwrap();
    // TODO: setup the particles
}

fn setup_particles(
    mut rng: ResMut<Rand>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    // sphere mesh
    let mesh = meshes.add(
        shape::Icosphere {
            radius: 0.5,
            subdivisions: 3,
        }
        .try_into()
        .unwrap(),
    );
    let material = materials.add(StandardMaterial {
        emissive: Color::hex("#E4E7E4").unwrap(),
        ..default()
    });

    for _ in 0..1000 {
        let random = rng.0.gen_range(1.0..100.0) / 100.0;
        let particle = Particle::new(random);

        commands.spawn((
            PbrBundle {
                mesh: mesh.clone(),
                material: material.clone(),
                transform: Transform::from_translation(particle.pos()),
                ..default()
            },
            particle,
        ));
    }
}

fn update(
    time: Res<Time>,
    mut rng: ResMut<Rand>,
    mut particles: Query<(&mut Transform, &mut Particle)>,
) {
    for (mut transform, mut particle) in particles.iter_mut() {
        particle.theta += time.delta_seconds();
        particle.radius -= time.delta_seconds() * 0.3;

        if particle.radius <= MIN_RADIUS {
            let random = rng.0.gen_range(1.0..100.0) / 100.0;
            *particle = Particle::new(random);
            particle.radius = MAX_RADIUS;
        }

        let pos = particle.pos();
        transform.translation = pos;
    }
}
