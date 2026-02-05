use minifb::{Key, Window, WindowOptions};

const SCREEN_WIDTH: usize = 320;
const SCREEN_HEIGHT: usize = 200;

use raze::{Player, WorldMap, render_frame};

fn main() {
    let mut buffer: Vec<u32> = vec![0; SCREEN_WIDTH * SCREEN_HEIGHT];

    let mut window = Window::new(
        "raze",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.set_target_fps(60);

    let player = Player::new(0.0, 0.0);
    let world_map = WorldMap::new(64, 64, vec![0; 64 * 64]);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        buffer =
            render_frame(&player, &world_map, SCREEN_WIDTH, SCREEN_HEIGHT).unwrap_or_else(|e| {
                panic!("{}", e);
            });

        window
            .update_with_buffer(&buffer, SCREEN_WIDTH, SCREEN_HEIGHT)
            .unwrap();
    }
}
