mod player;
mod ecs;

use std::time::{Instant, Duration};
use minifb::{Key, Window, WindowOptions};
use ecs::{draw_entities, movement_system, Entity, EntityManager, Tag};
use rand::prelude::*;

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;
const PLAYER_SPEED : usize = 12;
const PLAYER_BOOST_SPEED: usize = 24;

fn clear_window(buffer: &mut [u32]) {
    for i in 0..buffer.len() {
        buffer[i] = 0x000000;
    }
}

fn handle_input(window: &Window, ecs: &mut EntityManager, player: Entity) {

    
    let boost = window.is_key_down(Key::LeftShift);
    let mut dx = 0;
    let mut dy = 0;
    if window.is_key_down(Key::W) { 
        dy = if boost { PLAYER_BOOST_SPEED.wrapping_neg() } else { PLAYER_SPEED.wrapping_neg() };
    }
    else if window.is_key_down(Key::S) {
        dy = if boost { PLAYER_BOOST_SPEED } else { PLAYER_SPEED };
    }

    if window.is_key_down(Key::A) {
        dx = if boost { PLAYER_BOOST_SPEED.wrapping_neg() } else { PLAYER_SPEED.wrapping_neg() };
    }
    else if window.is_key_down(Key::D) {
        dx = if boost { PLAYER_BOOST_SPEED } else { PLAYER_SPEED };
    }

    if dx == 0 && dy == 0 {
        ecs.reset_velocity(player, true, true);
    } else {
        ecs.add_velocity(player, dx as i32, dy as i32);
    }

}

fn main() {
    // Render init
    let mut buffer = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new("Rust 2D Game", WIDTH, HEIGHT, WindowOptions::default()).expect("Failed to create window");
    // Update init
    let frame_duration = Duration::from_millis(16); // 60 fps
    let mut last_time = Instant::now();
    // ECS init
    let mut ecs: EntityManager = EntityManager::new();

    // Spawn player
    let player = ecs.create_entity();
    ecs.add_position(player, (WIDTH / 2) as u32, (HEIGHT / 2) as u32);
    ecs.add_velocity(player, 0 , 0);
    ecs.add_tag(player, Tag {tag: "player"});
    ecs.add_sprite(player, 24, 24, 0xFF1111);

    // Spawn enemy
    let num_enemies = 3;
    for idx in 1..num_enemies + 1 {
        let enemy = ecs.create_entity();
        ecs.add_position(enemy, rand::random_range(0..WIDTH) as u32, rand::random_range(0..HEIGHT) as u32);
        // ecs.add_velocity(enemy, rand::random_range(-1..1), rand::random_range(-1..1));
        ecs.add_velocity(enemy, 0, 0);
        ecs.add_tag(enemy, Tag {tag: "enemy"});
        ecs.add_sprite(enemy, 12*idx,12*idx, rand::random_range(0..0xFFFFFF));
    }

    while window.is_open() && !window.is_key_down(Key::Escape) {
        
        let now = Instant::now();
        if now.duration_since(last_time) >= frame_duration {
            last_time = now;
            clear_window(&mut buffer);
            // Update
            handle_input(&window, &mut ecs, player);
            movement_system(&mut ecs);
            
            // Render
            draw_entities(&ecs, &mut buffer);
            
            // player.draw(&mut buffer);
            window.update_with_buffer(&buffer, WIDTH, HEIGHT).expect("Failed to update buffer");
        }

    }
}
