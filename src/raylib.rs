// raylib functions from the header
extern "C" {
    pub fn InitWindow(width: i32, height: i32, title: *const i8);
    pub fn CloseWindow();
    pub fn BeginDrawing();
    pub fn ClearBackground(color: Color);
    pub fn EndDrawing();
    pub fn SetTargetFPS(fps: i32);
    pub fn SetRandomSeed(seed: u32);
    pub fn GetRandomValue(min: i32, max: i32) -> i32;
    pub fn DrawPixelV(position: Vector2, color: Color);
    pub fn GetMousePosition() -> Vector2;
    pub fn WindowShouldClose() -> bool;
    pub fn DrawFPS(x: i32, y: i32);
    pub fn IsKeyPressed(key: i32) -> bool;
    pub fn DrawCircleV(center: Vector2, radius: f32, color: Color);
    pub fn SetWindowState(state: i32);
    pub fn GetScreenWidth() -> i32;
    pub fn GetScreenHeight() -> i32;
    pub fn DrawText(text: *const i8, x: i32, y: i32, font_size: i32, color: Color);
    pub fn IsKeyDown(key: i32) -> bool;
    pub fn IsMouseButtonPressed(button: i32) -> bool;

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}
pub enum KeyboardKey {
    KeyB = 66,      // Key: B | b
    KeyR = 82,      // Key: R | r
    KeySpace = 32,  // Key: Space
    KeyUp = 265,    // Key: Up
    KeyDown = 264,  // Key: Down
    KeyLeft = 263,  // Key: Left
    KeyRight = 262, // Key: Right
    KeyK = 75,      // Key: K | k
    KeyJ = 74,      // Key: J | j
}
