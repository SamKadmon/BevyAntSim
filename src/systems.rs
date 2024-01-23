include!("events.rs");
include!("functions.rs");

const BOUNDARY: f32 = 500.;

fn setup(
    mut commands: Commands,
) 
{
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 0., 30.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
        
}

fn spawn_bound(mut commands: Commands, asset_server: Res<AssetServer>)
{
    let texture = asset_server.load("Bound.png");
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(-2. * BOUNDARY, 2. * BOUNDARY)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.,0.,0.),
                ..default()},
            texture,
            ..default()
        },
        Target{},
    ));
}

fn get_mouse_input(
    buttons: Res<Input<MouseButton>>,
    mut scroll: EventReader<bevy::input::mouse::MouseWheel>,
    windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
)
{
    let mut button_state: i32 = 0;
    let mut button_string: String = String::from("[");

    use bevy::input::mouse::MouseScrollUnit;
    for ev in scroll.read() {
        match ev.unit {
            MouseScrollUnit::Line => {
                println!("Scroll (line units): vertical: {}, horizontal: {}", ev.y, ev.x);
            }
            MouseScrollUnit::Pixel => {
                println!("Scroll (pixel units): vertical: {}, horizontal: {}", (ev.y * 2.).floor() - 1., (ev.x * 2.).floor() - 1.);
            }
        }
    }


    if let Some(position) = windows.single().cursor_position() 
    {
        if buttons.just_pressed(MouseButton::Left) { button_state += 1;}
        if buttons.pressed(MouseButton::Left) { button_state += 2}
        if buttons.just_released(MouseButton::Left) { button_state += 4}

        if buttons.just_pressed(MouseButton::Right) { button_state += 8}
        if buttons.pressed(MouseButton::Right) { button_state += 16}
        if buttons.just_released(MouseButton::Right) { button_state += 32}
        
        if buttons.just_pressed(MouseButton::Middle) { button_state += 64}
        if buttons.pressed(MouseButton::Middle) { button_state += 128}
        if buttons.just_released(MouseButton::Middle) { button_state += 256}
        // Decoding example
        println!("{}",button_state);
        if button_state - 256 >= 0 {button_state -= 256; button_string.push_str("MiddleReleased ");}
        if button_state - 128 >= 0 {button_state -= 128; button_string.push_str("MiddleHeld ");}
        if button_state - 64 >= 0 {button_state -= 64; button_string.push_str("MiddlePressed ");}
        if button_state - 32 >= 0 {button_state -= 32; button_string.push_str("RightReleased ");}
        if button_state - 16 >= 0 {button_state -= 16; button_string.push_str("RightHeld ");}
        if button_state - 8 >= 0 {button_state -= 8; button_string.push_str("RightPressed ");}
        if button_state - 4 >= 0 {button_state -= 4; button_string.push_str("LeftReleased ");}
        if button_state - 2 >= 0 {button_state -= 2; button_string.push_str("LeftHeld ");}
        if button_state - 1 >= 0 {button_state -= 1; button_string.push_str("LeftPressed ");}
        if button_string.ends_with(" ") {button_string.pop();}
        button_string.push(']');
        println!("{}\n{:#}",button_string,position);
    }
}

fn spawn_target(
    mut commands: Commands, 
    asset_server: Res<AssetServer> )
{
    let texture = asset_server.load("Target.png");
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(-25.0, 25.0)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(-400.,200.,0.),
                ..default()},
            texture,
            ..default()
        },
        Target{},
    ));
}

fn spawn_hive(mut commands: Commands, asset_server: Res<AssetServer>)
{
    let texture = asset_server.load("Hive.png");

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(75.0, 75.0)),
                ..default()
            },
            texture,
            ..default()
        },
        Hive::default()
    ));
}

fn camera_movement 
(
    mut cameras: Query<(&mut Transform, &Camera)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) 
{
    for (mut transform, _) in &mut cameras {
        let move_amount = 10.0 * time.delta_seconds();
        let mut move_vec: Vec3 = Vec3::new(0.,0.,0.);
        
        if input.pressed(KeyCode::W) {
            move_vec.y += 1.;
        }
        if input.pressed(KeyCode::A) {
            move_vec.x -= 1.;
        }
        if input.pressed(KeyCode::S) {
            move_vec.y -= 1.;
        }
        if input.pressed(KeyCode::D) {
            move_vec.x += 1.;
        }

        move_vec = move_vec.normalize_or_zero();

        transform.translation += move_vec * move_amount;
    }
}


// Spawns Ants
// Receives Food from Ants
// Checks if at Ant Cap


fn hive_mind(
    //mut commands: Commands,
    mut hives: Query<&mut Hive>,
    mut ev_spawnant: EventWriter<SpawnAntEvent>,
    time: Res<Time>
) {
    for mut hive in hives.iter_mut()
    {
        hive.spawntimer.tick(std::time::Duration::from_secs_f32(time.delta_seconds()));
        if hive.ants.0 < hive.ants.1 && hive.food >= 10 && hive.spawntimer.finished()
        { 
            ev_spawnant.send(SpawnAntEvent(hive.faction.id)); 
        }
    }
}


fn ant_movement (
    mut ants: Query<(&mut Transform, &mut Ant)>,
    time: Res<Time>,
) {

    for (mut atransform, mut ant) in ants.iter_mut() {
        
        let desired_velocity: Vec3 = ant.desired_direction * ant.max_speed * time.delta_seconds();
        let desired_steering_force: Vec3 = (desired_velocity - ant.velocity) * ant.steer_strength;
        ant.acceleration = desired_steering_force.clamp_length(0., ant.steer_strength);
        ant.velocity = (ant.velocity + ant.acceleration * time.delta_seconds()).clamp_length(0., ant.max_speed);
        //Circular Boundary
        if 
        (atransform.translation.x + ant.velocity.x).powf(2.0) + 
        (atransform.translation.y + ant.velocity.y).powf(2.0) > BOUNDARY.powf(2.0)
        {
            ant.velocity *= -1.;
            ant.desired_direction *= -1.;
        }
        atransform.translation += ant.velocity;

        ant.desired_direction = (ant.desired_direction + Vec3::new(random_number(-1.,1.),random_number(-1.,1.),0.) * ant.wanderlust).normalize_or_zero();
    }
}

fn spawn_ant(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut ev_spawnant: EventReader<SpawnAntEvent>,
    mut hives: Query<&mut Hive>
) {
    for spawn_event in ev_spawnant.read()
    {
        commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb_u8(124, 144, 255).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Ant { 
            faction: Faction{id: spawn_event.0}, //faction.id,
            ..default()
        },
    
    ));
        //Increment Ant Count in Hive AND Decrement Hive food
        for mut hive in &mut hives
        {
            hive.ants.0 += 1;
            hive.food -= 10;
        }


    }
}