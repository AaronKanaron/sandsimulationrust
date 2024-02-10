/*- Mods -*/
mod cell_system;
mod gui;
mod fps_counter;

/*- Imports -*/
use bevy::prelude::*;
use cell_system::CellSystem;
use gui::GuiSystem;
use fps_counter::FpsCounterPlugin;

const X_MAX_BOUNDS: isize = 35;
const X_MIN_BOUNDS: isize = -35;
const Y_MAX_BOUNDS: isize = 35;
const Y_MIN_BOUNDS: isize = -35;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Sand Simulation Test!".to_string(),
                fit_canvas_to_parent: true,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins((CellSystem, GuiSystem, FpsCounterPlugin))
        .run();
}
