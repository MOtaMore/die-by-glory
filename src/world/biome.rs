use crate::world::tile::BiomeKind;
use macroquad::prelude::*;

pub struct BiomeInfo {
    pub name: &'static str,
    pub description: &'static str,
    pub ambient_color: Color,
    pub glyph: char,
}

impl BiomeKind {
    pub fn info(&self) -> BiomeInfo {
        match self {
            BiomeKind::Meadow => BiomeInfo {
                name: "Open Meadow",
                description: "Vibrant lush grasslands and wildflowers with gentle, easily traversed plains.",
                ambient_color: Color::new(0.35, 0.70, 0.25, 1.0),
                glyph: '.',
            },
            BiomeKind::Forest => BiomeInfo {
                name: "Ancient Forest",
                description: "Towering ancient oaks and thick brush offering shelter, firewood, and shade.",
                ambient_color: Color::new(0.18, 0.55, 0.25, 1.0),
                glyph: '♣',
            },
            BiomeKind::River => BiomeInfo {
                name: "Crystal River",
                description: "Flowing freshwater artery vital for quenching thirst. Shallow fords slow movement.",
                ambient_color: Color::new(0.30, 0.65, 0.90, 1.0),
                glyph: '~',
            },
            BiomeKind::Riverbank => BiomeInfo {
                name: "Riverbank Sand",
                description: "Moist sandy shores and soft riverbanks bordering the freshwater stream.",
                ambient_color: Color::new(0.80, 0.70, 0.45, 1.0),
                glyph: '.',
            },
            BiomeKind::Mountain => BiomeInfo {
                name: "Rugged Mountains",
                description: "Steep granite bluffs, windblown scree, and cold snow-capped summits.",
                ambient_color: Color::new(0.70, 0.75, 0.85, 1.0),
                glyph: '▲',
            },
            BiomeKind::Ocean => BiomeInfo {
                name: "Deep Ocean",
                description: "Endless saltwater depths with crashing tides surrounding the continents.",
                ambient_color: Color::new(0.15, 0.35, 0.75, 1.0),
                glyph: '≈',
            },
            BiomeKind::Beach => BiomeInfo {
                name: "Coastal Beach",
                description: "Warm golden sands where ocean surf meets the edges of landmasses.",
                ambient_color: Color::new(0.90, 0.82, 0.55, 1.0),
                glyph: '.',
            },
            BiomeKind::Desert => BiomeInfo {
                name: "Sunscorched Desert",
                description: "Arid shifting sand dunes, tall cacti, and extreme dry heat.",
                ambient_color: Color::new(0.95, 0.75, 0.35, 1.0),
                glyph: '~',
            },
            BiomeKind::Cave => BiomeInfo {
                name: "Underground Caverns",
                description: "Subterranean grottos filled with glowing fungi and hidden subterranean pools.",
                ambient_color: Color::new(0.40, 0.50, 0.65, 1.0),
                glyph: '#',
            },
            BiomeKind::DeepRock => BiomeInfo {
                name: "Rocky Depths",
                description: "Dense bedrock, granite crust, and rich mineral ore deposits deep beneath.",
                ambient_color: Color::new(0.35, 0.35, 0.40, 1.0),
                glyph: '▒',
            },
            BiomeKind::Sky => BiomeInfo {
                name: "Open Sky",
                description: "Brisk atmospheric heights with drifting cloud formations and open air.",
                ambient_color: Color::new(0.65, 0.80, 0.95, 1.0),
                glyph: ' ',
            },
        }
    }
}
