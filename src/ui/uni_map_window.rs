use crate::model::vectors::{Vector2DF, Vector2DI};

#[derive(Debug)]
pub struct UniMapWindow {
    pub sec_size: f32,
    pub global_pos: Vector2DF,
    pub uni_map_debug_info: bool,
    pub n_sectors: Vector2DI,
}

impl UniMapWindow {
    pub fn new(sec_size: f32) -> UniMapWindow {
        UniMapWindow {
            sec_size,
            global_pos: Vector2DF { x: 0., y: 0. },
            uni_map_debug_info: false,
            n_sectors: Vector2DI { x: 0, y: 0 },
        }
    }
}

