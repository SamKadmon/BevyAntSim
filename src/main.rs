include!("components.rs");
include!("systems.rs");

use::bevy::prelude::*;

fn main() 
{
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Anthive Simulator".into(),
                        resolution: (1600.0, 900.0).into(),
                        resizable:true,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_event::<SpawnAntEvent>()
        .add_event::<MouseEvent>()
        .add_systems(Startup, (setup, spawn_light, spawn_ground, spawn_hive, spawn_target,))
        .add_systems(Update, (get_mouse_input,camera_movement, ant_movement, hive_mind,))
        .add_systems(Update, spawn_ant)
        .run();
}