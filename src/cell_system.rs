/*- Global allowings -*/


/*- Imports -*/
use std::{cell, time::Duration};
use rand::Rng;
use bevy::{prelude::*, window::PrimaryWindow};

use crate::{X_MAX_BOUNDS, X_MIN_BOUNDS, Y_MAX_BOUNDS, Y_MIN_BOUNDS};

/*- Constants -*/
static NEIGHBOR_DELTA: [(isize, isize); 2] = [(-1, -1), (1, -1)];

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
            .insert_resource(NextGenerationTimer(Timer::new(Duration::from_millis(10), TimerMode::Repeating)))
            .add_systems(Update, system_handle_mouse)
            .add_systems(Startup, init_cells.in_set(CellSet))
            .add_systems(Update, system_cells.in_set(CellSet));
    }
}

/*- Initialize -*/
fn init_cells(mut commands: Commands) {
    // commands.spawn(CellPosition { x: 0, y: 0, static_cell: false});
    // commands.spawn(CellPosition { x: 0, y: 12, static_cell: false});
    // commands.spawn(CellPosition { x: 0, y: -20, static_cell: true});
}

// fn system_cells(
//     mut commands: Commands,
//     q_cells: Query<(Entity, &CellPosition)>,
//     mut timer: ResMut<NextGenerationTimer>,
//     mut cell_params: ResMut<CellParams>,
//     time: Res<Time>,
// ) {
//     // Run next generation if the timer is finished.
//     if cell_params.playing {
//         timer.0.tick(time.delta());
//         if timer.0.finished() { cell_params.compute_next_gen = true }
//     }
//     if cell_params.compute_next_gen { cell_params.compute_next_gen = false }
//     else { return }

//     for (entity, cell_position) in q_cells.iter() {
//         if cell_position.static_cell { continue; }         // We do not want to iterate over static cells

//         let scan_pos_below = CellPosition {
//             x: cell_position.x,
//             y: cell_position.y - 1,
//             static_cell: true,
//         };

//         let scan_pos_right = CellPosition {
//             x: cell_position.x + 1,
//             y: cell_position.y,
//             static_cell: true,
//         };

//         let scan_pos_left = CellPosition {
//             x: cell_position.x - 1,
//             y: cell_position.y,
//             static_cell: true,
//         };

//         //check if any cell is equal to the scan position scan pos below
//         if !q_cells.iter().any(|(_, pos)| pos == &scan_pos_below) {
//             if scan_pos_below.y <= Y_MIN_BOUNDS || scan_pos_below.y >= Y_MAX_BOUNDS || scan_pos_below.x <= X_MIN_BOUNDS || scan_pos_below.x >= X_MAX_BOUNDS {

//                 continue;
//             }
//             /*
//             ! Lots of warnings from this code when spawning entities,
//             ! I think it has to do with the fact that the cells gets layered
//             ! on top of each other and it cannot despawn the entity because it already got despawned by the cell below it.
//             */
//             commands.spawn(scan_pos);
//             commands.entity(entity).despawn();
//         } else {
//             //Check left and right and move if possible
//             for position_delta in NEIGHBOR_DELTA.iter() {
//                 let scan_pos = CellPosition {
//                     x: cell.x + position_delta.0,
//                     y: cell.y + position_delta.1,
//                 };
//                 // ! PRETTY SURE THIS DOES THE RIGHT POSITION FIRST AND THEN THE LEFT IT SHOULD BE RANDOM
//                 // ! Om man skapar en pyramid, och sedan placerar en cell högst upp blir det en cell till höger och en cell till vänster
//                 // ! En bättre lösning hade varit att slumpa vilken sida som cellen hamnar på.
//                 // ! Anledningen till att den varnar då är på grund av att den försöker
//                 // ! Despawna en cell som inte längre existerar, eftersom att den blev bortagen av en utav de två cellerna som cellen splittades till.
//                 if !q_cells.iter().any(|(_, pos)| pos == &scan_pos) { 
//                     if scan_pos.x <= X_MIN_BOUNDS || scan_pos.x >= X_MAX_BOUNDS || scan_pos.y <= Y_MIN_BOUNDS || scan_pos.y >= Y_MAX_BOUNDS {
//                         // println!("{:?} is at the edge", scan_pos);
//                         continue;
//                     } 
//                     commands.entity(entity).despawn();
//                     commands.spawn(scan_pos);
//                     // println!("Moved cell to {:?} | {:?}", scan_pos, new_cell);
//                     // break;
//                 }
//             }
//         }
//     }
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

        if scan_pos_below.1 <= Y_MIN_BOUNDS || scan_pos_below.1 >= Y_MAX_BOUNDS || scan_pos_below.0 <= X_MIN_BOUNDS || scan_pos_below.0 >= X_MAX_BOUNDS {
            cell_position.static_cell = true;
            continue;
        }

        if !static_cell_positions.contains(&scan_pos_below) {
            cell_position.y -= 1
        } else if !static_cell_positions.contains(&scan_pos_right) && !static_cell_positions.contains(&scan_pos_left) {
            let mut rng = rand::thread_rng();

            //generate random boolean
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
) {
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

    let new_cell = CellPosition {
        x: target_pos.x as isize,
        y: target_pos.y as isize,
        static_cell: false,
    };

    commands.spawn(new_cell);
}

