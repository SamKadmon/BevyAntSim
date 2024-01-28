#[derive(Component)]
pub struct Anchor {} //Camera Marker

#[derive(Component)]
pub struct Player {
    pub speed: f32, // Movement Speed
}

#[derive(Component)]
pub struct Ant {
    pub faction: Faction,
    pub max_speed: f32, // Movement Speed
    pub steer_strength: f32,
    pub wanderlust: f32,
    pub velocity: Vec3,
    pub acceleration: Vec3,
    pub desired_direction: Vec3
}

impl Default for Ant {
    fn default() -> Ant {
        Ant {
            faction: Faction{id:0},
            max_speed: 4.0,
            steer_strength: 2.0,
            wanderlust: 0.5,
            velocity: Vec3::new(0.,0.,0.),
            acceleration: Vec3::new(0.,0.,0.),
            desired_direction: Vec3::new(0.,0.,0.), 
        }
    }
}


#[derive(Component)]
pub struct Target {} // Marker, no need for elements

#[derive(Component)]
pub struct Faction {id: i32}

#[derive(Component)]
pub struct Hive {
    pub faction: Faction,
    pub ants: (i32, i32), // (spawned, spawnlimit)
    pub spawntimer: Timer, // Ideally a low number
    pub food: i32,
}

impl Default for Hive {
    fn default() -> Hive {
        Hive {
            faction: Faction{id:0},
            ants: (0, 100),
            spawntimer:Timer::from_seconds(0.1, TimerMode::Repeating),
            food: 500,
        }
    }
}