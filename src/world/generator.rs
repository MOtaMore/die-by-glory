use crate::world::tile::{BiomeKind, Tile, TileKind};
use noise::{NoiseFn, Perlin};

pub const MAP_WIDTH: usize = 120;
pub const MAP_HEIGHT: usize = 80;
pub const Z_MIN: i32 = -50;
pub const Z_MAX: i32 = 49;
pub const NUM_Z: usize = 100;
pub const Z_SURFACE: i32 = 0;

pub fn z_to_index(z: i32) -> usize {
    ((z - Z_MIN).clamp(0, (NUM_Z - 1) as i32)) as usize
}

#[allow(dead_code)]
pub fn index_to_z(idx: usize) -> i32 {
    idx as i32 + Z_MIN
}

pub struct WorldGenerator {
    seed: u32,
    primary_biome: BiomeKind,
}

impl WorldGenerator {
    pub fn new(seed: u32, primary_biome: BiomeKind) -> Self {
        Self { seed, primary_biome }
    }

    pub fn generate(&self) -> (Vec<Tile>, Vec<i8>, (i32, i32, i32)) {
        macroquad::rand::srand(self.seed as u64);
        let total_tiles = MAP_WIDTH * MAP_HEIGHT * NUM_Z;
        let mut tiles = Vec::with_capacity(total_tiles);

        for _ in 0..total_tiles {
            tiles.push(Tile::air());
        }

        let mut surface_heights = vec![0i8; MAP_WIDTH * MAP_HEIGHT];

        let perlin_river = Perlin::new(self.seed);
        let perlin_terrain = Perlin::new(self.seed.wrapping_add(101));
        let perlin_relief = Perlin::new(self.seed.wrapping_add(404));
        let perlin_caves = Perlin::new(self.seed.wrapping_add(202));
        let perlin_clouds = Perlin::new(self.seed.wrapping_add(303));

        // 1. CALCULATE NATURAL SURFACE RELIEF ELEVATION FOR EVERY (X, Y)
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                let fx = x as f64;
                let fy = y as f64;

                let river_bend = perlin_river.get([fy * 0.04, 0.5]) * 18.0;
                let river_center = (MAP_WIDTH as f64 * 0.35) + ((fy / MAP_HEIGHT as f64) * 30.0) + river_bend;
                let dist_to_river = (fx - river_center).abs();

                // Base relief noise (gentle continuous hills)
                let relief_sample = perlin_relief.get([fx * 0.045, fy * 0.045]);

                let ground_z: i32 = match self.primary_biome {
                    BiomeKind::Mountain => {
                        let mount_sample = perlin_terrain.get([fx * 0.06, fy * 0.06]);
                        let elev = (relief_sample * 3.5 + mount_sample * 4.5).round() as i32;
                        elev.clamp(-1, 8)
                    }
                    BiomeKind::Desert => {
                        // Rolling dunes
                        let dune = (relief_sample * 2.8).round() as i32;
                        dune.clamp(-1, 4)
                    }
                    BiomeKind::Beach | BiomeKind::Ocean => {
                        let coast_line = (MAP_WIDTH as f64 * 0.40) + perlin_terrain.get([fy * 0.05, 0.2]) * 15.0;
                        let dist_to_coast = fx - coast_line;
                        if dist_to_coast < -3.0 {
                            -1 // Ocean water level
                        } else if dist_to_coast < 6.0 {
                            0 // Beach
                        } else {
                            (relief_sample * 2.0).round().clamp(0.0, 3.0) as i32 // Coastal bluffs
                        }
                    }
                    _ => {
                        // Meadow and Forest: gentle rolling hills with river depression
                        let mut h = (relief_sample * 2.5).round() as i32; // -2 to +2
                        if dist_to_river < 2.5 {
                            h = h.min(-1); // River trench
                        } else if dist_to_river < 4.5 {
                            h = h.min(0); // Riverbank slope
                        }
                        h.clamp(-2, 3)
                    }
                };

                let ground_z = ground_z.clamp(Z_MIN + 5, 15);
                surface_heights[x + y * MAP_WIDTH] = ground_z as i8;

                // 2. FILL SOLID UNDERGROUND UP TO GROUND_Z
                for z in Z_MIN..ground_z {
                    let idx = Self::tile_index(x, y, z_to_index(z));
                    if z == Z_MIN {
                        tiles[idx] = Tile::bedrock();
                    } else if z >= ground_z - 2 {
                        tiles[idx] = Tile::subsoil();
                    } else {
                        // Caverns in deep rock
                        let cave_sample = perlin_caves.get([
                            fx * 0.08,
                            fy * 0.08,
                            z as f64 * 0.12,
                        ]);

                        if cave_sample > 0.32 {
                            let roll: f32 = macroquad::rand::gen_range(0.0, 1.0);
                            if roll < 0.08 {
                                tiles[idx] = Tile::cavern_mushroom();
                            } else if roll < 0.15 {
                                tiles[idx] = Tile::water_shallow();
                            } else {
                                tiles[idx] = Tile::cavern_floor();
                            }
                        } else {
                            let roll: f32 = macroquad::rand::gen_range(0.0, 1.0);
                            if roll < 0.02 {
                                tiles[idx] = Tile::iron_ore();
                            } else if roll < 0.04 {
                                tiles[idx] = Tile::coal_ore();
                            } else if z < -25 {
                                tiles[idx] = Tile::granite();
                            } else {
                                tiles[idx] = Tile::stone();
                            }
                        }
                    }
                }

                // 3. PLACE WALKABLE SURFACE TILE AT GROUND_Z
                let surf_idx = Self::tile_index(x, y, z_to_index(ground_z));
                let surf_tile = match self.primary_biome {
                    BiomeKind::Mountain => {
                        if ground_z >= 4 {
                            Tile::snow()
                        } else if ground_z >= 2 {
                            if macroquad::rand::gen_range(0.0, 1.0) < 0.2 {
                                Tile::mountain_peak()
                            } else {
                                Tile::mountain_rock()
                            }
                        } else if ground_z <= -1 {
                            Tile::water_shallow()
                        } else {
                            Tile::grass(macroquad::rand::gen_range(0, 4))
                        }
                    }

                    BiomeKind::Desert => {
                        let oasis_dist = (fx - 60.0).hypot(fy - 40.0);
                        if oasis_dist < 4.0 {
                            Tile::water_shallow()
                        } else if oasis_dist < 7.0 && macroquad::rand::gen_range(0.0, 1.0) < 0.25 {
                            Tile::palm_tree()
                        } else if ground_z >= 2 {
                            Tile::desert_dune()
                        } else if macroquad::rand::gen_range(0.0, 1.0) < 0.06 {
                            Tile::cactus()
                        } else {
                            Tile::desert_sand()
                        }
                    }

                    BiomeKind::Beach | BiomeKind::Ocean => {
                        let coast_line = (MAP_WIDTH as f64 * 0.40) + perlin_terrain.get([fy * 0.05, 0.2]) * 15.0;
                        let dist_to_coast = fx - coast_line;

                        if dist_to_coast < -5.0 {
                            Tile::ocean_water()
                        } else if dist_to_coast < 0.0 {
                            Tile::water_shallow()
                        } else if dist_to_coast < 7.0 {
                            if macroquad::rand::gen_range(0.0, 1.0) < 0.03 {
                                Tile::palm_tree()
                            } else {
                                Tile::beach_sand()
                            }
                        } else {
                            Tile::grass(macroquad::rand::gen_range(0, 4))
                        }
                    }

                    BiomeKind::Forest => {
                        if dist_to_river < 2.0 {
                            Tile::water_deep()
                        } else if dist_to_river < 3.8 {
                            Tile::water_shallow()
                        } else if dist_to_river < 5.2 {
                            Tile::sand()
                        } else {
                            let roll: f32 = macroquad::rand::gen_range(0.0, 1.0);
                            if roll < 0.24 {
                                Tile::tree_trunk()
                            } else if roll < 0.42 {
                                Tile::foliage()
                            } else if roll < 0.68 {
                                Tile::soil()
                            } else {
                                Tile::grass(macroquad::rand::gen_range(0, 4))
                            }
                        }
                    }

                    _ => {
                        // Meadow and River Valley
                        if dist_to_river < 2.2 {
                            Tile::water_deep()
                        } else if dist_to_river < 4.2 {
                            Tile::water_shallow()
                        } else if dist_to_river < 6.0 {
                            Tile::sand()
                        } else {
                            let roll: f32 = macroquad::rand::gen_range(0.0, 1.0);
                            if roll < 0.12 {
                                Tile::tall_grass()
                            } else if roll < 0.22 {
                                Tile::wildflower(macroquad::rand::gen_range(0, 3))
                            } else {
                                Tile::grass(macroquad::rand::gen_range(0, 4))
                            }
                        }
                    }
                };

                tiles[surf_idx] = surf_tile;

                // Canopy layers for trees placed at ground_z
                if surf_tile.kind == TileKind::TreeTrunk {
                    for cz in 1..=2 {
                        let tree_z = ground_z + cz;
                        if tree_z <= Z_MAX {
                            let cz_idx = z_to_index(tree_z);
                            for dy in -1..=1 {
                                for dx in -1..=1 {
                                    let cx = x as i32 + dx;
                                    let cy = y as i32 + dy;
                                    if cx >= 0 && cx < MAP_WIDTH as i32 && cy >= 0 && cy < MAP_HEIGHT as i32 {
                                        let cidx = Self::tile_index(cx as usize, cy as usize, cz_idx);
                                        if tiles[cidx].kind == TileKind::Air {
                                            tiles[cidx] = Tile::tree_canopy(macroquad::rand::gen_range(0, 3));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // 4. GENERATE ATMOSPHERE & CLOUD DECK HIGH IN SKY
        for z in 20..=26 {
            let z_idx = z_to_index(z);
            for y in 0..MAP_HEIGHT {
                for x in 0..MAP_WIDTH {
                    let cloud_val = perlin_clouds.get([
                        x as f64 * 0.05,
                        y as f64 * 0.05,
                        (z - 20) as f64 * 0.2,
                    ]);
                    if cloud_val > 0.38 {
                        let idx = Self::tile_index(x, y, z_idx);
                        tiles[idx] = Tile::cloud();
                    }
                }
            }
        }

        // 5. SAFE SPAWN LOCATION ON SOLID DRY GROUND SURFACE
        let spawn_pos = Self::find_safe_spawn(&tiles, &surface_heights);

        (tiles, surface_heights, spawn_pos)
    }

    pub fn tile_index(x: usize, y: usize, z_idx: usize) -> usize {
        x + y * MAP_WIDTH + z_idx * (MAP_WIDTH * MAP_HEIGHT)
    }

    fn find_safe_spawn(tiles: &[Tile], surface_heights: &[i8]) -> (i32, i32, i32) {
        let center_x = (MAP_WIDTH / 2) as i32;
        let center_y = (MAP_HEIGHT / 2) as i32;

        for radius in 0..MAP_WIDTH.max(MAP_HEIGHT) as i32 {
            for dy in -radius..=radius {
                for dx in -radius..=radius {
                    if dx.abs() != radius && dy.abs() != radius {
                        continue;
                    }

                    let x = center_x + dx;
                    let y = center_y + dy;

                    if x >= 1 && x < (MAP_WIDTH as i32 - 1) && y >= 1 && y < (MAP_HEIGHT as i32 - 1) {
                        let gz = surface_heights[x as usize + y as usize * MAP_WIDTH] as i32;
                        let z_idx = z_to_index(gz);
                        let idx = Self::tile_index(x as usize, y as usize, z_idx);
                        let tile = &tiles[idx];

                        // Safe dry walkable land
                        if tile.walkable
                            && tile.kind != TileKind::WaterDeep
                            && tile.kind != TileKind::OceanWater
                            && tile.kind != TileKind::WaterShallow
                            && tile.kind != TileKind::TreeTrunk
                            && tile.kind != TileKind::MountainPeak
                        {
                            return (x, y, gz);
                        }
                    }
                }
            }
        }

        (center_x, center_y, Z_SURFACE)
    }
}
