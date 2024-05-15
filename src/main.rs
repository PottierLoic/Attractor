use macroquad::{miniquad::conf, prelude::*};

mod constants;
use constants::*;
mod vector3;
use vector3::Vector3;
mod attractor;
use attractor::Attractor;

fn window_conf () -> conf::Conf {
  conf::Conf {
    window_title: "Attractor".to_owned(),
    window_width: SCREEN_SIZE as i32,
    window_height: SCREEN_SIZE as i32,
    ..Default::default()
  }
}

fn display_circle(vec: Vector3, radius: f32, color: Color) {
  let pos_x: f32 = SCREEN_SIZE / 2.0 + vec.x * SCALE_FACTOR;
  let pos_y: f32 = SCREEN_SIZE / 2.0 + vec.y * SCALE_FACTOR;
  draw_circle(pos_x, pos_y, radius, color);
}

#[macroquad::main(window_conf)]
async fn main() {
  let mut attractor = Attractor::new(100, 10.0, 28.0, 8.0 / 3.0);

  let mut last_update_time = get_time();
  loop {
    let now = get_time();
    let delta_time = now - last_update_time;
    if delta_time >= REFRESH_RATE as f64 {
      last_update_time = now;
      attractor.update(delta_time as f32);
    }
    clear_background(BACKGROUND);
    for point in &attractor.points {
      display_circle(*point, BALL_RADIUS, WHITE);
    }
    next_frame().await
  }
}
