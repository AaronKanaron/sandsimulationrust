/*- Global allowings -*/


/*- Imports -*/
use std::time::Duration;
use bevy::prelude::*;

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
            .add_systems(Startup, init_cells.in_set(CellSet))
            .add_systems(Update, system_cells.in_set(CellSet));
    }
}

/*- Initialize -*/
fn init_cells(mut commands: Commands) {
    commands.spawn(CellPosition { x: 0, y: 0 });
    commands.spawn(CellPosition { x: 0, y: 12 });
}

fn system_cells(
    mut commands: Commands,
    q_cells: Query<(Entity, &CellPosition)>,
    mut timer: ResMut<NextGenerationTimer>,
    mut cell_params: ResMut<CellParams>,
    time: Res<Time>,
) {
    if cell_params.playing {
        timer.0.tick(time.delta());
        // println!("Timer: {:?}", timer.0.elapsed());
        if timer.0.finished() {
            cell_params.compute_next_gen = true;
        }
    } if cell_params.compute_next_gen {
        cell_params.compute_next_gen = false;
    } else { return }

    for (entity, cell) in &q_cells {
        let scan_pos = CellPosition {
            x: cell.x,
            y: cell.y - 1,
        };

        // if scan_pos.y <= Y_MIN_BOUNDS || scan_pos.y >= Y_MAX_BOUNDS {
        //     println!("Cell is at the edge");
        //     return;
        // } 

        //if there is no cell below, move down
        if !q_cells.iter().any(|(_, pos)| pos == &scan_pos) { // ! There might be a better way to do this
            if scan_pos.y <= Y_MIN_BOUNDS || scan_pos.y >= Y_MAX_BOUNDS {
                println!("{:?} is at the edge", scan_pos);
                // commands.entity(entity).despawn();
                continue;
            } 
            let new_cell = scan_pos;
            commands.entity(entity).despawn();
            commands.spawn(new_cell);
            // println!("Moved cell down to {} {:?} | {:?}", cell.x, cell.y - 1, new_cell);
        } else {
            //Check left and right and move if possible
            for position_delta in NEIGHBOR_DELTA.iter() {
                let scan_pos = CellPosition {
                    x: cell.x + position_delta.0,
                    y: cell.y + position_delta.1,
                };
 
                if !q_cells.iter().any(|(_, pos)| pos == &scan_pos) { //! PRETTY SURE THIS DOES THE RIGHT POSITION FIRST AND THEN THE LEFT IT SHOULD BE RANDOM
                    if scan_pos.x <= X_MIN_BOUNDS || scan_pos.x >= X_MAX_BOUNDS || scan_pos.y <= Y_MIN_BOUNDS || scan_pos.y >= Y_MAX_BOUNDS {
                        println!("{:?} is at the edge", scan_pos);
                        continue;
                    } 
                    let new_cell = scan_pos;
                    commands.entity(entity).despawn();
                    commands.spawn(new_cell);
                    // println!("Moved cell to {:?} | {:?}", scan_pos, new_cell);
                    // break;
                }
            }


        }
    }
}
/*- Method implementations - */



