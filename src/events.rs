#[derive(Event)]
struct SpawnAntEvent(i32);

#[derive(Event)]
struct MouseEvent(i32, i32, u32); // Mouse X, Mouse Y, Button Pressed