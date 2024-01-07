use bevy::{
    prelude::*, sprite::MaterialMesh2dBundle,
};

pub mod sketch_plugin;
use crate::sketch_plugin::*;

fn main() {
    App::new()
        .add_plugins(SketchSetupPlugin{
            sketch_type: SketchType::Sketch2d,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, bounce_circle)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // outer scene
    // spawn a 2d camera
    commands.spawn(Camera2dBundle::default());

    // make a material for the square
    // TODO: make material a texture?
    let material = materials.add(ColorMaterial {
        color: Color::rgb(0.0, 1.0, 0.0),
        // texture: ()
        ..default()
    });
    // spawn a square
    let mesh = meshes.add(shape::Circle::new(50.0).into());

    commands.spawn((MaterialMesh2dBundle {
        mesh: mesh.into(),
        material,
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        ..default()
    },
    Circle {
        vel: Vec3::new(1.0, -2.0, 0.0).normalize() * 250.0,
    }));
}

#[derive(Component)]
struct Circle {
    vel: Vec3,
}

fn bounce_circle(
    time: Res<Time>,
    mut circle_q: Query<(&mut Transform, &mut Circle)>,
) {
    let (mut transform, mut circle) = circle_q.get_single_mut().unwrap();
    let next_pos = transform.translation + circle.vel * time.delta_seconds();

    if next_pos.x >= 200.0 || next_pos.x <= -200.0 {
        circle.vel.x *= -1.0;
    }
    if next_pos.y >= 200.0 || next_pos.y <= -200.0 {
        circle.vel.y *= -1.0;
    }

    transform.translation += circle.vel * time.delta_seconds();
}
