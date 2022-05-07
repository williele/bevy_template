use bevy::{
  prelude::{Commands, OrthographicCameraBundle, Plugin},
  render::camera::ScalingMode,
};

use crate::constant::RESOLUTION;

#[derive(Default)]
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
  fn build(&self, app: &mut bevy::prelude::App) {
    app.add_startup_system(spawn_camera);
  }
}

fn spawn_camera(mut commands: Commands) {
  let mut camera = OrthographicCameraBundle::new_2d();

  camera.orthographic_projection.right = 1.0 * RESOLUTION;
  camera.orthographic_projection.left = -1.0 * RESOLUTION;

  camera.orthographic_projection.top = 1.0;
  camera.orthographic_projection.bottom = -1.0;

  camera.orthographic_projection.scaling_mode = ScalingMode::None;

  commands.spawn_bundle(camera);
}
