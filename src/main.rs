use macroquad::prelude::*;

const BOID_COUNT: usize = 175;
const BOX_SIZE: f32 = 20.0;       
const MAX_SPEED: f32 = 0.7;
const MAX_FORCE: f32 = 0.05;      
const PERSONAL_SPACE: f32 = 2.5;  
const VISUAL_RANGE: f32 = 6.0;    

const SEPARATION_WEIGHT: f32 = 1.5;
const ALIGNMENT_WEIGHT: f32 = 1.0;
const COHESION_WEIGHT: f32 = 1.0;

const WALL_MARGIN: f32 = 3.0;     
const WALL_FORCE: f32 = 0.2;      

struct Boid {
    position: Vec3,
    velocity: Vec3,
    acceleration: Vec3,
}

impl Boid {
    fn new() -> Self {
        let position = Vec3::new(
            rand::gen_range(-BOX_SIZE, BOX_SIZE),
            rand::gen_range(-BOX_SIZE, BOX_SIZE),
            rand::gen_range(-BOX_SIZE, BOX_SIZE),
        );

        let dir = Vec3::new(
            rand::gen_range(-1.0, 1.0),
            rand::gen_range(-1.0, 1.0),
            rand::gen_range(-1.0, 1.0),
        );
        let velocity = dir.normalize_or_zero() * rand::gen_range(0.2, MAX_SPEED);
        
        Self {
            position,
            velocity,
            acceleration: Vec3::ZERO,
        }
    }

    fn avoid_walls(&self) -> Vec3 {
        let mut desired_velocity = self.velocity;
        let mut near_wall = false;

        if self.position.x > BOX_SIZE - WALL_MARGIN {
            desired_velocity.x = -MAX_SPEED;
            near_wall = true;
        } else if self.position.x < -BOX_SIZE + WALL_MARGIN {
            desired_velocity.x = MAX_SPEED;
            near_wall = true;
        }

        if self.position.y > BOX_SIZE - WALL_MARGIN {
            desired_velocity.y = -MAX_SPEED;
            near_wall = true;
        } else if self.position.y < -BOX_SIZE + WALL_MARGIN {
            desired_velocity.y = MAX_SPEED;
            near_wall = true;
        }

        if self.position.z > BOX_SIZE - WALL_MARGIN {
            desired_velocity.z = -MAX_SPEED;
            near_wall = true;
        } else if self.position.z < -BOX_SIZE + WALL_MARGIN {
            desired_velocity.z = MAX_SPEED;
            near_wall = true;
        }

        if near_wall {
            let steer = desired_velocity - self.velocity;
            return steer.clamp_length_max(WALL_FORCE);
        }

        Vec3::ZERO
    }

    fn update(&mut self) {
        self.velocity += self.acceleration;
        self.velocity = self.velocity.clamp_length_max(MAX_SPEED);
        self.position += self.velocity;
        self.acceleration = Vec3::ZERO;

        if self.position.x < -BOX_SIZE { self.position.x = BOX_SIZE; }
        if self.position.x > BOX_SIZE { self.position.x = -BOX_SIZE; }
        if self.position.y < -BOX_SIZE { self.position.y = BOX_SIZE; }
        if self.position.y > BOX_SIZE { self.position.y = -BOX_SIZE; }
        if self.position.z < -BOX_SIZE { self.position.z = BOX_SIZE; }
        if self.position.z > BOX_SIZE { self.position.z = -BOX_SIZE; }
    }

    fn draw(&self) {
        draw_sphere(self.position, 0.2, None, SKYBLUE);
        
        // To draw the tail
        let tail_end = self.position - (self.velocity.normalize_or_zero() * 1.5);
        draw_line_3d(self.position, tail_end, BLUE); 
    }
}

#[macroquad::main("Boids 3D")]
async fn main() {
    rand::srand(macroquad::miniquad::date::now() as u64);

    let mut flock: Vec<Boid> = (0..BOID_COUNT).map(|_| Boid::new()).collect();

    loop {
        clear_background(Color::new(0.08, 0.09, 0.1, 1.0));

        let camera = Camera3D {
            position: Vec3::new(10.0, 20.0, -60.0), 
            target: Vec3::ZERO,                    
            up: Vec3::Y,                           
            ..Default::default()
        };
        
        set_camera(&camera);
        draw_cube_wires(Vec3::ZERO, Vec3::splat(BOX_SIZE * 2.0), WHITE);

        let flock_snapshot: Vec<(Vec3, Vec3)> = flock.iter().map(|b| (b.position, b.velocity)).collect();
        
        for boid in flock.iter_mut() {
            let mut separation_force = Vec3::ZERO;
            let mut separation_count = 0;

            let mut alignment_force = Vec3::ZERO;
            let mut alignment_count = 0;

            let mut cohesion_center = Vec3::ZERO;
            let mut cohesion_count = 0;

            let forward = boid.velocity.normalize_or_zero();

            for &(other_pos, other_vel) in flock_snapshot.iter() {
                if boid.position == other_pos {
                    continue;
                }

                let distance = boid.position.distance(other_pos);

                // 1. Separation
                if distance < PERSONAL_SPACE && distance > 0.0 {
                    let mut away_vector = boid.position - other_pos;
                    away_vector = away_vector.normalize() / distance;
                    separation_force += away_vector;
                    separation_count += 1;
                }

                // 2. Alignment & Cohesion
                if distance < VISUAL_RANGE {
                    let to_neighbor = (other_pos - boid.position).normalize_or_zero();
                    
                    // -0.5 gives them a 240-degree field of view.
                    if forward.dot(to_neighbor) > -0.5 {
                        alignment_force += other_vel;
                        alignment_count += 1;

                        cohesion_center += other_pos;
                        cohesion_count += 1;
                    }
                }
            }

            if separation_count > 0 {
                separation_force /= separation_count as f32;
                if separation_force.length_squared() > 0.0 {
                    let desired_velocity = separation_force.normalize() * MAX_SPEED;
                    let steer = desired_velocity - boid.velocity;
                    separation_force = steer.clamp_length_max(MAX_FORCE);
                }
            } else {
                separation_force = Vec3::ZERO;
            }

            if alignment_count > 0 {
                alignment_force /= alignment_count as f32;
                if alignment_force.length_squared() > 0.0 {
                    let desired_velocity = alignment_force.normalize() * MAX_SPEED;
                    let steer = desired_velocity - boid.velocity;
                    alignment_force = steer.clamp_length_max(MAX_FORCE);
                }
            } else {
                alignment_force = Vec3::ZERO;
            }

            let mut cohesion_force = Vec3::ZERO;
            if cohesion_count > 0 {
                cohesion_center /= cohesion_count as f32; 
                let desired_direction = cohesion_center - boid.position; 
                
                if desired_direction.length_squared() > 0.0 {
                    let desired_velocity = desired_direction.normalize() * MAX_SPEED;
                    let steer = desired_velocity - boid.velocity;
                    cohesion_force = steer.clamp_length_max(MAX_FORCE);
                }
            }

            let wall_avoidance_force = boid.avoid_walls();
            
            // Weights
            let mut flocking_steering = Vec3::ZERO;
            flocking_steering += separation_force * SEPARATION_WEIGHT;
            flocking_steering += alignment_force * ALIGNMENT_WEIGHT;
            flocking_steering += cohesion_force * COHESION_WEIGHT;
            
            boid.acceleration += flocking_steering.clamp_length_max(MAX_FORCE);
            boid.acceleration += wall_avoidance_force;

            boid.update();
            boid.draw();
        }

        set_default_camera();
        next_frame().await
    }
}
