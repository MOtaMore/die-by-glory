use crate::entity::creature::CreatureType;
use crate::entity::item::ItemType;
use crate::world::World;

#[derive(Clone, Debug)]
pub struct DetectedItem {
    #[allow(dead_code)]
    pub id: u64,
    pub item: ItemType,
    pub count: u32,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub distance: f32,
    pub direction: &'static str,
}

#[derive(Clone, Debug)]
pub struct DetectedCreature {
    #[allow(dead_code)]
    pub id: u64,
    pub kind: CreatureType,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub distance: f32,
    pub direction: &'static str,
    pub status: String,
    pub health: f32,
    #[allow(dead_code)]
    pub max_health: f32,
}

#[derive(Clone, Debug)]
pub struct RegionScannerInfo {
    pub biome_name: &'static str,
    pub elevation_z: i32,
    pub tile_name: &'static str,
    pub time_str: String,
    pub atmosphere: &'static str,
}

#[derive(Clone, Debug)]
pub struct Radar {
    pub vision_radius: f32,
    pub is_active: bool,
    pub is_list_visible: bool,
    pub detected_items: Vec<DetectedItem>,
    pub detected_creatures: Vec<DetectedCreature>,
    pub region_info: RegionScannerInfo,
    pub sweep_angle: f32,
}

impl Radar {
    pub fn new() -> Self {
        Self {
            vision_radius: 16.0,
            is_active: false,
            is_list_visible: true, // Visible by default so user immediately sees radar
            detected_items: Vec::new(),
            detected_creatures: Vec::new(),
            region_info: RegionScannerInfo {
                biome_name: "Unknown",
                elevation_z: 0,
                tile_name: "Ground",
                time_str: "12:00".to_string(),
                atmosphere: "Clear",
            },
            sweep_angle: 0.0,
        }
    }

    pub fn toggle_list_visibility(&mut self) {
        self.is_list_visible = !self.is_list_visible;
    }

    pub fn update(
        &mut self,
        dt: f32,
        player_x: i32,
        player_y: i32,
        player_z: i32,
        world: &World,
    ) {
        self.sweep_angle = (self.sweep_angle + dt * 3.5) % (std::f32::consts::TAU);

        // 1. Scan region info
        let biome_info = world.primary_biome.info();
        let current_tile = world
            .get_tile(player_x, player_y, player_z)
            .map(|t| t.name)
            .unwrap_or("Solid Ground");

        let atmosphere = if player_z > 15 {
            "High Altitude Winds"
        } else if player_z < -5 {
            "Subterranean Damp Air"
        } else if world.time_of_day > 6.0 && world.time_of_day < 18.0 {
            "Sunny Wild Frontier"
        } else {
            "Chilly Starry Night"
        };

        self.region_info = RegionScannerInfo {
            biome_name: biome_info.name,
            elevation_z: player_z,
            tile_name: current_tile,
            time_str: world.formatted_time(),
            atmosphere,
        };

        // 2. Scan items within vision radius
        self.detected_items.clear();
        for item in &world.ground_items {
            let dx = (item.x - player_x) as f32;
            let dy = (item.y - player_y) as f32;
            let dist = (dx * dx + dy * dy).sqrt();

            if dist <= self.vision_radius {
                let direction = calculate_direction(dx, dy);
                self.detected_items.push(DetectedItem {
                    id: item.id,
                    item: item.item,
                    count: item.count,
                    x: item.x,
                    y: item.y,
                    z: item.z,
                    distance: dist,
                    direction,
                });
            }
        }
        // Sort items by distance (closest first)
        self.detected_items
            .sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap_or(std::cmp::Ordering::Equal));

        // 3. Scan creatures within vision radius
        self.detected_creatures.clear();
        for creature in &world.creatures {
            if creature.health <= 0.0 {
                continue;
            }
            let dx = (creature.x - player_x) as f32;
            let dy = (creature.y - player_y) as f32;
            let dist = (dx * dx + dy * dy).sqrt();

            if dist <= self.vision_radius {
                let direction = calculate_direction(dx, dy);
                self.detected_creatures.push(DetectedCreature {
                    id: creature.id,
                    kind: creature.kind,
                    x: creature.x,
                    y: creature.y,
                    z: creature.z,
                    distance: dist,
                    direction,
                    status: creature.status_description(),
                    health: creature.health,
                    max_health: creature.max_health,
                });
            }
        }
        // Sort creatures by distance (closest first)
        self.detected_creatures
            .sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap_or(std::cmp::Ordering::Equal));

        // Radar activates whenever items or creatures are in the player's vision radius
        self.is_active = !self.detected_items.is_empty() || !self.detected_creatures.is_empty();
    }
}

fn calculate_direction(dx: f32, dy: f32) -> &'static str {
    if dx.abs() < 0.5 && dy.abs() < 0.5 {
        return "HERE";
    }

    let angle = dy.atan2(dx); // -PI to PI
    let deg = angle.to_degrees(); // -180 to 180

    if deg >= -22.5 && deg < 22.5 {
        "East [E]"
    } else if deg >= 22.5 && deg < 67.5 {
        "South-East [SE]"
    } else if deg >= 67.5 && deg < 112.5 {
        "South [S]"
    } else if deg >= 112.5 && deg < 157.5 {
        "South-West [SW]"
    } else if deg >= 157.5 || deg < -157.5 {
        "West [W]"
    } else if deg >= -157.5 && deg < -112.5 {
        "North-West [NW]"
    } else if deg >= -112.5 && deg < -67.5 {
        "North [N]"
    } else {
        "North-East [NE]"
    }
}
