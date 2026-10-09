use crate::world::World;
use macroquad::prelude::Color;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum CreatureType {
    Deer,
    Rabbit,
    Boar,
    MountainGoat,
}

impl CreatureType {
    pub fn name(&self) -> &'static str {
        match self {
            CreatureType::Deer => "Wild Deer",
            CreatureType::Rabbit => "Hare / Rabbit",
            CreatureType::Boar => "Forest Boar",
            CreatureType::MountainGoat => "Mountain Goat",
        }
    }

    pub fn glyph(&self) -> char {
        match self {
            CreatureType::Deer => 'd',
            CreatureType::Rabbit => 'r',
            CreatureType::Boar => 'b',
            CreatureType::MountainGoat => 'g',
        }
    }

    pub fn color(&self) -> Color {
        match self {
            CreatureType::Deer => Color::new(0.85, 0.65, 0.35, 1.0),
            CreatureType::Rabbit => Color::new(0.95, 0.95, 0.95, 1.0),
            CreatureType::Boar => Color::new(0.55, 0.38, 0.28, 1.0),
            CreatureType::MountainGoat => Color::new(0.90, 0.88, 0.78, 1.0),
        }
    }

    pub fn status_text(&self) -> &'static str {
        match self {
            CreatureType::Deer => "Grazing peacefully in meadow",
            CreatureType::Rabbit => "Alert, foraging among shrubs",
            CreatureType::Boar => "Rooting for acorns in dirt",
            CreatureType::MountainGoat => "Leaping across rocky crags",
        }
    }

    pub fn default_max_health(&self) -> f32 {
        match self {
            CreatureType::Deer => 30.0,
            CreatureType::Rabbit => 12.0,
            CreatureType::Boar => 45.0,
            CreatureType::MountainGoat => 35.0,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Creature {
    pub id: u64,
    pub kind: CreatureType,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub health: f32,
    pub max_health: f32,
    pub move_cooldown: f32,
    pub is_alert: bool,
}

impl Creature {
    pub fn new(id: u64, kind: CreatureType, x: i32, y: i32, z: i32) -> Self {
        let max_hp = kind.default_max_health();
        Self {
            id,
            kind,
            x,
            y,
            z,
            health: max_hp,
            max_health: max_hp,
            move_cooldown: macroquad::rand::gen_range(2.0, 5.0),
            is_alert: false,
        }
    }

    pub fn take_damage(&mut self, dmg: f32) -> bool {
        self.health -= dmg;
        self.is_alert = true;
        self.health <= 0.0
    }

    #[allow(dead_code)]
    pub fn is_alive(&self) -> bool {
        self.health > 0.0
    }

    pub fn status_description(&self) -> String {
        if self.health <= 0.0 {
            "Dead / Carcass".to_string()
        } else if self.is_alert && self.kind == CreatureType::Boar {
            format!("ENRAGED (HP {:.0}/{:.0})", self.health, self.max_health)
        } else if self.is_alert {
            format!("Fleeing (HP {:.0}/{:.0})", self.health, self.max_health)
        } else if self.health < self.max_health {
            format!("Wounded (HP {:.0}/{:.0})", self.health, self.max_health)
        } else {
            self.kind.status_text().to_string()
        }
    }

    #[allow(dead_code)]
    pub fn update(&mut self, dt: f32, world: &World) {
        if self.move_cooldown > 0.0 {
            self.move_cooldown -= dt;
            return;
        }

        // Reset cooldown between 2.0 and 5.0 seconds
        self.move_cooldown = macroquad::rand::gen_range(2.0, 5.0);

        // Try gentle random step in one of 4 cardinal directions or stay
        let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0), (0, 0)];
        let choice = macroquad::rand::gen_range(0, dirs.len());
        let (dx, dy) = dirs[choice];

        if dx == 0 && dy == 0 {
            return;
        }

        let nx = self.x + dx;
        let ny = self.y + dy;
        let nz = world.get_surface_z(nx, ny);

        // Only move if difference in elevation is <= 1 (gentle slope) and walkable
        if (nz - self.z).abs() <= 1 {
            if let Some(tile) = world.get_tile(nx, ny, nz) {
                if tile.walkable {
                    self.x = nx;
                    self.y = ny;
                    self.z = nz;
                }
            }
        }
    }
}
