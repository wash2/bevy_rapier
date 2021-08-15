use crate::ui::systems;
use bevy::app::{App, Plugin, CoreStage};

pub struct DebugUiPlugin;

impl Plugin for DebugUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(systems::setup_ui)
            .add_system_to_stage(CoreStage::Update, systems::text_update_system);
    }
}
