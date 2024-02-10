/*- Global allowings -*/

/*- Imports -*/
use bevy::prelude::*;
use crate::cell_system::{CellPosition, CellSet};


/*- Constants -*/
const BACKGROUND_COLOR: Color = Color::rgb(0.08, 0.08, 0.08);
const SCALE_DEF: f32 = 1. / 4.;
const SCALE_MAX: f32 = 1.;

/*- Structs, enums & unions -*/
pub struct GuiSystem;

impl Plugin for GuiSystem {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(BACKGROUND_COLOR))
            .add_systems(Startup, init_camera)
            .add_systems(Update, system_draw_new_cells.before(CellSet));
    }
}

// #[derive(Resource)]
// pub struct GuiParams {

// }
/*- Initialize -*/
fn init_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = SCALE_DEF;
    commands.spawn(camera);
}

/*- Update Systems -*/
fn system_draw_new_cells(
    mut commands: Commands,
    // q_new_cells: Query<(Entity, &CellPosition), Added<CellPosition>>
    q_new_cells: Query<(Entity, &CellPosition), Changed<CellPosition>>
) {
    for (entity, position) in q_new_cells.iter() {
        commands.entity(entity)
            .insert(SpriteBundle {
                sprite: Sprite {
                    color: Color::GREEN,
                    custom_size: Some(Vec2::new(1., 1.)),
                    ..Default::default()
                },
                transform: Transform::from_xyz(position.x as f32, position.y as f32, 0.),
                ..Default::default()
            });
    }
}


/*- Method implementations - */