use macroquad::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TileKind {
    Air,
    // Surface & Flora
    Grass,
    TallGrass,
    Wildflower,
    TreeTrunk,
    TreeCanopy,
    Foliage,
    WaterShallow,
    WaterDeep,
    Sand,
    Soil,
    // Mountain & Snow
    MountainRock,
    MountainPeak,
    Snow,
    // Desert
    DesertSand,
    DesertDune,
    Cactus,
    // Ocean & Coast
    OceanWater,
    BeachSand,
    PalmTree,
    // Subterranean
    Subsoil,
    Stone,
    Granite,
    CavernFloor,
    CavernMushroom,
    IronOre,
    CoalOre,
    Bedrock,
    // Constructed Blocks
    WoodPlanks,
    Cobblestone,
    // Atmosphere
    Cloud,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BiomeKind {
    Meadow,
    Forest,
    River,
    Riverbank,
    Mountain,
    Ocean,
    Beach,
    Desert,
    Cave,
    DeepRock,
    Sky,
}

#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub kind: TileKind,
    pub biome: BiomeKind,
    pub glyph: char,
    pub fg_color: Color,
    pub bg_color: Color,
    pub walkable: bool,
    pub move_cost: f32,
    #[allow(dead_code)]
    pub transparent: bool,
    pub name: &'static str,
}

impl Tile {
    pub fn air() -> Self {
        Self {
            kind: TileKind::Air,
            biome: BiomeKind::Sky,
            glyph: ' ',
            fg_color: Color::new(0.0, 0.0, 0.0, 0.0),
            bg_color: Color::new(0.0, 0.0, 0.0, 0.0),
            walkable: false,
            move_cost: 999.0,
            transparent: true,
            name: "Open Air",
        }
    }

    pub fn grass(variant: u8) -> Self {
        let (glyph, fg, bg) = match variant % 4 {
            0 => ('.', Color::new(0.35, 0.75, 0.25, 1.0), Color::new(0.06, 0.18, 0.08, 1.0)),
            1 => (',', Color::new(0.40, 0.80, 0.30, 1.0), Color::new(0.07, 0.19, 0.08, 1.0)),
            2 => ('\'', Color::new(0.45, 0.70, 0.20, 1.0), Color::new(0.05, 0.17, 0.07, 1.0)),
            _ => ('.', Color::new(0.30, 0.65, 0.22, 1.0), Color::new(0.05, 0.16, 0.07, 1.0)),
        };

        Self {
            kind: TileKind::Grass,
            biome: BiomeKind::Meadow,
            glyph,
            fg_color: fg,
            bg_color: bg,
            walkable: true,
            move_cost: 1.0,
            transparent: false,
            name: "Meadow Grass",
        }
    }

    pub fn tall_grass() -> Self {
        Self {
            kind: TileKind::TallGrass,
            biome: BiomeKind::Meadow,
            glyph: '"',
            fg_color: Color::new(0.50, 0.85, 0.25, 1.0),
            bg_color: Color::new(0.08, 0.22, 0.09, 1.0),
            walkable: true,
            move_cost: 1.2,
            transparent: false,
            name: "Tall Grass",
        }
    }

    pub fn wildflower(variant: u8) -> Self {
        let (glyph, fg) = match variant % 3 {
            0 => ('*', Color::new(0.95, 0.85, 0.20, 1.0)),
            1 => ('*', Color::new(0.95, 0.40, 0.60, 1.0)),
            _ => ('*', Color::new(0.40, 0.70, 0.95, 1.0)),
        };

        Self {
            kind: TileKind::Wildflower,
            biome: BiomeKind::Meadow,
            glyph,
            fg_color: fg,
            bg_color: Color::new(0.08, 0.20, 0.09, 1.0),
            walkable: true,
            move_cost: 1.0,
            transparent: false,
            name: "Wildflower",
        }
    }

    pub fn tree_trunk() -> Self {
        Self {
            kind: TileKind::TreeTrunk,
            biome: BiomeKind::Forest,
            glyph: 'T',
            fg_color: Color::new(0.70, 0.45, 0.25, 1.0),
            bg_color: Color::new(0.20, 0.12, 0.06, 1.0),
            walkable: false,
            move_cost: 999.0,
            transparent: false,
            name: "Tree Trunk",
        }
    }

    pub fn tree_canopy(variant: u8) -> Self {
        let (glyph, fg) = match variant % 3 {
            0 => ('♣', Color::new(0.18, 0.65, 0.28, 1.0)),
            1 => ('♠', Color::new(0.15, 0.55, 0.22, 1.0)),
            _ => ('%', Color::new(0.22, 0.70, 0.32, 1.0)),
        };

        Self {
            kind: TileKind::TreeCanopy,
            biome: BiomeKind::Forest,
            glyph,
            fg_color: fg,
            bg_color: Color::new(0.04, 0.18, 0.08, 0.85),
            walkable: false,
            move_cost: 999.0,
            transparent: false,
            name: "Tree Canopy",
        }
    }

    pub fn foliage() -> Self {
        Self {
            kind: TileKind::Foliage,
            biome: BiomeKind::Forest,
            glyph: '&',
            fg_color: Color::new(0.25, 0.60, 0.25, 1.0),
            bg_color: Color::new(0.06, 0.16, 0.07, 1.0),
            walkable: true,
            move_cost: 1.4,
            transparent: false,
            name: "Forest Foliage",
        }
    }

    pub fn soil() -> Self {
        Self {
            kind: TileKind::Soil,
            biome: BiomeKind::Forest,
            glyph: '.',
            fg_color: Color::new(0.55, 0.40, 0.25, 1.0),
            bg_color: Color::new(0.18, 0.12, 0.08, 1.0),
            walkable: true,
            move_cost: 1.0,
            transparent: false,
            name: "Forest Soil",
        }
    }

    pub fn water_shallow() -> Self {
        Self {
            kind: TileKind::WaterShallow,
            biome: BiomeKind::River,
            glyph: '~',
            fg_color: Color::new(0.40, 0.75, 0.95, 1.0),
            bg_color: Color::new(0.08, 0.25, 0.45, 1.0),
            walkable: true,
            move_cost: 2.5,
            transparent: false,
            name: "Shallow River Ford",
        }
    }

    pub fn water_deep() -> Self {
        Self {
            kind: TileKind::WaterDeep,
            biome: BiomeKind::River,
            glyph: '≈',
            fg_color: Color::new(0.25, 0.55, 0.85, 1.0),
            bg_color: Color::new(0.04, 0.15, 0.35, 1.0),
            walkable: false,
            move_cost: 999.0,
            transparent: false,
            name: "Deep River",
        }
    }

    pub fn sand() -> Self {
        Self {
            kind: TileKind::Sand,
            biome: BiomeKind::Riverbank,
            glyph: '.',
            fg_color: Color::new(0.85, 0.75, 0.50, 1.0),
            bg_color: Color::new(0.30, 0.25, 0.15, 1.0),
            walkable: true,
            move_cost: 1.1,
            transparent: false,
            name: "River Sandbank",
        }
    }

    // Mountain & Snow tiles
    pub fn mountain_rock() -> Self {
        Self {
            kind: TileKind::MountainRock,
            biome: BiomeKind::Mountain,
            glyph: '▲',
            fg_color: Color::new(0.70, 0.70, 0.75, 1.0),
            bg_color: Color::new(0.25, 0.25, 0.30, 1.0),
            walkable: true,
            move_cost: 1.8,
            transparent: false,
            name: "Mountain Rock Slope",
        }
    }

    pub fn mountain_peak() -> Self {
        Self {
            kind: TileKind::MountainPeak,
            biome: BiomeKind::Mountain,
            glyph: '^',
            fg_color: Color::new(0.85, 0.85, 0.92, 1.0),
            bg_color: Color::new(0.35, 0.35, 0.42, 1.0),
            walkable: false,
            move_cost: 999.0,
            transparent: false,
            name: "Jagged Peak Cliff",
        }
    }

    pub fn snow() -> Self {
        Self {
            kind: TileKind::Snow,
            biome: BiomeKind::Mountain,
            glyph: '*',
            fg_color: Color::new(0.95, 0.95, 1.0, 1.0),
            bg_color: Color::new(0.55, 0.60, 0.70, 1.0),
            walkable: true,
            move_cost: 1.6,
            transparent: false,
            name: "Crisp Alpine Snow",
        }
    }

    // Desert tiles
    pub fn desert_sand() -> Self {
        Self {
            kind: TileKind::DesertSand,
            biome: BiomeKind::Desert,
            glyph: '.',
            fg_color: Color::new(0.95, 0.80, 0.40, 1.0),
            bg_color: Color::new(0.40, 0.30, 0.15, 1.0),
            walkable: true,
            move_cost: 1.2,
            transparent: false,
            name: "Desert Sand",
        }
    }

    pub fn desert_dune() -> Self {
        Self {
            kind: TileKind::DesertDune,
            biome: BiomeKind::Desert,
            glyph: '~',
            fg_color: Color::new(1.0, 0.85, 0.45, 1.0),
            bg_color: Color::new(0.45, 0.35, 0.18, 1.0),
            walkable: true,
            move_cost: 1.5,
            transparent: false,
            name: "Windswept Sand Dune",
        }
    }

    pub fn cactus() -> Self {
        Self {
            kind: TileKind::Cactus,
            biome: BiomeKind::Desert,
            glyph: 'ψ',
            fg_color: Color::new(0.35, 0.80, 0.35, 1.0),
            bg_color: Color::new(0.40, 0.30, 0.15, 1.0),
            walkable: false,
            move_cost: 999.0,
            transparent: false,
            name: "Saguaro Cactus",
        }
    }

    // Ocean & Beach tiles
    pub fn ocean_water() -> Self {
        Self {
            kind: TileKind::OceanWater,
            biome: BiomeKind::Ocean,
            glyph: '≈',
            fg_color: Color::new(0.20, 0.45, 0.80, 1.0),
            bg_color: Color::new(0.02, 0.08, 0.28, 1.0),
            walkable: false,
            move_cost: 999.0,
            transparent: false,
            name: "Deep Ocean Water",
        }
    }

    pub fn beach_sand() -> Self {
        Self {
            kind: TileKind::BeachSand,
            biome: BiomeKind::Beach,
            glyph: '.',
            fg_color: Color::new(0.95, 0.88, 0.60, 1.0),
            bg_color: Color::new(0.35, 0.30, 0.18, 1.0),
            walkable: true,
            move_cost: 1.0,
            transparent: false,
            name: "Coastal Beach Shore",
        }
    }

    pub fn palm_tree() -> Self {
        Self {
            kind: TileKind::PalmTree,
            biome: BiomeKind::Beach,
            glyph: 'T',
            fg_color: Color::new(0.40, 0.75, 0.25, 1.0),
            bg_color: Color::new(0.35, 0.30, 0.18, 1.0),
            walkable: false,
            move_cost: 999.0,
            transparent: false,
            name: "Coastal Palm Tree",
        }
    }

    // Subterranean tiles
    pub fn subsoil() -> Self {
        Self {
            kind: TileKind::Subsoil,
            biome: BiomeKind::Cave,
            glyph: '░',
            fg_color: Color::new(0.50, 0.35, 0.25, 1.0),
            bg_color: Color::new(0.18, 0.12, 0.08, 1.0),
            walkable: false,
            move_cost: 999.0,
            transparent: false,
            name: "Clay Subsoil",
        }
    }

    pub fn stone() -> Self {
        Self {
            kind: TileKind::Stone,
            biome: BiomeKind::DeepRock,
            glyph: '#',
            fg_color: Color::new(0.55, 0.55, 0.60, 1.0),
            bg_color: Color::new(0.15, 0.15, 0.18, 1.0),
            walkable: false,
            move_cost: 999.0,
            transparent: false,
            name: "Underground Stone",
        }
    }

    pub fn granite() -> Self {
        Self {
            kind: TileKind::Granite,
            biome: BiomeKind::DeepRock,
            glyph: '▒',
            fg_color: Color::new(0.40, 0.40, 0.48, 1.0),
            bg_color: Color::new(0.10, 0.10, 0.14, 1.0),
            walkable: false,
            move_cost: 999.0,
            transparent: false,
            name: "Deep Granite",
        }
    }

    pub fn cavern_floor() -> Self {
        Self {
            kind: TileKind::CavernFloor,
            biome: BiomeKind::Cave,
            glyph: '.',
            fg_color: Color::new(0.45, 0.45, 0.50, 1.0),
            bg_color: Color::new(0.10, 0.10, 0.12, 1.0),
            walkable: true,
            move_cost: 1.0,
            transparent: false,
            name: "Cavern Floor",
        }
    }

    pub fn cavern_mushroom() -> Self {
        Self {
            kind: TileKind::CavernMushroom,
            biome: BiomeKind::Cave,
            glyph: 'ψ',
            fg_color: Color::new(0.30, 0.90, 0.70, 1.0),
            bg_color: Color::new(0.08, 0.15, 0.14, 1.0),
            walkable: true,
            move_cost: 1.0,
            transparent: false,
            name: "Bioluminescent Mushroom",
        }
    }

    pub fn iron_ore() -> Self {
        Self {
            kind: TileKind::IronOre,
            biome: BiomeKind::DeepRock,
            glyph: '$',
            fg_color: Color::new(0.80, 0.50, 0.35, 1.0),
            bg_color: Color::new(0.20, 0.14, 0.12, 1.0),
            walkable: false,
            move_cost: 999.0,
            transparent: false,
            name: "Iron Ore Deposit",
        }
    }

    pub fn coal_ore() -> Self {
        Self {
            kind: TileKind::CoalOre,
            biome: BiomeKind::DeepRock,
            glyph: '¤',
            fg_color: Color::new(0.30, 0.30, 0.30, 1.0),
            bg_color: Color::new(0.08, 0.08, 0.08, 1.0),
            walkable: false,
            move_cost: 999.0,
            transparent: false,
            name: "Coal Ore Seam",
        }
    }

    pub fn bedrock() -> Self {
        Self {
            kind: TileKind::Bedrock,
            biome: BiomeKind::DeepRock,
            glyph: '▓',
            fg_color: Color::new(0.25, 0.25, 0.30, 1.0),
            bg_color: Color::new(0.05, 0.05, 0.08, 1.0),
            walkable: false,
            move_cost: 999.0,
            transparent: false,
            name: "Impenetrable Bedrock",
        }
    }

    pub fn cloud() -> Self {
        Self {
            kind: TileKind::Cloud,
            biome: BiomeKind::Sky,
            glyph: '~',
            fg_color: Color::new(0.90, 0.95, 1.0, 0.75),
            bg_color: Color::new(0.60, 0.75, 0.90, 0.35),
            walkable: false,
            move_cost: 999.0,
            transparent: false,
            name: "Stratified Cloud",
        }
    }

    pub fn wood_planks() -> Self {
        Self {
            kind: TileKind::WoodPlanks,
            biome: BiomeKind::Forest,
            glyph: '#',
            fg_color: Color::new(0.85, 0.60, 0.35, 1.0),
            bg_color: Color::new(0.35, 0.22, 0.12, 1.0),
            walkable: true,
            move_cost: 1.0,
            transparent: false,
            name: "Wood Planks",
        }
    }

    pub fn cobblestone() -> Self {
        Self {
            kind: TileKind::Cobblestone,
            biome: BiomeKind::Mountain,
            glyph: '#',
            fg_color: Color::new(0.75, 0.75, 0.80, 1.0),
            bg_color: Color::new(0.25, 0.25, 0.28, 1.0),
            walkable: true,
            move_cost: 1.0,
            transparent: false,
            name: "Cobblestone Block",
        }
    }
}
