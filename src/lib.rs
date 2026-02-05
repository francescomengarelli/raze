#[derive(thiserror::Error, Debug)]
#[error("rendering failed")]
pub struct RenderError;

pub struct Player {
    x: f32,
    y: f32,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Player { x, y }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }
}

pub struct WorldMap {
    width: usize,
    height: usize,
    map: Vec<u32>,
}

impl WorldMap {
    pub fn new(width: usize, height: usize, map: Vec<u32>) -> Self {
        WorldMap { width, height, map }
    }
}

pub fn render_frame(
    player: &Player,
    world_map: &WorldMap,
    screen_width: usize,
    screen_height: usize,
) -> Result<Vec<u32>, RenderError> {
    let mut buffer: Vec<u32> = vec![0; screen_width * screen_height];

    buffer.fill(0);

    buffer[screen_height / 2 * screen_width + screen_width / 2] = 0xFF0000;

    return Ok(buffer);
}
