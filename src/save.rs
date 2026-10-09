use crate::entity::Player;
use crate::world::tile::BiomeKind;
use crate::world::World;
use serde::{Deserialize, Serialize};
#[cfg(not(target_arch = "wasm32"))]
use std::fs;
#[cfg(not(target_arch = "wasm32"))]
use std::path::Path;

#[allow(dead_code)]
pub const SAVE_FILENAME: &str = "savegame.json";

#[cfg(target_arch = "wasm32")]
unsafe extern "C" {
    fn web_save_game(ptr: *const u8, len: usize);
    fn web_load_game_len() -> i32;
    fn web_load_game_data(ptr: *mut u8);
    fn web_save_exists() -> i32;
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SaveGame {
    // Continental location
    pub overworld_seed: u32,
    pub ow_current_x: usize,
    pub ow_current_y: usize,
    pub discovered_regions: Vec<(usize, usize)>,

    // Local stage location (Escenario del PJ)
    pub player_x: i32,
    pub player_y: i32,
    pub player_z: i32,
    pub current_biome: BiomeKind,

    // Survival vitals
    pub health: f32,
    pub hunger: f32,
    pub thirst: f32,
    pub energy: f32,

    // Inventory and equipment
    #[serde(default)]
    pub inventory: Option<crate::entity::Inventory>,

    // Simulation time
    pub time_of_day: f32,
    pub saved_at_display: String,
}

impl SaveGame {
    pub fn create_from_state(
        world: &World,
        player: &Player,
        ow_seed: u32,
        ow_x: usize,
        ow_y: usize,
        discovered: Vec<(usize, usize)>,
    ) -> Self {
        Self {
            overworld_seed: ow_seed,
            ow_current_x: ow_x,
            ow_current_y: ow_y,
            discovered_regions: discovered,
            player_x: player.x,
            player_y: player.y,
            player_z: player.z,
            current_biome: world.primary_biome,
            health: player.health,
            hunger: player.hunger,
            thirst: player.thirst,
            energy: player.energy,
            inventory: Some(player.inventory.clone()),
            time_of_day: world.time_of_day,
            saved_at_display: world.formatted_time(),
        }
    }

    pub fn save(&self) -> Result<(), String> {
        let json_str = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize savegame: {}", e))?;

        #[cfg(not(target_arch = "wasm32"))]
        {
            fs::write(SAVE_FILENAME, json_str)
                .map_err(|e| format!("Failed to write savegame file: {}", e))?;
            Ok(())
        }

        #[cfg(target_arch = "wasm32")]
        {
            let bytes = json_str.as_bytes();
            unsafe {
                web_save_game(bytes.as_ptr(), bytes.len());
            }
            Ok(())
        }
    }

    pub fn load() -> Option<Self> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            if !Path::new(SAVE_FILENAME).exists() {
                return None;
            }
            let content = fs::read_to_string(SAVE_FILENAME).ok()?;
            serde_json::from_str(&content).ok()
        }

        #[cfg(target_arch = "wasm32")]
        {
            let len = unsafe { web_load_game_len() };
            if len <= 0 {
                return None;
            }
            let mut buf = vec![0u8; len as usize];
            unsafe {
                web_load_game_data(buf.as_mut_ptr());
            }
            let content = String::from_utf8(buf).ok()?;
            serde_json::from_str(&content).ok()
        }
    }

    pub fn exists() -> bool {
        #[cfg(not(target_arch = "wasm32"))]
        {
            Path::new(SAVE_FILENAME).exists()
        }

        #[cfg(target_arch = "wasm32")]
        {
            unsafe { web_save_exists() == 1 }
        }
    }
}
