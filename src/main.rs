#![windows_subsystem = "windows"]
mod particle;
mod raylib;

use clap::Parser;
use std::ffi::CString;

#[derive(Parser)]
#[clap(
    version = "1.1",
    author = "Lucas Linhares",
    about = r#"Raylib Particle Attraction/Repulsion
Keybindings:
    R: Reset Particles
    Space/M1 Click: Toggle Attraction/Repulsion
    B: Toggle Screen Border Collisions
    Arrow Keys (Up/Down/Left/Right): Add/Remove 1/1/100/100 Particles
    Mouse: Attract/Repel Particles
    ESC: Quit"#
)]
struct Cli {
    /// Number of particles
    #[arg(short, long)]
    particles_num: Option<usize>,

    /// Velocity Multiplier
    #[arg(short, long)]
    multiplier: Option<f32>,

    /// Friction Multiplier
    #[arg(short, long)]
    friction: Option<f32>,

    /// Screen Width
    #[arg(short, long)]
    width: Option<i32>,

    /// Screen Height
    #[arg(long)]
    height: Option<i32>,

    /// If set, particles will be drawn as circles
    #[arg(short, long)]
    balls: bool,

    /// Size of the balls
    #[arg(long, requires_all=["balls"])]
    ball_size: Option<f32>,

    /// If set, the FPS counter will not be drawn
    #[arg(long)]
    no_fps: bool,

    /// If set, the Particle count will not be drawn
    #[arg(long)]
    no_particle_count: bool,

    /// If set, the screen border collisions label will not be drawn
    #[arg(long)]
    no_border: bool,

    /// If set, the attract/repel label will not be drawn
    #[arg(long)]
    no_attract: bool,

    /// If set, no labels will be drawn.
    /// Equivalent to setting all the no_* flags
    #[arg(long)]
    no_labels: bool,
}

fn main() {
    let mut args = Cli::parse();

    let mut particle_count = if let Some(count) = args.particles_num {
        count
    } else {
        100_000
    };

    let multiplier = if let Some(mult) = args.multiplier {
        mult
    } else {
        1.0
    };

    let friction = if let Some(fric) = args.friction {
        fric
    } else {
        0.99
    };

    let ball_size = if let Some(ball_size) = args.ball_size {
        ball_size
    } else {
        20.0
    };

    if args.no_labels {
        args.no_fps = true;
        args.no_particle_count = true;
        args.no_border = true;
        args.no_attract = true;
    }

    let mut width = if let Some(w) = args.width { w } else { 800 };
    let mut height = if let Some(h) = args.height { h } else { 800 };
    unsafe {
        raylib::SetRandomSeed(69);
        // set window to resizable
        raylib::SetWindowState(4);

        let mut particles: Vec<particle::Particle> = (0..particle_count)
            .map(|_| particle::Particle::new(width, height))
            .collect();

        let mut attract = true;
        let mut border = true;

        let window_title = CString::new("Particles").unwrap();
        raylib::InitWindow(width, height, window_title.as_ptr());
        raylib::SetTargetFPS(60);

        while !raylib::WindowShouldClose() {
            width = raylib::GetScreenWidth();
            height = raylib::GetScreenHeight();

            if raylib::IsKeyPressed(raylib::KeyboardKey::KeyR as i32) {
                particles = (0..particle_count)
                    .map(|_| particle::Particle::new(width, height))
                    .collect();
            }

            if raylib::IsKeyPressed(raylib::KeyboardKey::KeySpace as i32)
                || raylib::IsMouseButtonPressed(0)
            {
                attract = !attract;
            }

            if raylib::IsKeyPressed(raylib::KeyboardKey::KeyB as i32) {
                border = !border;
            }

            if raylib::IsKeyDown(raylib::KeyboardKey::KeyUp as i32) {
                particles.push(particle::Particle::new(width, height));

                particle_count += 1;
            } else if raylib::IsKeyDown(raylib::KeyboardKey::KeyDown as i32) {
                if particle_count >= 1 {
                    particles.pop();
                    particle_count -= 1;
                }
            } else if raylib::IsKeyDown(raylib::KeyboardKey::KeyLeft as i32) {
                if particle_count >= 100 {
                    for _ in 0..100 {
                        particles.pop();
                    }

                    particle_count -= 100;
                }
            } else if raylib::IsKeyDown(raylib::KeyboardKey::KeyRight as i32) {
                for _ in 0..100 {
                    particles.push(particle::Particle::new(width, height));
                }
                particle_count += 100;
            }

            let mouse_position = raylib::GetMousePosition();

            for particle in particles.iter_mut() {
                particle.attract(mouse_position, multiplier, attract);
                particle.do_friction(friction);
                particle.move_particle(width as f32, height as f32, border);
                particle.color = raylib::Color {
                    r: (particle.velocity.x.abs() * 255.0) as u8,
                    g: (particle.velocity.y.abs() * 255.0) as u8,
                    b: 0,
                    a: 255,
                };
            }

            raylib::BeginDrawing();
            raylib::ClearBackground(raylib::Color {
                r: 0,
                g: 0,
                b: 0,
                a: 255,
            });

            for particle in particles.iter() {
                particle.draw(args.balls, ball_size);
            }

            if !args.no_fps {
                raylib::DrawFPS(10, 10);
            }

            if !args.no_particle_count {
                let text = CString::new(format!("Particles: {}", particle_count)).unwrap();
                let text_ptr = text.as_ptr();
                raylib::DrawText(
                    text_ptr,
                    10,
                    30,
                    20,
                    raylib::Color {
                        r: 255,
                        g: 255,
                        b: 255,
                        a: 255,
                    },
                );
            }

            if !args.no_border {
                let text = CString::new(format!("Border Collisions: {}", border)).unwrap();
                let text_ptr = text.as_ptr();
                raylib::DrawText(
                    text_ptr,
                    10,
                    50,
                    20,
                    raylib::Color {
                        r: 255,
                        g: 255,
                        b: 255,
                        a: 255,
                    },
                );
            }

            if !args.no_attract {
                let text = CString::new(format!("Attract: {}", attract)).unwrap();
                let text_ptr = text.as_ptr();
                raylib::DrawText(
                    text_ptr,
                    10,
                    70,
                    20,
                    raylib::Color {
                        r: 255,
                        g: 255,
                        b: 255,
                        a: 255,
                    },
                );
            }

            raylib::EndDrawing();
        }
        raylib::CloseWindow();
    }
}
