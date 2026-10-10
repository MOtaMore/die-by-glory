use crate::world::tile::BiomeKind;
use macroquad::prelude::*;
use noise::{NoiseFn, Perlin};

pub const OW_WIDTH: usize = 68;
pub const OW_HEIGHT: usize = 34;

#[derive(Clone, Copy, Debug)]
pub struct OverworldTile {
    pub biome: BiomeKind,
    pub glyph: char,
    pub fg_color: Color,
    pub bg_color: Color,
    pub is_land: bool,
}

#[derive(Clone, Debug)]
pub struct Overworld {
    pub tiles: Vec<OverworldTile>,
    pub seed: u32,
    pub cursor_x: usize,
    pub cursor_y: usize,
    pub current_station: Option<(usize, usize)>,
    pub visited_stations: Vec<(usize, usize)>,
}

impl Overworld {
    pub fn new(seed: u32) -> Self {
        let mut ow = Self {
            tiles: Vec::with_capacity(OW_WIDTH * OW_HEIGHT),
            seed,
            cursor_x: OW_WIDTH / 2,
            cursor_y: OW_HEIGHT / 2,
            current_station: None,
            visited_stations: Vec::new(),
        };
        ow.generate();
        ow.ensure_valid_cursor();
        ow
    }

    pub fn generate(&mut self) {
        self.tiles.clear();
        let perlin_land = Perlin::new(self.seed);
        let perlin_moist = Perlin::new(self.seed.wrapping_add(101));
        let perlin_temp = Perlin::new(self.seed.wrapping_add(202));
        let perlin_mount = Perlin::new(self.seed.wrapping_add(303));

        for y in 0..OW_HEIGHT {
            for x in 0..OW_WIDTH {
                let nx = x as f64 / OW_WIDTH as f64;
                let ny = y as f64 / OW_HEIGHT as f64;

                let edge_dist_x = (nx - 0.5).abs() * 2.0;
                let edge_dist_y = (ny - 0.5).abs() * 2.0;
                let edge_falloff = (edge_dist_x.max(edge_dist_y)).powf(2.2);

                let land_val = perlin_land.get([nx * 4.5, ny * 4.5]) * 0.7
                    + perlin_land.get([nx * 9.0, ny * 9.0]) * 0.3
                    - (edge_falloff * 0.85);

                let moisture = perlin_moist.get([nx * 3.5, ny * 3.5]);
                let temp = perlin_temp.get([nx * 3.0, ny * 3.0]);
                let mountain_val = perlin_mount.get([nx * 5.0, ny * 5.0]);

                let (biome, glyph, fg, bg, is_land) = if land_val < -0.05 {
                    (
                        BiomeKind::Ocean,
                        '≈',
                        Color::new(0.20, 0.45, 0.85, 1.0),
                        Color::new(0.04, 0.10, 0.28, 1.0),
                        false,
                    )
                } else if land_val < 0.06 {
                    (
                        BiomeKind::Beach,
                        '.',
                        Color::new(0.95, 0.88, 0.55, 1.0),
                        Color::new(0.35, 0.30, 0.18, 1.0),
                        true,
                    )
                } else {
                    if mountain_val > 0.38 {
                        if mountain_val > 0.55 {
                            (
                                BiomeKind::Mountain,
                                '^',
                                Color::new(0.95, 0.95, 1.0, 1.0),
                                Color::new(0.40, 0.42, 0.50, 1.0),
                                true,
                            )
                        } else {
                            (
                                BiomeKind::Mountain,
                                '▲',
                                Color::new(0.75, 0.75, 0.80, 1.0),
                                Color::new(0.28, 0.28, 0.35, 1.0),
                                true,
                            )
                        }
                    } else if temp > 0.18 && moisture < -0.08 {
                        let g = if ((x + y) % 7) == 0 { 'ψ' } else { '~' };
                        (
                            BiomeKind::Desert,
                            g,
                            Color::new(0.98, 0.82, 0.35, 1.0),
                            Color::new(0.40, 0.30, 0.14, 1.0),
                            true,
                        )
                    } else if moisture > 0.12 {
                        let g = if ((x + y) % 3) == 0 { '♣' } else { '♠' };
                        (
                            BiomeKind::Forest,
                            g,
                            Color::new(0.25, 0.70, 0.30, 1.0),
                            Color::new(0.06, 0.20, 0.08, 1.0),
                            true,
                        )
                    } else {
                        (
                            BiomeKind::Meadow,
                            '.',
                            Color::new(0.40, 0.82, 0.32, 1.0),
                            Color::new(0.08, 0.22, 0.09, 1.0),
                            true,
                        )
                    }
                };

                self.tiles.push(OverworldTile {
                    biome,
                    glyph,
                    fg_color: fg,
                    bg_color: bg,
                    is_land,
                });
            }
        }
    }

    pub fn set_station(&mut self, x: usize, y: usize) {
        self.current_station = Some((x, y));
        if !self.visited_stations.contains(&(x, y)) {
            self.visited_stations.push((x, y));
        }
    }

    pub fn is_adjacent_to_current_station(&self, target_x: usize, target_y: usize) -> bool {
        match self.current_station {
            Some((sx, sy)) => {
                let dx = (target_x as i32 - sx as i32).abs();
                let dy = (target_y as i32 - sy as i32).abs();
                dx <= 1 && dy <= 1 && !(dx == 0 && dy == 0)
            }
            None => true, // First embark allows any land tile
        }
    }

    pub fn station_has_adjacent_land(&self, sx: usize, sy: usize) -> bool {
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = sx as i32 + dx;
                let ny = sy as i32 + dy;
                if nx >= 0 && nx < OW_WIDTH as i32 && ny >= 0 && ny < OW_HEIGHT as i32 {
                    if let Some(tile) = self.get_tile(nx as usize, ny as usize) {
                        if tile.is_land {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    pub fn can_embark_at(&self, target_x: usize, target_y: usize, is_fast_travel: bool) -> (bool, &'static str) {
        let is_land = self.get_tile(target_x, target_y).map(|t| t.is_land).unwrap_or(false);
        if !is_land {
            return (false, "Cannot embark in deep ocean water!");
        }

        if !is_fast_travel {
            return (true, "Valid embark site.");
        }

        match self.current_station {
            Some((sx, sy)) => {
                if target_x == sx && target_y == sy {
                    return (true, "Current local camp: re-enter this area.");
                }

                if self.is_adjacent_to_current_station(target_x, target_y) {
                    return (true, "Adjacent region: valid travel step.");
                }

                if self.visited_stations.contains(&(target_x, target_y)) {
                    return (true, "Known station: travel to visited camp.");
                }

                // Si la estación actual es una isla aislada sin tierra vecina, permitir viaje marítimo al continente
                if !self.station_has_adjacent_land(sx, sy) {
                    return (true, "Coastal expedition: sea voyage from isolated island to mainland.");
                }

                (false, "Too far: you can only travel to adjacent regions or visited camps!")
            }
            None => (true, "Valid embark site."),
        }
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Option<&OverworldTile> {
        if x < OW_WIDTH && y < OW_HEIGHT {
            Some(&self.tiles[x + y * OW_WIDTH])
        } else {
            None
        }
    }

    pub fn current_selected_tile(&self) -> Option<&OverworldTile> {
        self.get_tile(self.cursor_x, self.cursor_y)
    }

    pub fn ensure_valid_cursor(&mut self) {
        if let Some(t) = self.current_selected_tile() {
            if t.is_land {
                return;
            }
        }

        let cx = OW_WIDTH / 2;
        let cy = OW_HEIGHT / 2;
        for radius in 1..OW_WIDTH {
            for dy in -(radius as i32)..=(radius as i32) {
                for dx in -(radius as i32)..=(radius as i32) {
                    let tx = cx as i32 + dx;
                    let ty = cy as i32 + dy;
                    if tx >= 0 && tx < OW_WIDTH as i32 && ty >= 0 && ty < OW_HEIGHT as i32 {
                        if let Some(tile) = self.get_tile(tx as usize, ty as usize) {
                            if tile.is_land {
                                self.cursor_x = tx as usize;
                                self.cursor_y = ty as usize;
                                return;
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn select_first_biome(&mut self, target_biome: BiomeKind) {
        for y in 0..OW_HEIGHT {
            for x in 0..OW_WIDTH {
                if let Some(t) = self.get_tile(x, y) {
                    if t.biome == target_biome {
                        self.cursor_x = x;
                        self.cursor_y = y;
                        return;
                    }
                }
            }
        }
    }

    pub fn regenerate(&mut self) {
        self.seed = macroquad::rand::rand();
        self.generate();
        self.current_station = None;
        self.visited_stations.clear();
        self.ensure_valid_cursor();
    }
}
