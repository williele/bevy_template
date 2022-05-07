use bevy::{
  input::Input,
  prelude::{KeyCode, Plugin, ResMut},
};
use bevy_inspector_egui::{WorldInspectorParams, WorldInspectorPlugin};

#[derive(Default)]
pub struct InspectorPlugin {
  pub init_enabled: bool,
}

impl Plugin for InspectorPlugin {
  fn build(&self, app: &mut bevy::prelude::App) {
    app
      .insert_resource(WorldInspectorParams {
        enabled: self.init_enabled,
        ..Default::default()
      })
      .add_plugin(WorldInspectorPlugin::new())
      .add_system(toggle_inspector);
  }
}

fn toggle_inspector(
  input: ResMut<Input<KeyCode>>,
  mut window_params: ResMut<WorldInspectorParams>,
) {
  if input.just_pressed(KeyCode::Grave) {
    window_params.enabled = !window_params.enabled;
  }
}
