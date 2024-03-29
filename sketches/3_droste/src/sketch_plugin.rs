use bevy::{
    prelude::*, DefaultPlugins, render::camera::ScalingMode,
};

pub enum SketchType {
    Sketch2d,
    Sketch3d,
}

pub struct SketchSetupPlugin{
    pub sketch_type: SketchType,
}

impl Plugin for SketchSetupPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(DefaultPlugins)
            .add_systems(Startup, setup_window);

        match self.sketch_type {
            SketchType::Sketch3d => {app.add_systems(Startup, setup_3d);},
            _ => {},
        }
    }
}

fn setup_window(
    mut clear_color: ResMut<ClearColor>,
    mut windows: Query<&mut Window>,
) {
    // setup the camera
    let mut window = windows.single_mut();
    window.resolution.set(500.0, 500.0);
    clear_color.0 = Color::BLACK;
}


fn setup_3d(
    mut commands: Commands,
) {
    let origin = Vec3::new(0.0, 25.0, 0.0);
    let dist: f32 = 50.0;
    let height: f32 = dist * 0.8534 + origin.y;


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

}
