use bevy::{camera::ScalingMode, prelude::*, window::WindowResolution}; 

use rust_game::*;








fn setup(mut commands: Commands)
{
    commands.spawn((Camera2d,
                           Projection::Orthographic(OrthographicProjection {
                                scaling_mode : ScalingMode::AutoMax { max_width: WINDOW_WIDTH, max_height: WINDOW_HEIGHT },
                                ..OrthographicProjection::default_2d()})
                            )
                    );
    
    
    commands.spawn((
        Visibility::Hidden,  
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
   // } TO RUN ON WSL DO THE FOLLOWING: unset WAYLAND_DISPLAY && cargo run

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
