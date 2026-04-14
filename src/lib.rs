use bevy::{prelude::*, reflect::Enum};
use std::convert;

pub const WINDOW_WIDTH: f32 = 1024.;
pub const WINDOW_HEIGHT: f32 = 720.;


//const WINDOW_BOTTOM_Y: f32 = WINDOW_HEIGHT / - 2.;
//const WINDOW_LEFT_X: f32 = WINDOW_WIDTH / - 2.;
// https://tetris.fandom.com/wiki/Super_Rotation_System

#[repr(u8)]
#[derive(PartialEq, Copy, Clone)]
pub enum TetrominoPiece {None, I, O, T, J, L, S, Z}

impl TetrominoPiece {
    const TETROMINO_PIECE_VARIANTS: [TetrominoPiece; 8] = [
        TetrominoPiece::None, TetrominoPiece::I, TetrominoPiece::O, TetrominoPiece::T, TetrominoPiece::J, TetrominoPiece::L, TetrominoPiece::S, TetrominoPiece::Z];
}

impl TryFrom<&u8> for TetrominoPiece {
    
    type Error = &'static str; 

    fn try_from(value: &u8) -> Result<TetrominoPiece, Self::Error> {
        for enum_value in TetrominoPiece::TETROMINO_PIECE_VARIANTS{
            if *value == enum_value as u8 { return Ok(enum_value) }
        }
        Err("Wrong input")
    }

}

const BORDER_PADDING: f32 = 32.4 / 2.;



pub const GAME_WINDOW_HEIGHT: f32 = 648.;
pub const GAME_WINDOW_WIDTH: f32 = 324.;

pub const HOLD_WINDOW_HEIGHT: f32 = GAME_WINDOW_HEIGHT / 20. * 5.;
pub const HOLD_WINDOW_WIDTH: f32 = GAME_WINDOW_WIDTH / 10. * 6.;


pub struct BoardPlugin;

#[derive(Component)]
struct BorderPart;
#[derive(Component)]
struct Tetromino {
    pub COLOR_LIST : [&'static str; 7]
}

impl Tetromino {
    const COLOR_LIST : [&'static str; 7] =  ["Cyan.png",
                                            "Orange.png",
                                            "Blue.png",
                                            "Yellow.png",
                                            "Red.png",
                                            "Green.png",
                                            "Purple.png"];

    const TILE_SIZE: f32 = 32.4;

    fn get_color(tetromino: TetrominoPiece) -> usize {
        match tetromino {
            TetrominoPiece::I => 0,
            TetrominoPiece::J => 1,
            TetrominoPiece::L => 2,
            TetrominoPiece::O => 3,
            TetrominoPiece::S => 4,
            TetrominoPiece::Z => 5,
            TetrominoPiece::T => 6,
            _ => 1000,
        }
    }
}

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
            .add_systems(Startup, build_board)
            .add_systems(Update, draw_board);
    }
}

// Border which is the parent
// Grid which is the child
fn build_board(mut commands: Commands, asset_server: Res<AssetServer>)
{
    // game board
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
                    image_mode: SpriteImageMode::Tiled { tile_x: true, tile_y: true, stretch_value: ((32.4)/(192.))}, // = (Size of Tile / Size of image) 
                    custom_size: Some(Vec2::new(GAME_WINDOW_WIDTH,GAME_WINDOW_HEIGHT)),
                    ..default()
                },
                Transform::from_xyz(0., 0., -0.1)
            )
        ]
    ));
    // hold board
    commands.spawn((
        BorderPart,
        Sprite {
            image: asset_server.load("Border.png"),
            //custom_size: Some(Vec2::new((GAME_WINDOW_WIDTH + BORDER_PADDING)/2.,(GAME_WINDOW_HEIGHT + BORDER_PADDING - 0.2)/2.)),
            custom_size: Some(Vec2::new(HOLD_WINDOW_WIDTH + BORDER_PADDING /*- 3.4*/,HOLD_WINDOW_HEIGHT + BORDER_PADDING /*  - 15.*/),),
            image_mode: SpriteImageMode::Sliced(
                TextureSlicer { 
                    border: BorderRect::all(BORDER_PADDING),  // 2050x3970
                    center_scale_mode: SliceScaleMode::Tile {stretch_value: 1.2}, 
                    sides_scale_mode: SliceScaleMode::Tile { stretch_value: 1.5 },
                    max_corner_scale: 1.2,
                    ..default() 
                },
            ),
            ..default()
        },
        Transform::from_xyz(-GAME_WINDOW_WIDTH / 2. - HOLD_WINDOW_WIDTH / 2. - 2. * BORDER_PADDING, GAME_WINDOW_HEIGHT / 2. - 5. * BORDER_PADDING, 1.0),
        children![
            (
                //Visibility::Hidden, 
                GridPart,
                Sprite {
                    image: asset_server.load("Grid.png"),
                    image_mode: SpriteImageMode::Tiled { tile_x: true, tile_y: true, stretch_value: ((32.4)/(192.))}, // = (Size of Tile / Size of image) 
                    custom_size: Some(Vec2::new(HOLD_WINDOW_WIDTH, HOLD_WINDOW_HEIGHT)),
                    ..default()
                },
                Transform::from_xyz(0., 0., -0.1)
            )
        ]
    ));

}


fn draw_board(mut commands: Commands, asset_server: Res<AssetServer>, board_resource: Res<BoardData>)
{
    for (i, row) in board_resource.data.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            if let Ok(tetromino_piece) = TetrominoPiece::try_from(val){
                if tetromino_piece != TetrominoPiece::None
                {
                    let (corrected_x, corrected_y) = position_correction(i as f32, j as f32);
                    let color_index = Tetromino::get_color(tetromino_piece);
                    
                    if color_index > 10 {break;}

                    let color = Tetromino::COLOR_LIST[color_index];
                    
                    commands.spawn(
                        (
                            Sprite{
                                image: asset_server.load(color),
                                custom_size: Some(Vec2::new(Tetromino::TILE_SIZE, Tetromino::TILE_SIZE)),
                                .. default()
                            },
                            Transform::from_xyz(corrected_x, corrected_y, 1.0),
                        )
                    );

                }
            }
        }
    }
}

/*
    I want to go through a for loop that looks through the grid and
    draws the corresponding tetris depeding on what identifier it is 
    // identifier is all set just need to set up the for loop // 
    We could have the grid be a tetromino piece grid. Where there might be
    a dictionary with tetromino piece corresponding to a color defined in 
    the tetromino struct 
*/
fn position_correction(x: f32, y: f32) -> (f32, f32){
    let corrected_x: f32 = x - 5.; // (-5 -> 4) left to right
    let corrected_y: f32 = -y + 9.; //(9 -> -10) to[ to bottom ]

    (corrected_x * 32.4 + 32.4 / 2., corrected_y * 32.4 + 32.4 / 2.)
}