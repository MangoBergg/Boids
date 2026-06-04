use macroquad::prelude::*;

const BOID_COUNT: usize = 150;
const MAX_SPEED: f32 = 4.0;

struct Boid {
    position: Vec2,
    velocity: Vec2,
}

impl Boid {
    fn new(x: f32, y: f32) -> Self {
        // Spawned with random vector
        let angle = rand::gen_range(0.0, std::f32::consts::TAU);
        let velocity = Vec2::new(angle.cos(), angle.sin()) * rand::gen_range(1.0, MAX_SPEED);
        
        Self {
            position: Vec2::new(x, y),
            velocity,
        }
    }

    fn update(&mut self) {
        self.position += self.velocity;

        // Screen wrapping
        if self.position.x < 0.0 { self.position.x = screen_width(); }
        if self.position.x > screen_width() { self.position.x = 0.0; }
        if self.position.y < 0.0 { self.position.y = screen_height(); }
        if self.position.y > screen_height() { self.position.y = 0.0; }
    }

    fn draw(&self) {
        // The boid is a circle xd
        draw_circle(self.position.x, self.position.y, 4.0, SKYBLUE);
    }
}

#[macroquad::main("Boids Simulation")]
async fn main() {
    rand::srand(macroquad::miniquad::date::now() as u64);

    // Population
    let mut flock: Vec<Boid> = (0..BOID_COUNT)
        .map(|_| Boid::new(rand::gen_range(0.0, screen_width()), rand::gen_range(0.0, screen_height())))
        .collect();

    loop {
        clear_background(Color::new(0.08, 0.09, 0.1, 1.0));

        for boid in flock.iter_mut() {
            boid.update();
            boid.draw();
        }

        next_frame().await
    }
}
