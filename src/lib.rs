#[derive(thiserror::Error, Debug)]
#[error("rendering failed")]
pub struct RenderError;

pub mod math;
pub mod player;
pub mod world_map;

pub fn render_frame(
    player: &player::Player,
    world_map: &world_map::WorldMap,
    screen_width: usize,
    screen_height: usize,
) -> Result<Vec<u32>, RenderError> {
    let mut buffer: Vec<u32> = vec![0; screen_width * screen_height];

    buffer.fill(0);

    buffer[screen_height / 2 * screen_width + screen_width / 2] = 0xFF0000;

    return Ok(buffer);
}
