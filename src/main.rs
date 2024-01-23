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
        .add_systems(Startup, (setup, spawn_hive, spawn_target, spawn_bound))
        .add_systems(Update, (camera_movement, ant_movement, hive_mind, get_mouse_input))
        .add_systems(Update, spawn_ant)
        .run();
}