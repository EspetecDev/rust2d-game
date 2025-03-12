mod player;

use std::time::{Instant, Duration};
use minifb::{Key, Window, WindowOptions};
use player::{Player, DirState};

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;

fn paint_window(buffer: &mut [u32], player: &Player) {
    for i in 0..buffer.len() {
        let color = 0x00AA11;
        // if i % 2 == 0 {
        //     color = 0x00FF11;
        // } else {
        //     color = 0x000000;
        // }

        buffer[i] = color;
    }
}

fn handle_input(window: &Window, player: &mut Player) {

    if window.is_key_down(Key::W) { player.move_player(DirState::Zero, DirState::Positive); }
    if window.is_key_down(Key::A) { player.move_player(DirState::Positive, DirState::Zero); }
    if window.is_key_down(Key::S) { player.move_player(DirState::Zero, DirState::Negative); }
    if window.is_key_down(Key::D) { player.move_player(DirState::Negative, DirState::Zero); }

    if window.is_key_down(Key::LeftShift) { player.boost(true); }
    if window.is_key_released(Key::LeftShift) { player.boost(false); }
}

fn main() {
    
    let mut buffer = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new("Rust 2D Game", WIDTH, HEIGHT, WindowOptions::default()).expect("Failed to create window");
    let frame_duration = Duration::from_millis(16); // 60 fps
    let mut last_time = Instant::now();
    let mut player = Player::new(0, 0, 0xFF1111, 24, 12, 24);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        
        let now = Instant::now();
        if now.duration_since(last_time) >= frame_duration {
            last_time = now;
            
            // Update
            handle_input(&window, &mut player);
            
            // Render
            paint_window(&mut buffer, &player);
            player.draw(&mut buffer);
            window.update_with_buffer(&buffer, WIDTH, HEIGHT).expect("Failed to update buffer");
        }

    }
}
