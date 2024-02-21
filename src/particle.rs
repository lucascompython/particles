use crate::raylib::{self, Vector2};

#[derive(Copy, Clone)]
pub struct Particle {
    pub position: raylib::Vector2,
    pub velocity: raylib::Vector2,
    pub color: raylib::Color,
}

impl Particle {
    pub fn new(width: i32, height: i32) -> Self {
        unsafe {
            Self {
                position: raylib::Vector2 {
                    x: raylib::GetRandomValue(0, width - 1) as f32,
                    y: raylib::GetRandomValue(0, height - 1) as f32,
                },
                velocity: raylib::Vector2 {
                    x: (raylib::GetRandomValue(-200, 200) / 100) as f32,
                    y: (raylib::GetRandomValue(-200, 200) / 100) as f32,
                },
                color: raylib::Color {
                    r: 0,
                    g: 0,
                    b: 0,
                    a: 100,
                },
            }

        }
    }

    pub fn get_distance_to(&self, other: Vector2) -> f32 {
        let x = self.position.x - other.x;
        let y = self.position.y - other.y;
        let distance = (x * x) + (y * y);
        distance.sqrt()
    }

    pub fn get_normalized_direction_to(&self, other: Vector2) -> raylib::Vector2 {
        let mut direction = raylib::Vector2 { x: 0.0, y: 0.0 };
        let distance = self.get_distance_to(other);
        if distance > 0.0 {
            direction.x = (other.x - self.position.x) / distance;
            direction.y = (other.y - self.position.y) / distance;
        }

        direction
    }

    pub fn attract(&mut self, other: Vector2, multiplier: f32, attract: bool) {
        let distance = f32::max(self.get_distance_to(other), 0.5);
        let normal = self.get_normalized_direction_to(other);

        if attract {
            self.velocity.x += normal.x / distance * multiplier;
            self.velocity.y += normal.y / distance * multiplier;
        } else {
            self.velocity.x -= normal.x / distance * multiplier;
            self.velocity.y -= normal.y / distance * multiplier;
        }
    }

    pub fn do_friction(&mut self, value: f32) {
        self.velocity.x *= value;
        self.velocity.y *= value;
    }

    pub fn move_particle(&mut self, width: f32, height: f32, border: bool) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;

        if self.position.x < 0.0 {
            if border {
                self.position.x = 0.0;
                self.velocity.x *= -1.0;
            } else {
                self.position.x += width;
            }
        } else if self.position.x > width {
            if border {
                self.position.x = width;
                self.velocity.x *= -1.0;
            } else {
                self.position.x -= width;
            }
        }

        if self.position.y < 0.0 {
            if border {
                self.position.y = 0.0;
                self.velocity.y *= -1.0;
            } else {
                self.position.y += height;
            }
        } else if self.position.y > height {
            if border {
                self.position.y = height;
                self.velocity.y *= -1.0;
            } else {
                self.position.y -= height;
            }
        }
    }

    pub fn draw(&self, balls: bool, size: f32) {
        unsafe {
            if balls {
                raylib::DrawCircleV(self.position, size, self.color);
            } else {
                raylib::DrawPixelV(self.position, self.color);
            }
        }
    }
}
