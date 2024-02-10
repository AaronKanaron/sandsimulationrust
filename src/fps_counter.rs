use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    
};

#[derive(Component)]
pub struct FpsCounter;

pub struct FpsCounterPlugin;

impl Plugin for FpsCounterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LogDiagnosticsPlugin::default());
            // .add_systems(Update, fps_update_system);
    }
}

// fn fps_init_system(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
// ) {
//     commands.spawn(Text)
// }

// fn fps_update_system(

// ) {

// }