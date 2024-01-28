#[derive(Event)]
struct SpawnAntEvent(i32);

#[derive(Event)]
struct MouseEvent(i32, Vec2, Vec2);