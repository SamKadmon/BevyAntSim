include!("events.rs");
include!("functions.rs");

const BOUNDARY: f32 = 10.;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    commands.spawn((
        PbrBundle {
            mesh: meshes
                .add(Mesh::from(shape::Cube { size: 0.5 })),
            material: materials
                .add(Color::rgb_u8(255, 255, 168).into()),
            transform: Transform::from_xyz(5.0, 5.0, 1.0),
            ..default()
        },
        Anchor{}
    )).with_children(|parent| {
        // child camera
        parent.spawn(
            Camera3dBundle {
                transform: Transform::from_xyz(0., 0., 30.).looking_at(Vec3::ZERO, Vec3::Y),
                ..default()
        }); 
    });
}
//ground is on the left
fn spawn_bound(
    mut commands: Commands, 
    asset_server: Res<AssetServer>
) {
    let texture = asset_server.load("Bound.png");
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(-2. * BOUNDARY, 2. * BOUNDARY)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0., 0., 0.),
                ..default()
            },
            texture,
            ..default()
        },
        Target {},
    ));
}

fn get_mouse_input(
    buttons: Res<Input<MouseButton>>,
    mut scroll: EventReader<bevy::input::mouse::MouseWheel>,
    mut motion: EventReader<bevy::input::mouse::MouseMotion>,
    windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
    mut mouse_event_writer: EventWriter<MouseEvent>,
) {
    let mut button_state: i32 = 0;
    use bevy::input::mouse::MouseScrollUnit;
    if let Some(position) = windows.single().cursor_position() {
        let mut actual_scroll_state: f32 = 0.;
        for ev in scroll.read() {
            if ev.unit == MouseScrollUnit::Pixel {
                actual_scroll_state = (ev.y * 2.).floor() - 1.;
            } else {
                actual_scroll_state = ev.y;
            }
        }
        if buttons.just_pressed(MouseButton::Left) {
            button_state += 1;
        }
        if buttons.pressed(MouseButton::Left) {
            button_state += 2
        }
        if buttons.just_released(MouseButton::Left) {
            button_state += 4
        }

        if buttons.just_pressed(MouseButton::Right) {
            button_state += 8
        }
        if buttons.pressed(MouseButton::Right) {
            button_state += 16
        }
        if buttons.just_released(MouseButton::Right) {
            button_state += 32
        }
    
        if buttons.just_pressed(MouseButton::Middle) {
            button_state += 256
        }
        if buttons.pressed(MouseButton::Middle) {
            button_state += 512
        }
        if buttons.just_released(MouseButton::Middle) {
            button_state += 1024
        }
    
        if actual_scroll_state > 0. {
            button_state += 2048
        }
        if actual_scroll_state < 0. {
            button_state += 4096
        }
        let mut movement = Vec2::ZERO;
        for m in motion.read() {
             movement = Vec2::new(m.delta.x,m.delta.y);
        }
        mouse_event_writer.send(MouseEvent(button_state, position, movement));
    }
}

/* // Decoding example
        println!("{}",button_state);
        if button_state - 1024 >= 0 {button_state -= 1024; button_string.push_str("MiddleReleased ");}
        if button_state - 512 >= 0 {button_state -= 512; button_string.push_str("MiddleHeld ");}
        if button_state - 256 >= 0 {button_state -= 256; button_string.push_str("MiddlePressed ");}
        if button_state - 128 >= 0 {button_state -= 128; button_string.push_str("ScrollDown ");}
        if button_state - 64 >= 0 {button_state -= 64; button_string.push_str("ScrollUp ");}
        if button_state - 32 >= 0 {button_state -= 32; button_string.push_str("RightReleased ");}
        if button_state - 16 >= 0 {button_state -= 16; button_string.push_str("RightHeld ");}
        if button_state - 8 >= 0 {button_state -= 8; button_string.push_str("RightPressed ");}
        if button_state - 4 >= 0 {button_state -= 4; button_string.push_str("LeftReleased ");}
        if button_state - 2 >= 0 {button_state -= 2; button_string.push_str("LeftHeld ");}
        if button_state - 1 >= 0 {button_state -= 1; button_string.push_str("LeftPressed ");}
        if button_string.ends_with(" ") {button_string.pop();}
        button_string.push(']');
        println!("{}\n{:#}",button_string,position);
*/

fn _decode_mouse_input() {}
// And then there was soil
fn spawn_ground(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Circle::new(BOUNDARY + 1.0).into()),
        material: materials.add(Color::WHITE.into()),
        transform: Transform::from_rotation(Quat::from_rotation_z(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });

}
// Let there be light
fn spawn_light(
    mut commands: Commands, 
)
{
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 9000.0,
            range: 100.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(BOUNDARY, BOUNDARY, BOUNDARY * 2.),
        ..default()
    });
}
// And man strived for purpose
fn spawn_target(
    mut commands: Commands, 
    asset_server: Res<AssetServer>
) {
    let texture = asset_server.load("Target.png");
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(-25.0, 25.0)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(-400., 200., 0.),
                ..default()
            },
            texture,
            ..default()
        },
        Target {},
    ));
}
//A home and hearth to one day return to
fn spawn_hive(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    commands.spawn((
        PbrBundle {
            mesh: meshes
                .add(Mesh::from(shape::Cube { size: 1.5 })),
            material: materials
                .add(Color::rgb_u8(168, 27, 13).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.75),
            ..default()
        },
        Hive::default()
    )).with_children(|parent| {
        // child cube
        parent.spawn(PbrBundle {
            mesh: meshes
                .add(Mesh::from(shape::Cube { size: 1. })),
            material: materials
            .add(Color::rgb_u8(168, 27, 13).into()),
            transform: Transform::from_xyz(0.0, 0.0, 1.25),
            ..default()
        }); 
    });
}
//And God watched his creation
fn camera_movement(
    mut anchors: Query<(&mut Transform, With<Anchor>, Without<Camera>)>,
    mut cameras: Query<(&mut Transform, With<Camera>, Without<Anchor>)>,
    input: Res<Input<KeyCode>>,
    mut mouse_reader: EventReader<MouseEvent>,
    time: Res<Time>,
) {
    let move_amount = 10.0 * time.delta_seconds();
    
    for (mut anchform, _, _) in &mut anchors {
        for (mut camform, _, _) in &mut cameras {
        let mut move_vec: Vec3 = Vec3::ZERO;

        if input.pressed(KeyCode::W) {
            move_vec += camform.forward();
            move_vec.z = 0.;
            if move_vec == Vec3::ZERO
            {
                move_vec += Vec3::Y;
            } 
        }
        if input.pressed(KeyCode::A) {
            move_vec += camform.left();
            move_vec.z = 0.;
        }
        if input.pressed(KeyCode::S) {
            move_vec += camform.back();
            move_vec.z = 0.;
            if move_vec == Vec3::ZERO
            {
                move_vec -= Vec3::Y;
            } 
        }
        if input.pressed(KeyCode::D) {
            move_vec += camform.right();
            move_vec.z = 0.;
        }
        move_vec = move_vec.normalize_or_zero();
        anchform.translation += move_vec * move_amount;

        //CAMERA ROTATION SECTION (So i dont lose it again) (AGAIN)
        move_vec = Vec3::ZERO;
        let camform2 = camform.clone();
        for mouse in mouse_reader.read() {
                let mut mousestate = mouse.0;
                //Shift = Up and down around the hemisphere
                //Ctrl = Left and right around the hemisphere
                if input.pressed(KeyCode::ShiftLeft)
                {
                    if mousestate - 4096 >= 0 && camform.rotation.x > 0. //TO FIX
                    {
                        mousestate -= 4096;
                        camform.rotate_around(Vec3::ZERO, Quat::from_axis_angle(camform2.left(), 4. * time.delta_seconds()) );
                    }
                    if mousestate - 2048 >= 0 && camform.rotation.x < 0.625 //TO FIX
                    {
                        mousestate -= 2048;
                        camform.rotate_around(Vec3::ZERO, Quat::from_axis_angle(camform2.right(), 4. * time.delta_seconds()) );
                    }
                }
            
                if input.pressed(KeyCode::ControlLeft)
                {
                    if mousestate - 4096 >= 0 
                    {
                        mousestate -= 4096;
                        camform.rotate_around(Vec3::ZERO, Quat::from_rotation_z( -4. * time.delta_seconds() ));
                    }
                    if mousestate - 2048 >= 0
                    {
                        mousestate -= 2048;
                        camform.rotate_around(Vec3::ZERO, Quat::from_rotation_z( 4. * time.delta_seconds() ));
                    }
                }
                else 
                {
                    if mousestate - 4096 >= 0 && camform.translation.distance(anchform.translation) < 100.
                    //ZOOM IN
                    {
                        mousestate -= 4096;
                        move_vec += camform.back() * 16.0;
                    } else if mousestate - 2048 >= 0 && camform.translation.distance(anchform.translation) > 10.0
                    //ZOOM OUT
                    {
                        mousestate -= 2048;
                        move_vec += camform.forward() * 16.0;
                    }  
                }
            }
            //move_vec = move_vec.normalize_or_zero();
            camform.translation += move_vec * move_amount;

    }
    }


}

// Spawns Ants
// Receives Food from Ants
// Checks if at Ant Cap

// A community must ant in the community's best intereset
fn hive_mind(
    //mut commands: Commands,
    mut hives: Query<&mut Hive>,
    mut ev_spawnant: EventWriter<SpawnAntEvent>,
    time: Res<Time>,
) {
    for mut hive in hives.iter_mut() {
        hive.spawntimer
            .tick(std::time::Duration::from_secs_f32(time.delta_seconds()));
        if hive.ants.0 < hive.ants.1 && hive.food >= 10 && hive.spawntimer.finished() {
            ev_spawnant.send(SpawnAntEvent(hive.faction.id));
        }
    }
}
// Antkind gained autonomy that day 
fn ant_movement(mut ants: Query<(&mut Transform, &mut Ant)>, time: Res<Time>) {
    for (mut atransform, mut ant) in ants.iter_mut() {
        let desired_velocity: Vec3 = ant.desired_direction * ant.max_speed * time.delta_seconds();
        let desired_steering_force: Vec3 = (desired_velocity - ant.velocity) * ant.steer_strength;
        ant.acceleration = desired_steering_force.clamp_length(0., ant.steer_strength);
        ant.velocity = (ant.velocity + ant.acceleration * time.delta_seconds())
            .clamp_length(0., ant.max_speed);
        //Circular Boundary
        if (atransform.translation.x + ant.velocity.x).powf(2.0)
            + (atransform.translation.y + ant.velocity.y).powf(2.0)
            > BOUNDARY.powf(2.0)
        {
            ant.velocity *= -1.;
            ant.desired_direction *= -1.;
        }
        atransform.translation += ant.velocity;

        ant.desired_direction = (ant.desired_direction
            + Vec3::new(random_number(-1., 1.), random_number(-1., 1.), 0.) * ant.wanderlust)
            .normalize_or_zero();
    }
}
//
fn spawn_ant(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut ev_spawnant: EventReader<SpawnAntEvent>,
    mut hives: Query<&mut Hive>,
) {
    for spawn_event in ev_spawnant.read() {
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                material: materials.add(Color::rgb_u8(124, 144, 255).into()),
                transform: Transform::from_xyz(0.0, 0.0, 0.5),
                ..default()
            },
            Ant {
                faction: Faction { id: spawn_event.0 }, //faction.id,
                ..default()
            },
        ));
        //Increment Ant Count in Hive AND Decrement Hive food
        for mut hive in &mut hives {
            hive.ants.0 += 1;
            hive.food -= 10;
        }
    }
}
