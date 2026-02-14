use bevy::prelude::*;

pub const WINDOW_WIDTH: f32 = 1024.;
pub const WINDOW_HEIGHT: f32 = 720.;


//const WINDOW_BOTTOM_Y: f32 = WINDOW_HEIGHT / - 2.;
//const WINDOW_LEFT_X: f32 = WINDOW_WIDTH / - 2.;

pub const GAME_WINDOW_HEIGHT: f32 = 648.;
pub const GAME_WINDOW_WIDTH: f32 = 324.;
pub struct BoardPlugin;

#[derive(Component)]
struct BorderPart;

#[derive(Component)]
struct GridPart;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        //build the board
        app.add_systems(Startup, build_board);
    }
}

// Border which is the parent
// Grid which is the child
fn build_board(mut commands: Commands, asset_server: Res<AssetServer>)
{
    commands.spawn((
        BorderPart,
        Sprite {
            image: asset_server.load("Border.png"),
            custom_size: Some(Vec2::new(GAME_WINDOW_WIDTH,GAME_WINDOW_HEIGHT)),
            ..default()
        },
        Transform::from_xyz(0., 0., 0.),
        children![
            (
                GridPart,
                Sprite {
                    image: asset_server.load("Grid.png"),
                    ..default()
                },
                Transform::from_xyz(0., 0., 0.)
            )
        ]
    ));
}
