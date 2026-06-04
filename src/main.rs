use macroquad::prelude::*;

#[macroquad::main("Boids Simulation")]
async fn main() {
    loop {
	clear_background(BLACK);
	// TODO: Boids logic
	
	// TODO: Draw Boids
	draw_text("Env rdy", 20.0, 40.0, 30.0, WHITE);
	
	// wait for the next frame
	next_frame().await
    }
}
