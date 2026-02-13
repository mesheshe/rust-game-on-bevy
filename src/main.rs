use bevy::{prelude::*, window::WindowResolution}; 

use rust_game::*;


const WINDOW_WIDTH: f32 = 1024.;
const WINDOW_HEIGHT: f32 = 720.;

//const WINDOW_BOTTOM_HEIGHT: f32 = WINDOW_HEIGHT / - 2.;
//const WINDOW_LEFT_WIDTH: f32 = WINDOW_WIDTH / - 2.;

const GAME_WINDOW_HEIGHT: f32 = 648.;
const GAME_WINDOW_WIDTH: f32 = 324.;

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>)
{
    commands.spawn(Camera2d::default());
    

    commands.spawn((
        Node{
            // left and top specify position
            left : Val::Px((WINDOW_WIDTH as f32 - GAME_WINDOW_WIDTH)/2.),
            top: Val::Px((WINDOW_HEIGHT as f32 - GAME_WINDOW_HEIGHT) / 2.),
            width: Val::Px(GAME_WINDOW_WIDTH),
            height: Val::Px(GAME_WINDOW_HEIGHT),
            border: UiRect::all(Val::Px(1.0)),
            ..default()
        },
        
        BorderColor::all(Color::WHITE),
        //BackgroundColor(Color::BLACK),

    ));
    /* 
    let mesh_handle = meshes.add(Circle::new(10.));
    let material_handle = materials.add(ColorMaterial::from_color(RED));

    commands.spawn((
        Mesh2d(mesh_handle), 
        MeshMaterial2d(material_handle),
        Transform::from_xyz(0., 0., 0.),
    ));
    */
}

fn main() {
    //if let Ok(val) = std::env::var("WAYLAND_DISPLAY")
    //unsafe {
        // wsl issue for XDG Settings Portal
       // std::env::set_var("WAYLAND_DISPLAY", "");
   // }

    App::new()
        .insert_resource(ClearColor(Color::linear_rgb(0.29, 0.31, 0.41)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window { resolution: WindowResolution::new(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32), title: "Bevy Game".to_string(), resizable: false, .. default() }),
            ..default()
        }))
        .add_plugins((
            BoardPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}
