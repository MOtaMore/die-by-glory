use macroquad::prelude::Color;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum ItemType {
    // 4 Base requested items
    TallGrass,
    Stick,
    Flint,
    SmallStone,

    // Craftable materials & tools
    FiberCord,
    FlintKnife,
    StoneHatchet,
    StonePickaxe,
    StoneShovel,
    HuntingBow,
    FlintArrow,
    Torch,
    Firestarter,
    PrimitiveBandage,
    GrassCloak,
    LeatherArmor,

    // Hunting, fauna & butchering yields
    AnimalCarcass,
    RawMeat,
    CookedMeat,
    Leather,
    Bone,

    // Forestry, Quarrying & Construction Blocks
    WoodLog,
    WoodPlanks,
    Cobblestone,
    DirtBlock,
}

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EquipSlot {
    MainHand,
    OffHand,
    Head,
    Chest,
    Legs,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ItemCategory {
    Material,
    Tool,
    Weapon,
    Armor,
    Consumable,
}

impl ItemType {
    pub fn name(&self) -> &'static str {
        match self {
            ItemType::TallGrass => "Tall Grass",
            ItemType::Stick => "Stick",
            ItemType::Flint => "Flint",
            ItemType::SmallStone => "Small Stone",
            ItemType::FiberCord => "Plant Fiber Cord",
            ItemType::FlintKnife => "Flint Knife",
            ItemType::StoneHatchet => "Stone Hatchet",
            ItemType::StonePickaxe => "Stone Pickaxe",
            ItemType::StoneShovel => "Stone Shovel",
            ItemType::HuntingBow => "Hunting Bow",
            ItemType::FlintArrow => "Flint Arrow",
            ItemType::Torch => "Torch",
            ItemType::Firestarter => "Flint Firestarter",
            ItemType::PrimitiveBandage => "Herbal Bandage",
            ItemType::GrassCloak => "Woven Grass Cloak",
            ItemType::LeatherArmor => "Cured Leather Armor",
            ItemType::AnimalCarcass => "Animal Carcass",
            ItemType::RawMeat => "Raw Meat",
            ItemType::CookedMeat => "Cooked Roasted Meat",
            ItemType::Leather => "Animal Leather",
            ItemType::Bone => "Dense Bone",
            ItemType::WoodLog => "Wood Log",
            ItemType::WoodPlanks => "Wood Planks",
            ItemType::Cobblestone => "Cobblestone Block",
            ItemType::DirtBlock => "Dirt Block",
        }
    }

    pub fn glyph(&self) -> char {
        match self {
            ItemType::TallGrass => '\"',
            ItemType::Stick => '/',
            ItemType::Flint => '^',
            ItemType::SmallStone => '•',
            ItemType::FiberCord => '§',
            ItemType::FlintKnife => '†',
            ItemType::StoneHatchet => 'p',
            ItemType::StonePickaxe => 'T',
            ItemType::StoneShovel => '!',
            ItemType::HuntingBow => ')',
            ItemType::FlintArrow => '>',
            ItemType::Torch => 'i',
            ItemType::Firestarter => '*',
            ItemType::PrimitiveBandage => '+',
            ItemType::GrassCloak => '[',
            ItemType::LeatherArmor => ']',
            ItemType::AnimalCarcass => '%',
            ItemType::RawMeat => '~',
            ItemType::CookedMeat => '~',
            ItemType::Leather => '{',
            ItemType::Bone => 'l',
            ItemType::WoodLog => '=',
            ItemType::WoodPlanks => '#',
            ItemType::Cobblestone => '#',
            ItemType::DirtBlock => 'o',
        }
    }

    pub fn color(&self) -> Color {
        match self {
            ItemType::TallGrass => Color::new(0.40, 0.88, 0.35, 1.0),
            ItemType::Stick => Color::new(0.74, 0.48, 0.26, 1.0),
            ItemType::Flint => Color::new(0.55, 0.65, 0.78, 1.0),
            ItemType::SmallStone => Color::new(0.72, 0.74, 0.78, 1.0),
            ItemType::FiberCord => Color::new(0.85, 0.72, 0.35, 1.0),
            ItemType::FlintKnife => Color::new(0.40, 0.85, 0.95, 1.0),
            ItemType::StoneHatchet => Color::new(0.92, 0.75, 0.45, 1.0),
            ItemType::StonePickaxe => Color::new(0.68, 0.78, 0.88, 1.0),
            ItemType::StoneShovel => Color::new(0.75, 0.65, 0.52, 1.0),
            ItemType::HuntingBow => Color::new(0.88, 0.62, 0.32, 1.0),
            ItemType::FlintArrow => Color::new(0.82, 0.86, 0.92, 1.0),
            ItemType::Torch => Color::new(1.0, 0.72, 0.22, 1.0),
            ItemType::Firestarter => Color::new(1.0, 0.55, 0.20, 1.0),
            ItemType::PrimitiveBandage => Color::new(0.50, 0.95, 0.60, 1.0),
            ItemType::GrassCloak => Color::new(0.35, 0.80, 0.40, 1.0),
            ItemType::LeatherArmor => Color::new(0.75, 0.48, 0.28, 1.0),
            ItemType::AnimalCarcass => Color::new(0.80, 0.28, 0.25, 1.0),
            ItemType::RawMeat => Color::new(0.92, 0.35, 0.35, 1.0),
            ItemType::CookedMeat => Color::new(0.85, 0.52, 0.22, 1.0),
            ItemType::Leather => Color::new(0.78, 0.55, 0.35, 1.0),
            ItemType::Bone => Color::new(0.95, 0.93, 0.85, 1.0),
            ItemType::WoodLog => Color::new(0.70, 0.45, 0.25, 1.0),
            ItemType::WoodPlanks => Color::new(0.85, 0.60, 0.35, 1.0),
            ItemType::Cobblestone => Color::new(0.75, 0.75, 0.80, 1.0),
            ItemType::DirtBlock => Color::new(0.55, 0.40, 0.25, 1.0),
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            ItemType::TallGrass => "Wild fibrous stalks. Can be braided into strong cordage or bandages.",
            ItemType::Stick => "Sturdy fallen branch. Essential for tool handles, shafts, and campfires.",
            ItemType::Flint => "Razor-sharp mineral nodule. Strikes hot sparks and knaps into cutting blades.",
            ItemType::SmallStone => "Smooth, dense river pebble. Useful as a hammer stone or ballast.",
            ItemType::FiberCord => "Tightly woven plant twine. Essential binding material for crafting.",
            ItemType::FlintKnife => "Knapped flint blade tied to a stick. Excellent for hunting, cutting and skinning (+8 Atk).",
            ItemType::StoneHatchet => "Grooved river stone bound to a handle. Chops wood and breaks rocks (+10 Atk).",
            ItemType::StonePickaxe => "Chiseled stone pick bound to shaft. Mines bedrock and stone (+7 Atk).",
            ItemType::StoneShovel => "Broad stone spade bound to handle. Excavates earth, sand and clay (+5 Atk).",
            ItemType::HuntingBow => "Flexible timber staff strung with fiber cord. Fires arrows up to 8m (+16 Ranged Atk).",
            ItemType::FlintArrow => "Pointed flint arrowhead bound with grass fletching. Bow ammunition.",
            ItemType::Torch => "Resinous stick burning with warm embers. Illuminates the dark when held in Off-Hand.",
            ItemType::Firestarter => "Flint nodule paired with iron-rich stone. Strikes survival embers.",
            ItemType::PrimitiveBandage => "Antiseptic herb poultice wrapped in fibers. Restores 25 Health.",
            ItemType::GrassCloak => "Braided straw cloak worn over torso. Shields from wind (+10 Armor).",
            ItemType::LeatherArmor => "Reinforced tanned hide cuirass. Substantial protection from claws and bites (+22 Armor).",
            ItemType::AnimalCarcass => "Hunted game animal carcass. Skin with knife or tool [E] to harvest leather, bones, and raw meat.",
            ItemType::RawMeat => "Fresh game meat cut. Edible raw (+15 Hunger) or roast over a torch/fire.",
            ItemType::CookedMeat => "Savory roasted meat seared over flame. Restores +40 Hunger and heals +15 Health.",
            ItemType::Leather => "Cured animal hide. Heavy-duty material for stitching leather armor and gear.",
            ItemType::Bone => "Dense animal bone. Used for tools, spears, needles, and crafts.",
            ItemType::WoodLog => "Felled tree trunk chopped down with a hatchet. Can be sawed into 4x Wood Planks.",
            ItemType::WoodPlanks => "Sawed wooden construction planks. Place on terrain [C / Right-Click] to build wooden structures.",
            ItemType::Cobblestone => "Dense chiseled stone block quarried with a pickaxe. Place [V / Right-Click] to build stone walls/floors.",
            ItemType::DirtBlock => "Excavated soil clod dug with a shovel. Can be placed for landscaping or earthen mounds.",
        }
    }

    pub fn max_stack(&self) -> u32 {
        match self {
            ItemType::AnimalCarcass => 5,
            ItemType::Torch => 16,
            _ => match self.category() {
                ItemCategory::Tool | ItemCategory::Weapon | ItemCategory::Armor => 1,
                _ => 64,
            },
        }
    }

    pub fn category(&self) -> ItemCategory {
        match self {
            ItemType::TallGrass
            | ItemType::Stick
            | ItemType::Flint
            | ItemType::SmallStone
            | ItemType::FiberCord
            | ItemType::FlintArrow
            | ItemType::Leather
            | ItemType::Bone
            | ItemType::WoodLog
            | ItemType::WoodPlanks
            | ItemType::Cobblestone
            | ItemType::DirtBlock
            | ItemType::AnimalCarcass => ItemCategory::Material,
            ItemType::StoneHatchet | ItemType::StonePickaxe | ItemType::StoneShovel | ItemType::Torch | ItemType::Firestarter => ItemCategory::Tool,
            ItemType::FlintKnife | ItemType::HuntingBow => ItemCategory::Weapon,
            ItemType::PrimitiveBandage | ItemType::RawMeat | ItemType::CookedMeat => ItemCategory::Consumable,
            ItemType::GrassCloak | ItemType::LeatherArmor => ItemCategory::Armor,
        }
    }

    pub fn equip_slot(&self) -> Option<EquipSlot> {
        match self {
            ItemType::FlintKnife
            | ItemType::StoneHatchet
            | ItemType::StonePickaxe
            | ItemType::StoneShovel
            | ItemType::HuntingBow => Some(EquipSlot::MainHand),
            ItemType::Firestarter | ItemType::Torch | ItemType::WoodPlanks | ItemType::Cobblestone | ItemType::DirtBlock => Some(EquipSlot::OffHand),
            ItemType::GrassCloak | ItemType::LeatherArmor => Some(EquipSlot::Chest),
            _ => None,
        }
    }

    pub fn placeable_tile(&self) -> Option<crate::world::tile::TileKind> {
        match self {
            ItemType::WoodPlanks => Some(crate::world::tile::TileKind::WoodPlanks),
            ItemType::Cobblestone => Some(crate::world::tile::TileKind::Cobblestone),
            ItemType::DirtBlock => Some(crate::world::tile::TileKind::Soil),
            _ => None,
        }
    }

    pub fn weapon_damage(&self) -> f32 {
        match self {
            ItemType::HuntingBow => 16.0,
            ItemType::StoneHatchet => 12.0,
            ItemType::FlintKnife => 10.0,
            ItemType::StonePickaxe => 8.0,
            ItemType::StoneShovel => 6.0,
            _ => 4.0,
        }
    }

    pub fn armor_defense(&self) -> f32 {
        match self {
            ItemType::LeatherArmor => 22.0,
            ItemType::GrassCloak => 10.0,
            _ => 0.0,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ItemStack {
    pub item: ItemType,
    pub count: u32,
}

impl ItemStack {
    pub fn new(item: ItemType, count: u32) -> Self {
        Self { item, count }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GroundItem {
    pub id: u64,
    pub item: ItemType,
    pub count: u32,
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl GroundItem {
    pub fn new(id: u64, item: ItemType, count: u32, x: i32, y: i32, z: i32) -> Self {
        Self {
            id,
            item,
            count,
            x,
            y,
            z,
        }
    }
}
