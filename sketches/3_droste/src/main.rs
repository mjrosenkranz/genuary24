use bevy::{
    prelude::*, sprite::MaterialMesh2dBundle, render::{render_resource::{TextureDescriptor, TextureDimension, TextureFormat, TextureUsages, Extent3d}, view::RenderLayers, camera::RenderTarget}, core_pipeline::clear_color::ClearColorConfig,
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
    mut images: ResMut<Assets<Image>>,
) {
    // image setup
    let size = Extent3d {
        width: 500,
        height: 500,
        ..default()
    };

    // create an image to render to
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    // fill image.data with zeroes
    image.resize(size);

    let image_handle = images.add(image);

    // --------------------------------------

    // inner scene (default layer is 0)
    let inner_pass = RenderLayers::layer(1);

    // make a material for the square
    // TODO: make material a texture?
    // spawn a square
    let mesh = meshes.add(shape::Circle::new(50.0).into());

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: mesh.into(),
            material: materials.add(ColorMaterial {
                color: Color::rgb(0.0, 1.0, 0.0),
                ..default()
            }),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..default()
        },
        Circle {
            vel: Vec3::new(1.0, -2.0, 0.0).normalize() * 250.0,
        },
        inner_pass,
    ));

    // spawn a 2d camera that targets the image
    commands.spawn((
            Camera2dBundle{
                camera_2d: Camera2d {
                    clear_color: ClearColorConfig::Custom(Color::WHITE),
                    ..default()
                },
                camera: Camera {
                    // render before main pass
                    order: -1,
                    target: RenderTarget::Image(image_handle.clone()),
                    ..default()
                },
                ..default()
            },
            inner_pass,
    ));

    // --------------------------------------

    // outer scene 

    // spawn a 2d camera
    commands.spawn(Camera2dBundle{
        ..default()
    });

    // make a material for the square
    // TODO: make material a texture?
    let material = materials.add(ColorMaterial {
        color: Color::rgb(1.0, 1.0, 0.0),
        texture: Some(image_handle),
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
        vel: Vec3::new(-1.0, -2.0, 0.0).normalize() * 150.0,
    }
    ));

    // -----------------------------------
}

#[derive(Component)]
struct Circle {
    vel: Vec3,
}

fn bounce_circle(
    time: Res<Time>,
    mut circle_q: Query<(&mut Transform, &mut Circle)>,
) {
    for (mut transform, mut circle) in circle_q.iter_mut() {
    let next_pos = transform.translation + circle.vel * time.delta_seconds();

    if next_pos.x >= 200.0 || next_pos.x <= -200.0 {
        circle.vel.x *= -1.0;
    }
    if next_pos.y >= 200.0 || next_pos.y <= -200.0 {
        circle.vel.y *= -1.0;
    }

    transform.translation += circle.vel * time.delta_seconds();
    }
}
