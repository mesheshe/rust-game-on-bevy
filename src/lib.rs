use bevy::prelude::*;

pub struct BoardPlugin;

#[derive(Component)]
struct BoardPiece;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        //build the board
        app.add_systems(Startup, build_board);
    }
}


fn build_board(mut commands: Commands, asset_server: Res<AssetServer>)
{
    commands.spawn((
        BoardPiece,
        Sprite {
            image: asset_server.load("Blue.png"),
            
            ..default()
        }
    ));
}
