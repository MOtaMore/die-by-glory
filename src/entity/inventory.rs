use crate::entity::item::{EquipSlot, ItemStack, ItemType};
use serde::{Deserialize, Serialize};

pub const INVENTORY_CAPACITY: usize = 20;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Equipment {
    pub main_hand: Option<ItemType>,
    pub off_hand: Option<ItemType>,
    pub head: Option<ItemType>,
    pub chest: Option<ItemType>,
    pub legs: Option<ItemType>,
}

impl Equipment {
    pub fn get(&self, slot: EquipSlot) -> Option<ItemType> {
        match slot {
            EquipSlot::MainHand => self.main_hand,
            EquipSlot::OffHand => self.off_hand,
            EquipSlot::Head => self.head,
            EquipSlot::Chest => self.chest,
            EquipSlot::Legs => self.legs,
        }
    }

    pub fn set(&mut self, slot: EquipSlot, item: Option<ItemType>) {
        match slot {
            EquipSlot::MainHand => self.main_hand = item,
            EquipSlot::OffHand => self.off_hand = item,
            EquipSlot::Head => self.head = item,
            EquipSlot::Chest => self.chest = item,
            EquipSlot::Legs => self.legs = item,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Inventory {
    pub slots: Vec<Option<ItemStack>>,
    pub equipment: Equipment,
    // 2x2 Minecraft style crafting grid
    pub crafting_grid: [Option<ItemStack>; 4],
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            slots: vec![None; INVENTORY_CAPACITY],
            equipment: Equipment::default(),
            crafting_grid: [None, None, None, None],
        }
    }

    /// Starter survival kit for new game
    pub fn with_starter_items() -> Self {
        let mut inv = Self::new();
        inv.add_item(ItemType::StoneHatchet, 1);
        inv.add_item(ItemType::StonePickaxe, 1);
        inv.add_item(ItemType::StoneShovel, 1);
        inv.add_item(ItemType::TallGrass, 4);
        inv.add_item(ItemType::Stick, 4);
        inv.add_item(ItemType::SmallStone, 3);
        inv.equip_from_slot(0); // Equips Stone Hatchet in Main Hand
        inv
    }

    pub fn count_item(&self, item: ItemType) -> u32 {
        self.slots
            .iter()
            .flatten()
            .filter(|s| s.item == item)
            .map(|s| s.count)
            .sum()
    }

    pub fn has_item(&self, item: ItemType, count: u32) -> bool {
        self.count_item(item) >= count
    }

    pub fn add_item(&mut self, item: ItemType, mut count: u32) -> u32 {
        let max_stack = item.max_stack();

        // 1. First add to existing incomplete stacks
        for slot in self.slots.iter_mut().flatten() {
            if slot.item == item && slot.count < max_stack {
                let can_add = (max_stack - slot.count).min(count);
                slot.count += can_add;
                count -= can_add;
                if count == 0 {
                    return 0;
                }
            }
        }

        // 2. Add to empty slots
        for slot in self.slots.iter_mut() {
            if slot.is_none() {
                let add_now = count.min(max_stack);
                *slot = Some(ItemStack::new(item, add_now));
                count -= add_now;
                if count == 0 {
                    return 0;
                }
            }
        }

        count // Return leftover count if inventory full
    }

    pub fn remove_item(&mut self, item: ItemType, mut count: u32) -> bool {
        if !self.has_item(item, count) {
            return false;
        }

        for slot in self.slots.iter_mut() {
            if let Some(s) = slot {
                if s.item == item {
                    if s.count <= count {
                        count -= s.count;
                        *slot = None;
                    } else {
                        s.count -= count;
                        count = 0;
                    }
                    if count == 0 {
                        return true;
                    }
                }
            }
        }

        count == 0
    }

    pub fn equip_from_slot(&mut self, slot_idx: usize) -> bool {
        if slot_idx >= self.slots.len() {
            return false;
        }

        let item_stack = match self.slots[slot_idx] {
            Some(ref s) => s.clone(),
            None => return false,
        };

        let target_slot = match item_stack.item.equip_slot() {
            Some(s) => s,
            None => return false,
        };

        let previous_equipped = self.equipment.get(target_slot);

        // Equip the item
        self.equipment.set(target_slot, Some(item_stack.item));

        // Decrease count or remove from slot
        if item_stack.count > 1 {
            if let Some(ref mut s) = self.slots[slot_idx] {
                s.count -= 1;
            }
            if let Some(prev) = previous_equipped {
                self.add_item(prev, 1);
            }
        } else {
            self.slots[slot_idx] = previous_equipped.map(|p| ItemStack::new(p, 1));
        }

        true
    }

    pub fn unequip(&mut self, slot: EquipSlot) -> bool {
        if let Some(item) = self.equipment.get(slot) {
            let leftover = self.add_item(item, 1);
            if leftover == 0 {
                self.equipment.set(slot, None);
                return true;
            }
        }
        false
    }

    pub fn drop_from_slot(&mut self, slot_idx: usize) -> Option<ItemStack> {
        if slot_idx < self.slots.len() {
            self.slots[slot_idx].take()
        } else {
            None
        }
    }

    pub fn can_craft(&self, recipe: &CraftingRecipe) -> bool {
        for (ing_type, ing_count) in &recipe.ingredients {
            if !self.has_item(*ing_type, *ing_count) {
                return false;
            }
        }
        true
    }

    pub fn craft(&mut self, recipe: &CraftingRecipe) -> bool {
        if !self.can_craft(recipe) {
            return false;
        }

        // Deduct materials
        for (ing_type, ing_count) in &recipe.ingredients {
            self.remove_item(*ing_type, *ing_count);
        }

        // Add crafted result
        self.add_item(recipe.result.item, recipe.result.count);
        for extra in &recipe.extra_results {
            self.add_item(extra.item, extra.count);
        }
        true
    }

    #[allow(dead_code)]
    pub fn consume_item(&mut self, item: ItemType) -> Option<(f32, f32)> {
        match item {
            ItemType::PrimitiveBandage => {
                if self.remove_item(item, 1) {
                    Some((25.0, 0.0))
                } else {
                    None
                }
            }
            ItemType::RawMeat => {
                if self.remove_item(item, 1) {
                    Some((5.0, 18.0))
                } else {
                    None
                }
            }
            ItemType::CookedMeat => {
                if self.remove_item(item, 1) {
                    Some((15.0, 40.0))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn butcher_carcass(&mut self, has_knife: bool) -> (u32, u32, u32) {
        if self.remove_item(ItemType::AnimalCarcass, 1) {
            let (leather, bones, meat) = if has_knife { (3, 3, 3) } else { (1, 2, 1) };
            self.add_item(ItemType::Leather, leather);
            self.add_item(ItemType::Bone, bones);
            self.add_item(ItemType::RawMeat, meat);
            (leather, bones, meat)
        } else {
            (0, 0, 0)
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CraftingRecipe {
    pub name: &'static str,
    pub result: ItemStack,
    pub extra_results: Vec<ItemStack>,
    pub ingredients: Vec<(ItemType, u32)>,
    pub description: &'static str,
}

pub fn get_all_recipes() -> Vec<CraftingRecipe> {
    vec![
        CraftingRecipe {
            name: "Plant Fiber Cord",
            result: ItemStack::new(ItemType::FiberCord, 1),
            extra_results: Vec::new(),
            ingredients: vec![(ItemType::TallGrass, 2)],
            description: "Twist 2 wild grass stalks into tough cordage.",
        },
        CraftingRecipe {
            name: "Herbal Bandage",
            result: ItemStack::new(ItemType::PrimitiveBandage, 1),
            extra_results: Vec::new(),
            ingredients: vec![(ItemType::TallGrass, 2)],
            description: "Crush soothing wild herbs to treat wounds (+25 HP).",
        },
        CraftingRecipe {
            name: "Flint Knife",
            result: ItemStack::new(ItemType::FlintKnife, 1),
            extra_results: Vec::new(),
            ingredients: vec![
                (ItemType::Stick, 1),
                (ItemType::Flint, 1),
                (ItemType::FiberCord, 1),
            ],
            description: "Knapped flint blade bound to stick. [Main Hand Tool/Weapon +8 Atk].",
        },
        CraftingRecipe {
            name: "Stone Hatchet",
            result: ItemStack::new(ItemType::StoneHatchet, 1),
            extra_results: Vec::new(),
            ingredients: vec![
                (ItemType::Stick, 2),
                (ItemType::SmallStone, 1),
                (ItemType::FiberCord, 1),
            ],
            description: "Heavy river stone bound to shaft. [Main Hand Tool +10 Atk].",
        },
        CraftingRecipe {
            name: "Stone Pickaxe",
            result: ItemStack::new(ItemType::StonePickaxe, 1),
            extra_results: Vec::new(),
            ingredients: vec![
                (ItemType::Stick, 2),
                (ItemType::SmallStone, 1),
                (ItemType::FiberCord, 1),
            ],
            description: "Chiseled stone pick for mining rocks and minerals. [Main Hand +7 Atk].",
        },
        CraftingRecipe {
            name: "Stone Shovel",
            result: ItemStack::new(ItemType::StoneShovel, 1),
            extra_results: Vec::new(),
            ingredients: vec![
                (ItemType::Stick, 2),
                (ItemType::SmallStone, 1),
                (ItemType::FiberCord, 1),
            ],
            description: "Wide stone spade for digging soil and clay. [Main Hand +5 Atk].",
        },
        CraftingRecipe {
            name: "Hunting Bow",
            result: ItemStack::new(ItemType::HuntingBow, 1),
            extra_results: Vec::new(),
            ingredients: vec![
                (ItemType::Stick, 3),
                (ItemType::FiberCord, 2),
            ],
            description: "Curved bow of bent timber and fiber bowstring. Fires arrows up to 8m (+16 Ranged Atk).",
        },
        CraftingRecipe {
            name: "Flint Arrows (x4)",
            result: ItemStack::new(ItemType::FlintArrow, 4),
            extra_results: Vec::new(),
            ingredients: vec![
                (ItemType::Stick, 1),
                (ItemType::Flint, 1),
                (ItemType::TallGrass, 1),
            ],
            description: "Quiver bundle of 4 flint arrows fletched with dried grass stalks.",
        },
        CraftingRecipe {
            name: "Torches (x2)",
            result: ItemStack::new(ItemType::Torch, 2),
            extra_results: Vec::new(),
            ingredients: vec![
                (ItemType::Stick, 1),
                (ItemType::TallGrass, 2),
            ],
            description: "Pair of dry resinous torches. Lights up the darkness when held in Off Hand.",
        },
        CraftingRecipe {
            name: "Cooked Roasted Meat",
            result: ItemStack::new(ItemType::CookedMeat, 1),
            extra_results: Vec::new(),
            ingredients: vec![
                (ItemType::RawMeat, 1),
                (ItemType::Torch, 1),
            ],
            description: "Roast fresh game meat over a burning torch. Restores +40 Hunger and +15 Health.",
        },
        CraftingRecipe {
            name: "Skin & Butcher Carcass",
            result: ItemStack::new(ItemType::Leather, 2),
            extra_results: vec![
                ItemStack::new(ItemType::Bone, 3),
                ItemStack::new(ItemType::RawMeat, 2),
            ],
            ingredients: vec![(ItemType::AnimalCarcass, 1)],
            description: "Skin and butcher game carcass -> Harvests 2x Leather, 3x Bones, and 2x Raw Meat.",
        },
        CraftingRecipe {
            name: "Cured Leather Armor",
            result: ItemStack::new(ItemType::LeatherArmor, 1),
            extra_results: Vec::new(),
            ingredients: vec![
                (ItemType::Leather, 3),
                (ItemType::FiberCord, 2),
            ],
            description: "Tough cuirass sewn from treated animal hides. [Chest Armor +22 Defense].",
        },
        CraftingRecipe {
            name: "Bone Dagger",
            result: ItemStack::new(ItemType::FlintKnife, 1),
            extra_results: Vec::new(),
            ingredients: vec![
                (ItemType::Bone, 2),
                (ItemType::FiberCord, 1),
            ],
            description: "Carve sharp bone slivers into a stabbing hunting dagger (+8 Atk).",
        },
        CraftingRecipe {
            name: "Flint Firestarter",
            result: ItemStack::new(ItemType::Firestarter, 1),
            extra_results: Vec::new(),
            ingredients: vec![(ItemType::Flint, 1), (ItemType::SmallStone, 1)],
            description: "Strike flint against iron stone for sparks. [Off Hand].",
        },
        CraftingRecipe {
            name: "Woven Grass Cloak",
            result: ItemStack::new(ItemType::GrassCloak, 1),
            extra_results: Vec::new(),
            ingredients: vec![(ItemType::TallGrass, 4), (ItemType::FiberCord, 1)],
            description: "Dense braided straw armor. [Chest Armor +10 Defense].",
        },
        CraftingRecipe {
            name: "Wood Planks (x4)",
            result: ItemStack::new(ItemType::WoodPlanks, 4),
            extra_results: Vec::new(),
            ingredients: vec![(ItemType::WoodLog, 1)],
            description: "Saw 1 felled tree log into 4 wooden construction planks for building.",
        },
        CraftingRecipe {
            name: "Cobblestone Block (x1)",
            result: ItemStack::new(ItemType::Cobblestone, 1),
            extra_results: Vec::new(),
            ingredients: vec![(ItemType::SmallStone, 3)],
            description: "Chisel and mortar 3 small stones into a solid cobblestone building block.",
        },
        CraftingRecipe {
            name: "Sticks from Planks (x4)",
            result: ItemStack::new(ItemType::Stick, 4),
            extra_results: Vec::new(),
            ingredients: vec![(ItemType::WoodPlanks, 1)],
            description: "Whittle 1 wood plank into 4 sturdy tool handle sticks.",
        },
    ]
}
