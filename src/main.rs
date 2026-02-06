use minifb::{Key, Window, WindowOptions};

const SCREEN_WIDTH: usize = 320;
const SCREEN_HEIGHT: usize = 200;

use raze::{math::Vector, player::Player, render_frame, world_map::WorldMap};

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

    let player = Player::new(Vector::new(0.0, 0.0), Vector::new(0.0, 0.0));
    let mut world_map = vec![0u32; 64 * 64];
    world_map[64 * 2 + 3] = 1;

    let world_map = WorldMap::new(64, 64, world_map);

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
