/*- Global allowings -*/


/*- Imports -*/
use std::time::Duration;
use rand::Rng;
use bevy::{prelude::*, window::PrimaryWindow};

use crate::{X_MAX_BOUNDS, X_MIN_BOUNDS, Y_MAX_BOUNDS, Y_MIN_BOUNDS};

/*- Constants -*/
const PI: f64 = 3.14159265359;

/*- Structs, enums & unions -*/
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct CellSet;

#[derive(Component, Clone, Copy, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct CellPosition {
    pub x: isize,
    pub y: isize,
    pub static_cell: bool,
}

#[derive(Resource)]
pub struct CellParams {
    pub playing: bool,
    pub compute_next_gen: bool,
}

impl Default for CellParams {
    fn default() -> Self {
        CellParams {
            playing: true,
            compute_next_gen: false,
        }
    }
}

#[derive(Resource)]
pub struct NextGenerationTimer(Timer);

pub struct CellSystem;

impl Plugin for CellSystem {
    fn build(&self, app: &mut App) {
        let cell_params = CellParams::default();
        app.insert_resource(cell_params)
            .insert_resource(NextGenerationTimer(Timer::new(Duration::from_millis(1), TimerMode::Repeating)))
            .add_systems(Update, system_handle_mouse)
            // .add_systems(Startup, init_cells.in_set(CellSet))
            .add_systems(Update, system_cells.in_set(CellSet));
    }
}

/*- Initialize -*/
// fn init_cells(mut commands: Commands) {
//     // commands.spawn(CellPosition { x: 0, y: 0, static_cell: false});
//     // commands.spawn(CellPosition { x: 0, y: 12, static_cell: false});
//     // commands.spawn(CellPosition { x: 0, y: -20, static_cell: true});
// }

fn system_cells(
    // mut commands: Commands,
    mut q_cells: Query<(Entity, &mut CellPosition)>,
    mut timer: ResMut<NextGenerationTimer>,
    mut cell_params: ResMut<CellParams>,
    time: Res<Time>,
) {
    // Run next generation if the timer is finished.
    if cell_params.playing {
        timer.0.tick(time.delta());
        if timer.0.finished() { cell_params.compute_next_gen = true }
    }
    if cell_params.compute_next_gen { cell_params.compute_next_gen = false }
    else { return }

    let static_cell_positions: Vec<(isize, isize)> = q_cells
        .iter()
        .filter(|(_, cell_position)| cell_position.static_cell)
        .map(|(_, cell_position)| (cell_position.x, cell_position.y))
        .collect();

    for (_, mut cell_position) in q_cells.iter_mut() {
        if cell_position.static_cell { continue; }         // We do not want to iterate over static cells

        let scan_pos_below = (cell_position.x, cell_position.y - 1);
        let scan_pos_right = (cell_position.x + 1, cell_position.y - 1);
        let scan_pos_left = (cell_position.x - 1, cell_position.y - 1);

        // Skip out of bounds cells and mark them as static since they can't move
        if scan_pos_below.1 < Y_MIN_BOUNDS || scan_pos_below.1 > Y_MAX_BOUNDS || scan_pos_below.0 < X_MIN_BOUNDS || scan_pos_below.0 > X_MAX_BOUNDS {
            cell_position.static_cell = true;
            continue;
        }

        if !static_cell_positions.contains(&scan_pos_below) {
            cell_position.y -= 1
        } else if !static_cell_positions.contains(&scan_pos_right) && !static_cell_positions.contains(&scan_pos_left) {
            let mut rng = rand::thread_rng();

            //if boolean true, move right, else move left
            if rng.gen() {
                cell_position.x += 1;
                cell_position.y -= 1;
            } else {
                cell_position.x -= 1;
                cell_position.y -= 1;
            }
        } else if !static_cell_positions.contains(&scan_pos_right) {
            cell_position.x += 1;
            cell_position.y -= 1;
        } else if !static_cell_positions.contains(&scan_pos_left) {
            cell_position.x -= 1;
            cell_position.y -= 1;
        } else {
            //this means that the cell below is a static cell and it can not move anywhere
            cell_position.static_cell = true;
        }
    }
}

/*- Method implementations - */

fn system_handle_mouse(
    mut commands: Commands,
    mouse_button: Res<Input<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    q_cells: Query<(Entity, &CellPosition)>
) {
    let ball_size = 4;
    if !mouse_button.pressed(MouseButton::Left) { return }
    
    // Get the cursor position
    let Some(cursor_position) = q_windows.single().cursor_position() else { return };

    let (camera, camera_transform) = q_camera.single();

    // Convert the cursor position to world coordinates
    let Some(target_pos) = camera.viewport_to_world(camera_transform, cursor_position)
        .map(|ray| ray.origin.truncate().round())
        else { return };
    
    if target_pos.x < X_MIN_BOUNDS as f32 || target_pos.x > X_MAX_BOUNDS as f32 || target_pos.y < Y_MIN_BOUNDS as f32 || target_pos.y > Y_MAX_BOUNDS as f32 {
        return;
    }

    let deltas = vec![(0, 0), (0, 1), (1, 0), (1, 1)];

    for (dx, dy) in deltas {
        let new_cell = CellPosition {
            x: (target_pos.x + dx as f32) as isize,
            y: (target_pos.y + dy as f32) as isize,
            static_cell: false,
        };

        //check if there is already a cell at the position
        if q_cells.iter().any(|(_, cell_position)| cell_position.x == new_cell.x && cell_position.y == new_cell.y) {
            continue;
        }

        //check if outside boundaries
        if new_cell.x < X_MIN_BOUNDS || new_cell.x > X_MAX_BOUNDS || new_cell.y < Y_MIN_BOUNDS || new_cell.y > Y_MAX_BOUNDS {
            continue;
        }

        commands.spawn(new_cell);
    }

    // let new_cell = CellPosition {
    //     x: target_pos.x as isize,
    //     y: target_pos.y as isize,
    //     static_cell: false,
    // };

    // commands.spawn(new_cell);
}

