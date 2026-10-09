use crate::entity::{Creature, CreatureType, GroundItem, ItemType};

pub mod biome;
pub mod generator;
pub mod overworld;
pub mod tile;

pub use tile::BiomeKind;
pub use generator::{
    z_to_index, WorldGenerator, MAP_HEIGHT, MAP_WIDTH, Z_MAX, Z_MIN, Z_SURFACE,
};
pub use overworld::{Overworld, OW_HEIGHT, OW_WIDTH};
pub use tile::{Tile, TileKind};

pub struct World {
    pub tiles: Vec<Tile>,
    pub surface_heights: Vec<i8>,
    pub spawn_pos: (i32, i32, i32),
    pub time_of_day: f32, // Hours (0.0 to 24.0)
    pub primary_biome: BiomeKind,
    pub ground_items: Vec<GroundItem>,
    pub creatures: Vec<Creature>,
    pub next_entity_id: u64,
}

impl World {
    pub fn new(seed: u32, primary_biome: BiomeKind) -> Self {
        let generator = WorldGenerator::new(seed, primary_biome);
        let (tiles, surface_heights, spawn_pos) = generator.generate();

        let mut world = Self {
            tiles,
            surface_heights,
            spawn_pos,
            time_of_day: 10.5, // Starts mid-morning (10:30 AM)
            primary_biome,
            ground_items: Vec::new(),
            creatures: Vec::new(),
            next_entity_id: 1,
        };

        world.spawn_initial_entities(seed);
        world
    }

    #[allow(dead_code)]
    pub fn default_meadow(seed: u32) -> Self {
        Self::new(seed, BiomeKind::Meadow)
    }

    #[inline]
    pub fn get_surface_z(&self, x: i32, y: i32) -> i32 {
        if x >= 0 && x < MAP_WIDTH as i32 && y >= 0 && y < MAP_HEIGHT as i32 {
            self.surface_heights[x as usize + y as usize * MAP_WIDTH] as i32
        } else {
            Z_SURFACE
        }
    }

    #[inline]
    pub fn is_valid_coord(x: i32, y: i32, z: i32) -> bool {
        x >= 0
            && x < MAP_WIDTH as i32
            && y >= 0
            && y < MAP_HEIGHT as i32
            && z >= Z_MIN
            && z <= Z_MAX
    }

    #[inline]
    pub fn get_tile(&self, x: i32, y: i32, z: i32) -> Option<&Tile> {
        if !Self::is_valid_coord(x, y, z) {
            return None;
        }
        let z_idx = z_to_index(z);
        let idx = WorldGenerator::tile_index(x as usize, y as usize, z_idx);
        self.tiles.get(idx)
    }

    #[allow(dead_code)]
    #[inline]
    pub fn get_tile_mut(&mut self, x: i32, y: i32, z: i32) -> Option<&mut Tile> {
        if !Self::is_valid_coord(x, y, z) {
            return None;
        }
        let z_idx = z_to_index(z);
        let idx = WorldGenerator::tile_index(x as usize, y as usize, z_idx);
        self.tiles.get_mut(idx)
    }

    /// Casts ray downward from `from_z` to find the first non-air tile.
    pub fn find_visible_surface_down(&self, x: i32, y: i32, from_z: i32) -> Option<(i32, &Tile)> {
        if x < 0 || x >= MAP_WIDTH as i32 || y < 0 || y >= MAP_HEIGHT as i32 {
            return None;
        }

        let min_search = (from_z - 35).max(Z_MIN);

        for z in (min_search..=from_z).rev() {
            if let Some(tile) = self.get_tile(x, y, z) {
                if tile.kind != TileKind::Air {
                    return Some((z, tile));
                }
            }
        }

        None
    }

    pub fn update_time(&mut self, dt: f32) {
        // Full 24h cycle every ~12 real minutes (0.05 game hours per second)
        self.time_of_day = (self.time_of_day + dt * 0.05) % 24.0;
        self.update_creatures(dt);
    }

    pub fn formatted_time(&self) -> String {
        let hours = self.time_of_day.floor() as u32;
        let minutes = ((self.time_of_day.fract()) * 60.0).floor() as u32;
        let am_pm = if hours >= 12 { "PM" } else { "AM" };
        let display_hour = if hours == 0 { 12 } else if hours > 12 { hours - 12 } else { hours };
        format!("{:02}:{:02} {}", display_hour, minutes, am_pm)
    }

    pub fn spawn_initial_entities(&mut self, seed: u32) {
        let (sx, sy, _) = self.spawn_pos;
        let mut rng_state = seed.wrapping_add(1337);

        let mut next_rnd = || -> u32 {
            rng_state = rng_state.wrapping_mul(1664525).wrapping_add(1013904223);
            rng_state
        };

        // 1. Scatter ground items (Tall Grass, Sticks, Flint, Small Stones)
        for y in 2..(MAP_HEIGHT as i32 - 2) {
            for x in 2..(MAP_WIDTH as i32 - 2) {
                if (x - sx).abs() <= 1 && (y - sy).abs() <= 1 {
                    continue;
                }

                let roll = next_rnd() % 100;
                if roll < 5 {
                    let sz = self.get_surface_z(x, y);
                    if let Some(tile) = self.get_tile(x, y, sz) {
                        if tile.walkable
                            && tile.kind != TileKind::WaterShallow
                            && tile.kind != TileKind::WaterDeep
                            && tile.kind != TileKind::OceanWater
                        {
                            let item_roll = next_rnd() % 100;
                            let item_type = match self.primary_biome {
                                BiomeKind::Meadow => {
                                    if item_roll < 45 {
                                        ItemType::TallGrass
                                    } else if item_roll < 75 {
                                        ItemType::Stick
                                    } else if item_roll < 90 {
                                        ItemType::SmallStone
                                    } else {
                                        ItemType::Flint
                                    }
                                }
                                BiomeKind::Forest => {
                                    if item_roll < 55 {
                                        ItemType::Stick
                                    } else if item_roll < 80 {
                                        ItemType::TallGrass
                                    } else if item_roll < 92 {
                                        ItemType::SmallStone
                                    } else {
                                        ItemType::Flint
                                    }
                                }
                                BiomeKind::Mountain => {
                                    if item_roll < 45 {
                                        ItemType::SmallStone
                                    } else if item_roll < 80 {
                                        ItemType::Flint
                                    } else if item_roll < 92 {
                                        ItemType::Stick
                                    } else {
                                        ItemType::TallGrass
                                    }
                                }
                                BiomeKind::Desert => {
                                    if item_roll < 45 {
                                        ItemType::Flint
                                    } else if item_roll < 85 {
                                        ItemType::SmallStone
                                    } else {
                                        ItemType::Stick
                                    }
                                }
                                BiomeKind::Beach => {
                                    if item_roll < 50 {
                                        ItemType::SmallStone
                                    } else if item_roll < 80 {
                                        ItemType::Stick
                                    } else {
                                        ItemType::Flint
                                    }
                                }
                                _ => {
                                    if item_roll < 45 {
                                        ItemType::SmallStone
                                    } else if item_roll < 75 {
                                        ItemType::TallGrass
                                    } else if item_roll < 90 {
                                        ItemType::Stick
                                    } else {
                                        ItemType::Flint
                                    }
                                }
                            };

                            let count = if item_type == ItemType::TallGrass {
                                (next_rnd() % 2) + 1
                            } else {
                                1
                            };

                            self.spawn_ground_item(item_type, count, x, y, sz);
                        }
                    }
                }
            }
        }

        // Guarantee starter cluster near spawn so player can immediately pick up items
        for (ox, oy, it) in [
            (2, 1, ItemType::TallGrass),
            (-2, 1, ItemType::Stick),
            (1, -2, ItemType::SmallStone),
            (-1, -2, ItemType::Flint),
            (3, -1, ItemType::TallGrass),
        ] {
            let gx = sx + ox;
            let gy = sy + oy;
            let gz = self.get_surface_z(gx, gy);
            self.spawn_ground_item(it, 1, gx, gy, gz);
        }

        // 2. Spawn creatures on walkable land
        let creature_types = match self.primary_biome {
            BiomeKind::Meadow => vec![CreatureType::Deer, CreatureType::Rabbit, CreatureType::Deer, CreatureType::Rabbit],
            BiomeKind::Forest => vec![CreatureType::Deer, CreatureType::Boar, CreatureType::Rabbit, CreatureType::Deer],
            BiomeKind::Mountain => vec![CreatureType::MountainGoat, CreatureType::MountainGoat, CreatureType::Rabbit],
            BiomeKind::Desert => vec![CreatureType::Rabbit, CreatureType::Rabbit, CreatureType::Rabbit],
            _ => vec![CreatureType::Deer, CreatureType::Rabbit, CreatureType::Rabbit],
        };

        for (_i, c_type) in creature_types.into_iter().enumerate() {
            let offset_x = ((next_rnd() % 28) as i32) - 14;
            let offset_y = ((next_rnd() % 28) as i32) - 14;
            let cx = (sx + offset_x).clamp(4, MAP_WIDTH as i32 - 5);
            let cy = (sy + offset_y).clamp(4, MAP_HEIGHT as i32 - 5);
            let cz = self.get_surface_z(cx, cy);

            let id = self.next_entity_id;
            self.next_entity_id += 1;
            self.creatures.push(Creature::new(id, c_type, cx, cy, cz));
        }
    }

    pub fn update_creatures(&mut self, dt: f32) {
        let count = self.creatures.len();
        for i in 0..count {
            if self.creatures[i].health <= 0.0 {
                continue;
            }
            let (cx, cy, cz, cooldown) = {
                let c = &self.creatures[i];
                (c.x, c.y, c.z, c.move_cooldown)
            };
            if cooldown > 0.0 {
                self.creatures[i].move_cooldown = cooldown - dt;
                continue;
            }
            self.creatures[i].move_cooldown = 3.5 + (i as f32 * 0.8 % 3.0);
            let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0), (0, 0)];
            let step = dirs[(i + (self.time_of_day * 10.0) as usize) % dirs.len()];
            if step.0 == 0 && step.1 == 0 {
                continue;
            }
            let nx = (cx + step.0).clamp(1, MAP_WIDTH as i32 - 2);
            let ny = (cy + step.1).clamp(1, MAP_HEIGHT as i32 - 2);
            let nz = self.get_surface_z(nx, ny);

            if (nz - cz).abs() <= 1 {
                if let Some(tile) = self.get_tile(nx, ny, nz) {
                    if tile.walkable && tile.kind != TileKind::WaterDeep && tile.kind != TileKind::OceanWater {
                        self.creatures[i].x = nx;
                        self.creatures[i].y = ny;
                        self.creatures[i].z = nz;
                    }
                }
            }
        }
    }

    pub fn find_creature_near(&self, x: i32, y: i32, max_dist: f32) -> Option<usize> {
        let mut closest_idx = None;
        let mut min_dist = max_dist;

        for (idx, c) in self.creatures.iter().enumerate() {
            if c.health <= 0.0 {
                continue;
            }
            let dx = (c.x - x) as f32;
            let dy = (c.y - y) as f32;
            let dist = (dx * dx + dy * dy).sqrt();
            if dist <= min_dist {
                min_dist = dist;
                closest_idx = Some(idx);
            }
        }
        closest_idx
    }

    pub fn pickup_carcass_at(&mut self, x: i32, y: i32, max_dist: f32) -> Option<GroundItem> {
        let mut closest_idx = None;
        let mut min_dist = max_dist;

        for (idx, item) in self.ground_items.iter().enumerate() {
            if item.item != ItemType::AnimalCarcass {
                continue;
            }
            let dx = (item.x - x) as f32;
            let dy = (item.y - y) as f32;
            let dist = (dx * dx + dy * dy).sqrt();
            if dist <= min_dist {
                min_dist = dist;
                closest_idx = Some(idx);
            }
        }
        closest_idx.map(|idx| self.ground_items.remove(idx))
    }

    pub fn pickup_item_at(&mut self, x: i32, y: i32, max_dist: f32) -> Option<GroundItem> {
        let mut closest_idx = None;
        let mut min_dist = max_dist;

        for (idx, item) in self.ground_items.iter().enumerate() {
            let dx = (item.x - x) as f32;
            let dy = (item.y - y) as f32;
            let dist = (dx * dx + dy * dy).sqrt();
            if dist <= min_dist {
                min_dist = dist;
                closest_idx = Some(idx);
            }
        }

        closest_idx.map(|idx| self.ground_items.remove(idx))
    }

    pub fn spawn_ground_item(&mut self, item: ItemType, count: u32, x: i32, y: i32, z: i32) {
        let id = self.next_entity_id;
        self.next_entity_id += 1;
        self.ground_items.push(GroundItem::new(id, item, count, x, y, z));
    }

    pub fn creature_flee(&mut self, creature_idx: usize, from_x: i32, from_y: i32) {
        if creature_idx >= self.creatures.len() {
            return;
        }
        let (cx, cy, cz) = (self.creatures[creature_idx].x, self.creatures[creature_idx].y, self.creatures[creature_idx].z);
        let mut dx = (cx - from_x).clamp(-1, 1);
        let dy = (cy - from_y).clamp(-1, 1);
        if dx == 0 && dy == 0 {
            dx = 1;
        }
        let nx = (cx + dx).clamp(2, (MAP_WIDTH as i32) - 3);
        let ny = (cy + dy).clamp(2, (MAP_HEIGHT as i32) - 3);
        let nz = self.get_surface_z(nx, ny);
        if (nz - cz).abs() <= 1 {
            if let Some(tile) = self.get_tile(nx, ny, nz) {
                if tile.walkable && tile.kind != TileKind::WaterDeep && tile.kind != TileKind::OceanWater {
                    self.creatures[creature_idx].x = nx;
                    self.creatures[creature_idx].y = ny;
                    self.creatures[creature_idx].z = nz;
                }
            }
        }
    }

    pub fn chop_tree_at(&mut self, x: i32, y: i32) -> Result<(u32, u32), &'static str> {
        if x < 0 || x >= MAP_WIDTH as i32 || y < 0 || y >= MAP_HEIGHT as i32 {
            return Err("Target is outside map boundaries.");
        }
        let sz = self.get_surface_z(x, y);
        let idx = WorldGenerator::tile_index(x as usize, y as usize, z_to_index(sz));
        let curr_kind = self.tiles[idx].kind;

        if curr_kind == TileKind::TreeTrunk {
            self.tiles[idx] = Tile::soil();
            // Clear tree canopy and foliage above
            for cz in 1..=3 {
                let tree_z = sz + cz;
                if tree_z <= Z_MAX {
                    let cz_idx = z_to_index(tree_z);
                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            let cx = x + dx;
                            let cy = y + dy;
                            if cx >= 0 && cx < MAP_WIDTH as i32 && cy >= 0 && cy < MAP_HEIGHT as i32 {
                                let cidx = WorldGenerator::tile_index(cx as usize, cy as usize, cz_idx);
                                if self.tiles[cidx].kind == TileKind::TreeCanopy || self.tiles[cidx].kind == TileKind::Foliage {
                                    self.tiles[cidx] = Tile::air();
                                }
                            }
                        }
                    }
                }
            }
            Ok((2, 2))
        } else if curr_kind == TileKind::PalmTree {
            self.tiles[idx] = Tile::beach_sand();
            Ok((2, 2))
        } else {
            Err("No tree trunk to chop down here.")
        }
    }

    pub fn shovel_dirt_at(&mut self, x: i32, y: i32) -> Result<ItemType, &'static str> {
        if x < 0 || x >= MAP_WIDTH as i32 || y < 0 || y >= MAP_HEIGHT as i32 {
            return Err("Target is outside map boundaries.");
        }
        let sz = self.get_surface_z(x, y);
        let idx = WorldGenerator::tile_index(x as usize, y as usize, z_to_index(sz));
        let curr_kind = self.tiles[idx].kind;

        match curr_kind {
            TileKind::Grass
            | TileKind::TallGrass
            | TileKind::Wildflower
            | TileKind::Soil
            | TileKind::Subsoil
            | TileKind::Sand
            | TileKind::DesertSand
            | TileKind::DesertDune
            | TileKind::BeachSand => {
                self.tiles[idx] = Tile::air();
                let new_sz = (sz - 1).max(Z_MIN + 5);
                self.surface_heights[x as usize + y as usize * MAP_WIDTH] = new_sz as i8;
                let new_idx = WorldGenerator::tile_index(x as usize, y as usize, z_to_index(new_sz));
                self.tiles[new_idx] = Tile::subsoil();
                Ok(ItemType::DirtBlock)
            }
            _ => Err("Ground is too hard or cannot be shoveled."),
        }
    }

    pub fn pickaxe_rock_at(&mut self, x: i32, y: i32) -> Result<(ItemType, u32), &'static str> {
        if x < 0 || x >= MAP_WIDTH as i32 || y < 0 || y >= MAP_HEIGHT as i32 {
            return Err("Target is outside map boundaries.");
        }
        let sz = self.get_surface_z(x, y);
        let idx = WorldGenerator::tile_index(x as usize, y as usize, z_to_index(sz));
        let curr_kind = self.tiles[idx].kind;

        match curr_kind {
            TileKind::MountainRock | TileKind::MountainPeak | TileKind::Stone | TileKind::Granite => {
                if sz > 0 {
                    self.tiles[idx] = Tile::air();
                    let new_sz = (sz - 1).max(Z_MIN + 5);
                    self.surface_heights[x as usize + y as usize * MAP_WIDTH] = new_sz as i8;
                    let new_idx = WorldGenerator::tile_index(x as usize, y as usize, z_to_index(new_sz));
                    self.tiles[new_idx] = Tile::stone();
                } else {
                    self.tiles[idx] = Tile::cavern_floor();
                }
                Ok((ItemType::Cobblestone, 2))
            }
            TileKind::Cobblestone => {
                self.tiles[idx] = Tile::soil();
                Ok((ItemType::Cobblestone, 1))
            }
            TileKind::IronOre => {
                self.tiles[idx] = Tile::stone();
                Ok((ItemType::SmallStone, 3))
            }
            TileKind::CoalOre => {
                self.tiles[idx] = Tile::stone();
                Ok((ItemType::Stick, 3))
            }
            _ => Err("No mineable rock or cobblestone deposit here."),
        }
    }

    pub fn place_block_at(&mut self, x: i32, y: i32, item: ItemType) -> Result<TileKind, &'static str> {
        if x < 0 || x >= MAP_WIDTH as i32 || y < 0 || y >= MAP_HEIGHT as i32 {
            return Err("Target is outside map boundaries.");
        }
        let tile_kind = item.placeable_tile().ok_or("Item cannot be placed as a block.")?;
        let sz = self.get_surface_z(x, y);
        let idx = WorldGenerator::tile_index(x as usize, y as usize, z_to_index(sz));
        let curr_kind = self.tiles[idx].kind;

        // If placing on top of an already paved block, stack elevation +1 (wall)
        if curr_kind == TileKind::WoodPlanks || curr_kind == TileKind::Cobblestone {
            let new_sz = (sz + 1).min(Z_MAX);
            self.surface_heights[x as usize + y as usize * MAP_WIDTH] = new_sz as i8;
            let new_idx = WorldGenerator::tile_index(x as usize, y as usize, z_to_index(new_sz));
            match tile_kind {
                TileKind::WoodPlanks => self.tiles[new_idx] = Tile::wood_planks(),
                TileKind::Cobblestone => self.tiles[new_idx] = Tile::cobblestone(),
                _ => self.tiles[new_idx] = Tile::soil(),
            }
            Ok(tile_kind)
        } else {
            // Paving the surface
            match tile_kind {
                TileKind::WoodPlanks => self.tiles[idx] = Tile::wood_planks(),
                TileKind::Cobblestone => self.tiles[idx] = Tile::cobblestone(),
                _ => self.tiles[idx] = Tile::soil(),
            }
            Ok(tile_kind)
        }
    }

    pub fn demolish_at(&mut self, x: i32, y: i32) -> Result<ItemType, &'static str> {
        if x < 0 || x >= MAP_WIDTH as i32 || y < 0 || y >= MAP_HEIGHT as i32 {
            return Err("Target is outside map boundaries.");
        }
        let sz = self.get_surface_z(x, y);
        let idx = WorldGenerator::tile_index(x as usize, y as usize, z_to_index(sz));
        let curr_kind = self.tiles[idx].kind;

        match curr_kind {
            TileKind::WoodPlanks => {
                let below_kind = self.get_tile(x, y, sz - 1).map(|t| t.kind);
                if below_kind == Some(TileKind::WoodPlanks) {
                    self.tiles[idx] = Tile::air();
                    self.surface_heights[x as usize + y as usize * MAP_WIDTH] = (sz - 1) as i8;
                } else {
                    self.tiles[idx] = Tile::soil();
                }
                Ok(ItemType::WoodPlanks)
            }
            TileKind::Cobblestone => {
                let below_kind = self.get_tile(x, y, sz - 1).map(|t| t.kind);
                if below_kind == Some(TileKind::Cobblestone) {
                    self.tiles[idx] = Tile::air();
                    self.surface_heights[x as usize + y as usize * MAP_WIDTH] = (sz - 1) as i8;
                } else {
                    self.tiles[idx] = Tile::soil();
                }
                Ok(ItemType::Cobblestone)
            }
            TileKind::TreeTrunk | TileKind::PalmTree => {
                let _ = self.chop_tree_at(x, y)?;
                Ok(ItemType::WoodLog)
            }
            TileKind::MountainRock | TileKind::MountainPeak | TileKind::Stone | TileKind::Granite => {
                let (it, _) = self.pickaxe_rock_at(x, y)?;
                Ok(it)
            }
            _ => Err("No constructible or demolishable block found here."),
        }
    }
}

