use macroquad::prelude::*;

const BOID_COUNT: usize = 150;
const MAX_SPEED: f32 = 4.0;
const MAX_FORCE: f32 = 0.15;
const PERSONAL_SPACE: f32 = 25.0;

struct Boid {
    position: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
}

impl Boid {
    fn new(x: f32, y: f32) -> Self {
        // Spawned with random vector
        let angle = rand::gen_range(0.0, std::f32::consts::TAU);
        let velocity = Vec2::new(angle.cos(), angle.sin()) * rand::gen_range(1.0, MAX_SPEED);
        
        Self {
            position: Vec2::new(x, y),
            velocity,
	    acceleration: Vec2::ZERO,
        }
    }

    fn update(&mut self) {
	self.velocity += self.acceleration;
	self.velocity = self.velocity.clamp_length_max(MAX_SPEED);
        self.position += self.velocity;
	self.acceleration = Vec2::ZERO;

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

        let current_positions: Vec<Vec2> = flock.iter().map(|b| b.position).collect();
        
	for boid in flock.iter_mut() {
            let mut steering_force = Vec2{x: 0.0, y: 0.0};
            let mut neighbor_count = 0;

            for &other_pos in current_positions.iter() {
                if boid.position == other_pos {
                    continue;
                }

                let distance = boid.position.distance(other_pos);

                if distance < PERSONAL_SPACE && distance > 0.0 {
                    let mut away_vector = boid.position - other_pos;

                    // Closer boids push harder?
                    away_vector = away_vector.normalize() / distance;

                    steering_force += away_vector;
                    neighbor_count += 1;
                }
            }

            if neighbor_count > 0 {
		steering_force /= neighbor_count as f32;

		if steering_force.length_squared() > 0.0 {
		    let desired_velocity = steering_force.normalize() * MAX_SPEED;
		    let steer = desired_velocity - boid.velocity;
		    steering_force = steer.clamp_length_max(MAX_FORCE);
		}
	    } else {
		steering_force = Vec2::ZERO;
            }

            boid.acceleration += steering_force;

            boid.update();
            boid.draw();
        }

        next_frame().await
    }
}
