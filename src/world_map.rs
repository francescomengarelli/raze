pub struct WorldMap {
    width: usize,
    height: usize,
    map: Vec<u32>,
}

impl WorldMap {
    pub fn new(width: usize, height: usize, map: Vec<u32>) -> Self {
        Self { width, height, map }
    }
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}
