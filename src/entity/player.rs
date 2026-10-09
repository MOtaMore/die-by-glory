use crate::entity::inventory::Inventory;
use crate::entity::item::ItemType;
use crate::world::tile::{BiomeKind, TileKind};
use crate::world::World;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum PlayerEvent {
    ReachedDestination,
    DrankWater,
    PickedUpItem(ItemType, u32),
    LowThirstWarning,
    LowHungerWarning,
    Exhausted,
}

#[derive(Clone, Debug)]
pub struct Player {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub path: Vec<(i32, i32)>,
    pub move_cooldown: f32,

    // Inventory & Equipment
    pub inventory: Inventory,

    // Survival vitals
    pub health: f32,
    pub max_health: f32,
    pub hunger: f32,
    pub max_hunger: f32,
    pub thirst: f32,
    pub max_thirst: f32,
    pub energy: f32,
    pub max_energy: f32,

    pub anim_timer: f32,
}

impl Player {
    pub fn new(spawn_pos: (i32, i32, i32)) -> Self {
        Self {
            x: spawn_pos.0,
            y: spawn_pos.1,
            z: spawn_pos.2,
            path: Vec::new(),
            move_cooldown: 0.0,
            inventory: Inventory::with_starter_items(),
            health: 100.0,
            max_health: 100.0,
            hunger: 92.0,
            max_hunger: 100.0,
            thirst: 88.0,
            max_thirst: 100.0,
            energy: 100.0,
            max_energy: 100.0,
            anim_timer: 0.0,
        }
    }

    pub fn set_path(&mut self, mut new_path: Vec<(i32, i32)>) {
        if let Some(&(fx, fy)) = new_path.first() {
            if fx == self.x && fy == self.y {
                new_path.remove(0);
            }
        }
        self.path = new_path;
    }

    pub fn cancel_movement(&mut self) {
        self.path.clear();
    }

    pub fn update(&mut self, dt: f32, world: &World) -> Option<PlayerEvent> {
        self.anim_timer += dt;

        // Passive metabolic decay
        self.thirst = (self.thirst - dt * 0.08).max(0.0);
        self.hunger = (self.hunger - dt * 0.04).max(0.0);

        // Passive stamina recovery when resting
        if self.path.is_empty() && self.thirst > 20.0 && self.hunger > 20.0 {
            self.energy = (self.energy + dt * 4.0).min(self.max_energy);
        }

        // Starvation and dehydration damage
        if self.thirst <= 0.0 || self.hunger <= 0.0 {
            self.health = (self.health - dt * 1.5).max(0.0);
        }

        if self.move_cooldown > 0.0 {
            self.move_cooldown -= dt;
        }

        // Step execution across relief
        if self.move_cooldown <= 0.0 && !self.path.is_empty() {
            let next_pos = self.path.remove(0);
            let next_z = world.get_surface_z(next_pos.0, next_pos.1);

            if let Some(target_tile) = world.get_tile(next_pos.0, next_pos.1, next_z) {
                if target_tile.walkable {
                    let old_z = self.z;
                    self.x = next_pos.0;
                    self.y = next_pos.1;
                    self.z = next_z;

                    // Slope movement cost
                    let slope_mult = if next_z > old_z {
                        1.35
                    } else if next_z < old_z {
                        0.90
                    } else {
                        1.0
                    };

                    let step_cost = target_tile.move_cost * slope_mult;
                    self.move_cooldown = 0.13 * step_cost;
                    self.energy = (self.energy - 0.45 * step_cost).max(0.0);
                    self.hunger = (self.hunger - 0.05 * step_cost).max(0.0);
                    self.thirst = (self.thirst - 0.08 * step_cost).max(0.0);

                    if self.path.is_empty() {
                        return Some(PlayerEvent::ReachedDestination);
                    }
                } else {
                    self.path.clear();
                }
            }
        }

        None
    }

    pub fn can_drink_water(&self, world: &World) -> bool {
        for dy in -1..=1 {
            for dx in -1..=1 {
                let tx = self.x + dx;
                let ty = self.y + dy;
                let tz = world.get_surface_z(tx, ty);
                if let Some(t) = world.get_tile(tx, ty, tz) {
                    if t.kind == TileKind::WaterShallow
                        || t.kind == TileKind::WaterDeep
                        || t.biome == BiomeKind::River
                    {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn drink_water(&mut self, world: &World) -> bool {
        if self.can_drink_water(world) {
            self.thirst = (self.thirst + 45.0).min(self.max_thirst);
            self.health = (self.health + 5.0).min(self.max_health);
            true
        } else {
            false
        }
    }

    pub fn get_attack_damage(&self) -> f32 {
        if let Some(weapon) = self.inventory.equipment.main_hand {
            weapon.weapon_damage()
        } else {
            4.0
        }
    }

    pub fn get_defense(&self) -> f32 {
        let mut def = 0.0;
        if let Some(chest) = self.inventory.equipment.chest {
            def += chest.armor_defense();
        }
        def
    }

    pub fn take_damage(&mut self, incoming_dmg: f32) -> f32 {
        let def = self.get_defense();
        // Each point of defense absorbs 1% up to 60%
        let reduction = (def * 0.01).min(0.60);
        let actual_dmg = (incoming_dmg * (1.0 - reduction)).max(1.0);
        self.health = (self.health - actual_dmg).max(0.0);
        actual_dmg
    }

    pub fn has_ranged_weapon(&self) -> bool {
        self.inventory.equipment.main_hand == Some(ItemType::HuntingBow)
    }

    pub fn has_ammo(&self) -> bool {
        self.inventory.count_item(ItemType::FlintArrow) > 0
    }

    pub fn has_hatchet(&self) -> bool {
        self.inventory.equipment.main_hand == Some(ItemType::StoneHatchet)
            || self.inventory.has_item(ItemType::StoneHatchet, 1)
    }

    pub fn has_pickaxe(&self) -> bool {
        self.inventory.equipment.main_hand == Some(ItemType::StonePickaxe)
            || self.inventory.has_item(ItemType::StonePickaxe, 1)
    }

    pub fn has_shovel(&self) -> bool {
        self.inventory.equipment.main_hand == Some(ItemType::StoneShovel)
            || self.inventory.has_item(ItemType::StoneShovel, 1)
    }
}
