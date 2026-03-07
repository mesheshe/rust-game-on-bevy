use bevy::prelude::*;

pub const WINDOW_WIDTH: f32 = 1024.;
pub const WINDOW_HEIGHT: f32 = 720.;


//const WINDOW_BOTTOM_Y: f32 = WINDOW_HEIGHT / - 2.;
//const WINDOW_LEFT_X: f32 = WINDOW_WIDTH / - 2.;
// https://tetris.fandom.com/wiki/Super_Rotation_System

enum TetrominoPiece {I, O, T, J, L, S, Z}

const BORDER_PADDING: f32 = 32.4 / 2.;



pub const GAME_WINDOW_HEIGHT: f32 = 648.;
pub const GAME_WINDOW_WIDTH: f32 = 324.;
pub struct BoardPlugin;

#[derive(Component)]
struct BorderPart;
#[derive(Component)]
struct Tetromino(TetrominoPiece);

#[derive(Resource)]
pub struct BoardData{
    pub data: [[u8;20]; 10]    
}

// PositionComponent, ShapeComponent, jk
// RenderComponent-color, ActiveComponent

// Handle Input SysteM(rotation, lateral movement, and hard/soft drops)
// 



/*
    L J I O S Z T

   rotate is that a system
    what about wall kicks
*/
#[derive(Component)]
struct GridPart;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        //build the board
        app
            .add_systems(Startup, (build_board, test_piece).chain());
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
            custom_size: Some(Vec2::new(GAME_WINDOW_WIDTH + BORDER_PADDING,GAME_WINDOW_HEIGHT + BORDER_PADDING - 0.2)),
            ..default()
        },
        Transform::from_xyz(0., 0., 1.0),
        children![
            (
                //Visibility::Hidden, 
                GridPart,
                Sprite {
                    image: asset_server.load("Grid.png"),
                    image_mode: SpriteImageMode::Tiled { tile_x: true, tile_y: true, stretch_value: 1.6875 / 10.0}, // 1.68 = (Size of Rect / Size of image) 
                    custom_size: Some(Vec2::new(GAME_WINDOW_WIDTH,GAME_WINDOW_HEIGHT)),
                    ..default()
                },
                Transform::from_xyz(0., 0., -0.1)
            )
        ]
    ));

}


// has rendered to the screen the small piece
// now we want to work on the layer before 
// Render map system -> Will render the internal map for player to csee
fn test_piece(mut commands: Commands, asset_server: Res<AssetServer>)
{
    const X: f32 = 9.; // (0 -> 9)
    const Y: f32 = 17.;// (0 -> 19)

    let (CORRECTED_X, CORRECTED_Y) = position_correction(X, Y);
    // Spawn a second piece and do it in such a way you are not calling a draw call 
    // every time 

    commands.spawn((
        Tetromino(TetrominoPiece::I),
        Sprite {
            image: asset_server.load("Yellow.png"),
            custom_size: Some(Vec2::new(32.4,32.4)),
            ..default()
        },
        Transform::from_xyz(CORRECTED_X, CORRECTED_Y, 1.0),
    ));
}

fn position_correction(x: f32, y: f32) -> (f32, f32){
    let corrected_x: f32 = x - 5.; // (-5 -> 4) left to right
    let corrected_y: f32 = -y + 9.; //(9 -> -10) to[ to bottom ]

    (corrected_x * 32.4 + 32.4 / 2., corrected_y * 32.4 + 32.4 / 2.)
}