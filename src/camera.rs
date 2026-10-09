use crate::world::{Z_MAX, Z_MIN};
use macroquad::prelude::*;

pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub view_z: i32,
    pub cell_w: f32,
    pub cell_h: f32,
    pub zoom: f32,
    pub follow_player: bool,
    pub viewport_x: f32,
    pub viewport_y: f32,
    pub viewport_w: f32,
    pub viewport_h: f32,
}

impl Camera {
    pub fn new(spawn_x: i32, spawn_y: i32, spawn_z: i32) -> Self {
        Self {
            x: spawn_x as f32 + 0.5,
            y: spawn_y as f32 + 0.5,
            view_z: spawn_z,
            cell_w: 16.0,
            cell_h: 22.0,
            zoom: 1.0,
            follow_player: true,
            viewport_x: 0.0,
            viewport_y: 40.0,
            viewport_w: 960.0,
            viewport_h: 640.0,
        }
    }

    pub fn update_viewport(&mut self, screen_w: f32, screen_h: f32, sidebar_w: f32, top_h: f32, bottom_h: f32) {
        self.viewport_x = 0.0;
        self.viewport_y = top_h;
        self.viewport_w = (screen_w - sidebar_w).max(200.0);
        self.viewport_h = (screen_h - top_h - bottom_h).max(200.0);
    }

    pub fn effective_cell_size(&self) -> (f32, f32) {
        (self.cell_w * self.zoom, self.cell_h * self.zoom)
    }

    pub fn world_to_screen(&self, wx: f32, wy: f32) -> (f32, f32) {
        let (cw, ch) = self.effective_cell_size();
        let center_sx = self.viewport_x + self.viewport_w * 0.5;
        let center_sy = self.viewport_y + self.viewport_h * 0.5;

        let sx = center_sx + (wx - self.x) * cw;
        let sy = center_sy + (wy - self.y) * ch;
        (sx, sy)
    }

    pub fn screen_to_world(&self, sx: f32, sy: f32) -> (i32, i32) {
        let (cw, ch) = self.effective_cell_size();
        let center_sx = self.viewport_x + self.viewport_w * 0.5;
        let center_sy = self.viewport_y + self.viewport_h * 0.5;

        let wx = self.x + (sx - center_sx) / cw;
        let wy = self.y + (sy - center_sy) / ch;

        (wx.floor() as i32, wy.floor() as i32)
    }

    pub fn is_point_in_viewport(&self, sx: f32, sy: f32) -> bool {
        sx >= self.viewport_x
            && sx < (self.viewport_x + self.viewport_w)
            && sy >= self.viewport_y
            && sy < (self.viewport_y + self.viewport_h)
    }

    pub fn change_view_z(&mut self, delta: i32) {
        self.view_z = (self.view_z + delta).clamp(Z_MIN, Z_MAX);
    }

    pub fn set_view_z(&mut self, z: i32) {
        self.view_z = z.clamp(Z_MIN, Z_MAX);
    }

    pub fn zoom_by(&mut self, factor: f32) {
        self.zoom = (self.zoom * factor).clamp(0.5, 2.5);
    }

    pub fn smooth_follow(&mut self, target_x: f32, target_y: f32, dt: f32) {
        if self.follow_player {
            let speed = 9.0 * dt;
            self.x += (target_x - self.x) * speed.min(1.0);
            self.y += (target_y - self.y) * speed.min(1.0);
        }
    }
}
