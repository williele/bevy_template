use bevy::{prelude::*, window::PresentMode};

mod camera;
mod constant;
mod inspector;

fn main() {
  App::new()
    .insert_resource(ClearColor(constant::CLEAR))
    .insert_resource(WindowDescriptor {
      width: constant::HEIGHT * constant::RESOLUTION,
      height: constant::HEIGHT,
      title: "Demo shader".to_string(),
      present_mode: PresentMode::Fifo,
      resizable: false,
      ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_plugin(camera::CameraPlugin::default())
    .add_plugin(inspector::InspectorPlugin::default())
    .run();
}
