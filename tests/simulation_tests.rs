#[cfg(test)]
mod tests {
    use die_by_glory::world::generator::{MAP_HEIGHT, MAP_WIDTH, Z_MAX, Z_MIN, Z_SURFACE};
    use die_by_glory::world::tile::{BiomeKind, TileKind};
    use die_by_glory::world::{Overworld, World};
    use die_by_glory::entity::Player;
    use die_by_glory::pathfinding::find_path;
    use die_by_glory::save::SaveGame;

    #[test]
    fn test_world_dimensions_and_layers() {
        let world = World::new(999, BiomeKind::Meadow);
        assert_eq!(Z_MIN, -50);
        assert_eq!(Z_MAX, 49);
        assert_eq!(Z_SURFACE, 0);

        // Deepest layer (-50) must be bedrock
        let bottom_tile = world.get_tile(MAP_WIDTH as i32 / 2, MAP_HEIGHT as i32 / 2, -50);
        assert!(bottom_tile.is_some());
        assert_eq!(bottom_tile.unwrap().kind, TileKind::Bedrock);

        // Highest layer (+49) must be open air
        let top_tile = world.get_tile(MAP_WIDTH as i32 / 2, MAP_HEIGHT as i32 / 2, 49);
        assert!(top_tile.is_some());
        assert_eq!(top_tile.unwrap().kind, TileKind::Air);
    }

    #[test]
    fn test_player_spawns_on_solid_ground() {
        let world = World::new(42, BiomeKind::Meadow);
        let player = Player::new(world.spawn_pos);

        assert_eq!(player.z, world.spawn_pos.2);
        let spawn_tile = world.get_tile(player.x, player.y, player.z).expect("Spawn tile must exist");
        
        assert!(spawn_tile.walkable, "Player must spawn on walkable terrain");
        assert_ne!(spawn_tile.kind, TileKind::WaterDeep, "Must not spawn in deep water");
        assert_ne!(spawn_tile.kind, TileKind::OceanWater, "Must not spawn in ocean water");
        assert_ne!(spawn_tile.kind, TileKind::WaterShallow, "Must not spawn submerged in water");
        assert_ne!(spawn_tile.kind, TileKind::TreeTrunk, "Must not spawn inside tree trunk");
    }

    #[test]
    fn test_mountain_and_desert_biomes_generate() {
        let mount_world = World::new(101, BiomeKind::Mountain);
        let m_tile = mount_world.get_tile(mount_world.spawn_pos.0, mount_world.spawn_pos.1, mount_world.spawn_pos.2).unwrap();
        assert!(m_tile.walkable);

        let desert_world = World::new(202, BiomeKind::Desert);
        let d_tile = desert_world.get_tile(desert_world.spawn_pos.0, desert_world.spawn_pos.1, desert_world.spawn_pos.2).unwrap();
        assert!(d_tile.walkable);
    }

    #[test]
    fn test_overworld_continents_generate_land() {
        let ow = Overworld::new(777);
        let land_count = ow.tiles.iter().filter(|t| t.is_land).count();
        let ocean_count = ow.tiles.iter().filter(|t| t.biome == BiomeKind::Ocean).count();

        assert!(land_count > 50, "Overworld must contain continent landmasses");
        assert!(ocean_count > 50, "Overworld must contain oceans around continents");
        assert!(ow.cursor_x < die_by_glory::world::overworld::OW_WIDTH);
        assert!(ow.cursor_y < die_by_glory::world::overworld::OW_HEIGHT);
    }

    #[test]
    fn test_recovery_from_ocean_save() {
        let ow = Overworld::new(42);
        let (lx, ly) = ow.find_nearest_land(25, 18);
        let tile = ow.get_tile(lx, ly).expect("Land tile must exist");
        assert!(tile.is_land, "Must find a solid land tile");
        assert_ne!(tile.biome, BiomeKind::Ocean, "Biome must not be ocean");

        // Verify can_embark_at from recovered land
        let mut test_ow = ow.clone();
        test_ow.set_station(lx, ly);
        let (can_embark, reason) = test_ow.can_embark_at(lx, ly, true);
        assert!(can_embark, "Must be able to embark at recovered location: {}", reason);
    }

    #[test]
    fn test_continental_adjacent_restrictions() {
        let mut ow = Overworld::new(888);
        ow.set_station(30, 20);
        assert_eq!(ow.current_station, Some((30, 20)));

        // Adjacent cells must be allowed
        assert!(ow.is_adjacent_to_current_station(31, 20));
        assert!(ow.is_adjacent_to_current_station(29, 21));
        assert!(ow.is_adjacent_to_current_station(30, 19));

        // Distant cells must be rejected
        assert!(!ow.is_adjacent_to_current_station(35, 20));
        assert!(!ow.is_adjacent_to_current_station(10, 10));

        // Taking new station as baseline
        ow.set_station(31, 20);
        assert_eq!(ow.current_station, Some((31, 20)));
        assert!(ow.is_adjacent_to_current_station(32, 20));
        // (29, 20) is no longer adjacent to (31, 20)!
        assert!(!ow.is_adjacent_to_current_station(29, 20));
    }

    #[test]
    fn test_savegame_memory_storage() {
        let world = World::new(42, BiomeKind::Forest);
        let mut player = Player::new(world.spawn_pos);
        player.health = 85.0;
        player.hunger = 70.0;

        let test_path = "target/test_savegame.json";
        let save = SaveGame::create_from_state(&world, &player, 42, 25, 18, vec![(25, 18)]);
        let res = save.save_to_path(test_path);
        assert!(res.is_ok(), "SaveGame must serialize to disk successfully");

        let loaded = SaveGame::load_from_path(test_path);
        assert!(loaded.is_some(), "SaveGame must load from disk");
        let l = loaded.unwrap();
        assert_eq!(l.overworld_seed, 42);
        assert_eq!(l.ow_current_x, 25);
        assert_eq!(l.ow_current_y, 18);
        assert_eq!(l.health, 85.0);
        assert_eq!(l.current_biome, BiomeKind::Forest);
        assert!(l.inventory.is_some(), "Inventory must be serialized and loaded");
        let inv = l.inventory.unwrap();
        assert_eq!(inv.slots.len(), 20);

        let _ = std::fs::remove_file(test_path);
    }

    #[test]
    fn test_pathfinding_finds_valid_route() {
        let world = World::new(1234, BiomeKind::Meadow);
        let start = (world.spawn_pos.0, world.spawn_pos.1);

        let mut target = None;
        for dx in 3..15 {
            let tx = start.0 + dx;
            let ty = start.1;
            let tz = world.get_surface_z(tx, ty);
            if let Some(tile) = world.get_tile(tx, ty, tz) {
                if tile.walkable {
                    target = Some((tx, ty));
                    break;
                }
            }
        }

        if let Some(goal) = target {
            let path = find_path(&world, start, goal);
            assert!(path.is_some(), "Pathfinding must find route on level ground");
            let p = path.unwrap();
            assert_eq!(p.first(), Some(&start));
            assert_eq!(p.last(), Some(&goal));
        }
    }

    #[test]
    fn test_translucent_surface_raycast() {
        let world = World::new(777, BiomeKind::Meadow);
        let (sx, sy, _) = world.spawn_pos;

        // Looking down from sky (Z = +10) must find surface level
        let surface = world.find_visible_surface_down(sx, sy, 10);
        assert!(surface.is_some());
        let (found_z, tile) = surface.unwrap();
        assert!(found_z <= 8, "Surface must be at ground elevation");
        assert_ne!(tile.kind, TileKind::Air);
    }

    #[test]
    fn test_cover_jpg_decoding() {
        let cover_bytes = include_bytes!("../assets/cover.jpg");
        assert!(!cover_bytes.is_empty());
        let img = macroquad::texture::Image::from_file_with_format(cover_bytes, None);
        assert!(img.is_ok(), "Image should decode cover.jpg");
        let decoded = img.unwrap();
        println!("Cover image dimensions: {}x{}", decoded.width, decoded.height);
        assert!(decoded.width > 0 && decoded.height > 0);
    }

    #[test]
    fn test_main_menu_state_and_actions() {
        use die_by_glory::render::{MenuAction, MenuState};
        let mut menu = MenuState::new();
        assert_eq!(menu.selected_index, 0);

        menu.selected_index = 1;
        assert_eq!(menu.selected_index, 1);

        menu.selected_index = 2;
        assert_eq!(menu.selected_index, 2);

        assert_ne!(MenuAction::NewGame, MenuAction::LoadGame);
        assert_ne!(MenuAction::LoadGame, MenuAction::Exit);
        assert_eq!(MenuAction::None, MenuAction::None);
    }

    #[test]
    fn test_inventory_storage_and_equipment() {
        use die_by_glory::entity::{EquipSlot, Inventory, ItemType};

        let mut inv = Inventory::new();
        assert_eq!(inv.slots.len(), 20);

        // Add initial resources requested by user
        let rem1 = inv.add_item(ItemType::TallGrass, 10);
        let rem2 = inv.add_item(ItemType::Stick, 5);
        let rem3 = inv.add_item(ItemType::Flint, 3);
        let rem4 = inv.add_item(ItemType::SmallStone, 8);

        assert_eq!(rem1, 0);
        assert_eq!(rem2, 0);
        assert_eq!(rem3, 0);
        assert_eq!(rem4, 0);

        assert_eq!(inv.count_item(ItemType::TallGrass), 10);
        assert_eq!(inv.count_item(ItemType::Stick), 5);
        assert_eq!(inv.count_item(ItemType::Flint), 3);
        assert_eq!(inv.count_item(ItemType::SmallStone), 8);

        // Test equipping weapon / tool
        inv.add_item(ItemType::FlintKnife, 1);
        let knife_slot = inv.slots.iter().position(|s| s.as_ref().is_some_and(|it| it.item == ItemType::FlintKnife)).unwrap();
        assert!(inv.equip_from_slot(knife_slot));
        assert_eq!(inv.equipment.get(EquipSlot::MainHand), Some(ItemType::FlintKnife));

        // Test equipping armor
        inv.add_item(ItemType::GrassCloak, 1);
        let cloak_slot = inv.slots.iter().position(|s| s.as_ref().is_some_and(|it| it.item == ItemType::GrassCloak)).unwrap();
        assert!(inv.equip_from_slot(cloak_slot));
        assert_eq!(inv.equipment.get(EquipSlot::Chest), Some(ItemType::GrassCloak));

        // Test un-equipping armor back into inventory
        assert!(inv.unequip(EquipSlot::Chest));
        assert_eq!(inv.equipment.get(EquipSlot::Chest), None);
        assert_eq!(inv.count_item(ItemType::GrassCloak), 1);
    }

    #[test]
    fn test_crafting_recipes() {
        use die_by_glory::entity::{get_all_recipes, Inventory, ItemType};

        let recipes = get_all_recipes();
        assert!(recipes.len() >= 6);

        let mut inv = Inventory::new();
        // Give tall grass for fiber cord
        inv.add_item(ItemType::TallGrass, 2);

        let cord_recipe = recipes.iter().find(|r| r.result.item == ItemType::FiberCord).unwrap();
        assert!(inv.can_craft(cord_recipe));
        assert!(inv.craft(cord_recipe));

        // Materials consumed, result created
        assert_eq!(inv.count_item(ItemType::TallGrass), 0);
        assert_eq!(inv.count_item(ItemType::FiberCord), 1);

        // Now craft Flint Knife: requires 1 Stick, 1 Flint, 1 FiberCord
        let knife_recipe = recipes.iter().find(|r| r.result.item == ItemType::FlintKnife).unwrap();
        assert!(!inv.can_craft(knife_recipe)); // missing stick and flint

        inv.add_item(ItemType::Stick, 1);
        inv.add_item(ItemType::Flint, 1);
        assert!(inv.can_craft(knife_recipe));
        assert!(inv.craft(knife_recipe));

        assert_eq!(inv.count_item(ItemType::FiberCord), 0);
        assert_eq!(inv.count_item(ItemType::Stick), 0);
        assert_eq!(inv.count_item(ItemType::Flint), 0);
        assert_eq!(inv.count_item(ItemType::FlintKnife), 1);
    }

    #[test]
    fn test_radar_detection_and_vision() {
        use die_by_glory::entity::{CreatureType, ItemType, Radar};
        use die_by_glory::world::tile::BiomeKind;
        use die_by_glory::world::World;

        let mut world = World::new(555, BiomeKind::Meadow);
        let (px, py, pz) = world.spawn_pos;

        // Clear default ground items/creatures for isolated test
        world.ground_items.clear();
        world.creatures.clear();

        // Spawn items within 16m radius
        world.spawn_ground_item(ItemType::TallGrass, 3, px + 3, py + 4, pz);
        world.spawn_ground_item(ItemType::Flint, 1, px - 5, py, pz);
        // Spawn item far away outside vision radius (> 16m)
        world.spawn_ground_item(ItemType::Stick, 2, px + 30, py + 30, pz);

        // Spawn creature within vision radius
        world.creatures.push(die_by_glory::entity::Creature::new(1, CreatureType::Deer, px, py + 6, pz));

        let mut radar = Radar::new();
        assert_eq!(radar.vision_radius, 16.0);
        radar.update(0.1, px, py, pz, &world);

        // Radar must activate because items and fauna are in vision area
        assert!(radar.is_active);
        assert_eq!(radar.detected_items.len(), 2, "Items within 16m should be detected, distant ones ignored");
        assert_eq!(radar.detected_creatures.len(), 1, "Creature within 16m should be detected");

        // Closest item should be the one at dx=3, dy=4 (dist = 5.0)
        let first_item = &radar.detected_items[0];
        assert_eq!(first_item.item, ItemType::TallGrass);
        assert!((first_item.distance - 5.0).abs() < 0.1);

        // Check relative direction
        assert_eq!(first_item.direction, "South-East [SE]");

        // Check creature detection
        let creature = &radar.detected_creatures[0];
        assert_eq!(creature.kind, CreatureType::Deer);
        assert_eq!(creature.direction, "South [S]");

        // Check region info
        assert_eq!(radar.region_info.biome_name, "Open Meadow");
        assert_eq!(radar.region_info.elevation_z, pz);

        // Test item pickup
        let picked = world.pickup_item_at(px + 3, py + 4, 1.0);
        assert!(picked.is_some());
        let gi = picked.unwrap();
        assert_eq!(gi.item, ItemType::TallGrass);
        assert_eq!(gi.count, 3);
    }

    #[test]
    fn test_crafting_all_tools_and_torches() {
        use die_by_glory::entity::{get_all_recipes, Inventory, ItemType};

        let recipes = get_all_recipes();
        let mut inv = Inventory::new();

        // Stock inventory with base resources
        inv.add_item(ItemType::TallGrass, 30);
        inv.add_item(ItemType::Stick, 30);
        inv.add_item(ItemType::Flint, 15);
        inv.add_item(ItemType::SmallStone, 15);

        // Craft Fiber Cords first (needed for tools)
        let cord_rec = recipes.iter().find(|r| r.result.item == ItemType::FiberCord).unwrap();
        for _ in 0..10 {
            assert!(inv.craft(cord_rec));
        }
        assert!(inv.count_item(ItemType::FiberCord) >= 10);

        // 1. Craft Stone Pickaxe (Pico)
        let pick_rec = recipes.iter().find(|r| r.result.item == ItemType::StonePickaxe).unwrap();
        assert!(inv.can_craft(pick_rec));
        assert!(inv.craft(pick_rec));
        assert_eq!(inv.count_item(ItemType::StonePickaxe), 1);

        // 2. Craft Stone Shovel (Pala)
        let shovel_rec = recipes.iter().find(|r| r.result.item == ItemType::StoneShovel).unwrap();
        assert!(inv.can_craft(shovel_rec));
        assert!(inv.craft(shovel_rec));
        assert_eq!(inv.count_item(ItemType::StoneShovel), 1);

        // 3. Craft Stone Hatchet (Hacha)
        let axe_rec = recipes.iter().find(|r| r.result.item == ItemType::StoneHatchet).unwrap();
        assert!(inv.can_craft(axe_rec));
        assert!(inv.craft(axe_rec));
        assert_eq!(inv.count_item(ItemType::StoneHatchet), 1);

        // 4. Craft Flint Knife (Cuchillo)
        let knife_rec = recipes.iter().find(|r| r.result.item == ItemType::FlintKnife).unwrap();
        assert!(inv.can_craft(knife_rec));
        assert!(inv.craft(knife_rec));
        assert_eq!(inv.count_item(ItemType::FlintKnife), 1);

        // 5. Craft Hunting Bow (Arco)
        let bow_rec = recipes.iter().find(|r| r.result.item == ItemType::HuntingBow).unwrap();
        assert!(inv.can_craft(bow_rec));
        assert!(inv.craft(bow_rec));
        assert_eq!(inv.count_item(ItemType::HuntingBow), 1);

        // 6. Craft Flint Arrows (Flechas x4)
        let arrow_rec = recipes.iter().find(|r| r.result.item == ItemType::FlintArrow).unwrap();
        assert!(inv.can_craft(arrow_rec));
        assert!(inv.craft(arrow_rec));
        assert_eq!(inv.count_item(ItemType::FlintArrow), 4);

        // 7. Craft Torches (Antorchas x2)
        let torch_rec = recipes.iter().find(|r| r.result.item == ItemType::Torch).unwrap();
        assert!(inv.can_craft(torch_rec));
        assert!(inv.craft(torch_rec));
        assert_eq!(inv.count_item(ItemType::Torch), 2);
    }

    #[test]
    fn test_hunting_and_butchering_yields() {
        use die_by_glory::entity::{get_all_recipes, Creature, CreatureType, EquipSlot, Inventory, ItemType};
        use die_by_glory::world::tile::BiomeKind;
        use die_by_glory::world::World;

        let mut world = World::new(999, BiomeKind::Forest);
        world.creatures.clear();
        world.ground_items.clear();

        // Spawn a Wild Deer (30 HP)
        let deer = Creature::new(100, CreatureType::Deer, 10, 10, 0);
        assert_eq!(deer.health, 30.0);
        assert_eq!(deer.max_health, 30.0);
        world.creatures.push(deer);

        // Test hunting creature: deals damage
        let dead1 = world.creatures[0].take_damage(16.0);
        assert!(!dead1);
        assert_eq!(world.creatures[0].health, 14.0);
        assert!(world.creatures[0].is_alert);

        // Deal fatal hit
        let dead2 = world.creatures[0].take_damage(15.0);
        assert!(dead2, "Lethal damage must slay the creature");
        assert!(world.creatures[0].health <= 0.0);

        // Spawn animal carcass on the ground
        world.spawn_ground_item(ItemType::AnimalCarcass, 1, 10, 10, 0);

        // Player approaches and skins the carcass
        let carcass = world.pickup_carcass_at(10, 10, 1.5);
        assert!(carcass.is_some(), "Carcass must be present on ground");

        let mut inv = Inventory::new();
        inv.add_item(ItemType::AnimalCarcass, 1);

        // Butchering with knife yields Leather, Bone, and Meat
        let (leather, bones, meat) = inv.butcher_carcass(true);
        assert_eq!(leather, 3, "Skinned carcass must yield leather");
        assert_eq!(bones, 3, "Skinned carcass must yield bones");
        assert_eq!(meat, 3, "Skinned carcass must yield raw meat");

        assert_eq!(inv.count_item(ItemType::Leather), 3);
        assert_eq!(inv.count_item(ItemType::Bone), 3);
        assert_eq!(inv.count_item(ItemType::RawMeat), 3);

        // Test cooking meat over torch
        let recipes = get_all_recipes();
        let cook_rec = recipes.iter().find(|r| r.result.item == ItemType::CookedMeat).unwrap();
        inv.add_item(ItemType::Torch, 1);
        assert!(inv.can_craft(cook_rec));
        assert!(inv.craft(cook_rec));

        assert_eq!(inv.count_item(ItemType::RawMeat), 2);
        assert_eq!(inv.count_item(ItemType::CookedMeat), 1);

        // Test crafting Cured Leather Armor from leather + cords
        let armor_rec = recipes.iter().find(|r| r.result.item == ItemType::LeatherArmor).unwrap();
        inv.add_item(ItemType::FiberCord, 2);
        assert!(inv.can_craft(armor_rec));
        assert!(inv.craft(armor_rec));
        assert_eq!(inv.count_item(ItemType::LeatherArmor), 1);

        // Equip armor and verify defense
        let slot = inv.slots.iter().position(|s| s.as_ref().is_some_and(|i| i.item == ItemType::LeatherArmor)).unwrap();
        assert!(inv.equip_from_slot(slot));
        assert_eq!(inv.equipment.get(EquipSlot::Chest), Some(ItemType::LeatherArmor));
    }

    #[test]
    fn test_woodcutting_quarrying_and_building() {
        use die_by_glory::world::tile::Tile;
        use die_by_glory::entity::{get_all_recipes, Inventory, ItemType};

        let mut world = World::new(42, BiomeKind::Forest);

        // 1. Test tree chopping with axe
        // Place a test tree at (20, 20)
        let sz = world.get_surface_z(20, 20);
        let idx = die_by_glory::world::generator::WorldGenerator::tile_index(20, 20, die_by_glory::world::generator::z_to_index(sz));
        world.tiles[idx] = Tile::tree_trunk();

        let chop_res = world.chop_tree_at(20, 20);
        assert!(chop_res.is_ok(), "Tree must be choppable");
        let (logs, sticks) = chop_res.unwrap();
        assert_eq!(logs, 2, "Chopping tree yields wood logs");
        assert_eq!(sticks, 2, "Chopping tree yields sticks");

        // Surface tile should now be soil
        assert_eq!(world.get_tile(20, 20, sz).unwrap().kind, TileKind::Soil);

        // 2. Test crafting Wood Planks from Wood Log
        let mut inv = Inventory::new();
        inv.add_item(ItemType::WoodLog, 1);
        let recipes = get_all_recipes();
        let plank_rec = recipes.iter().find(|r| r.result.item == ItemType::WoodPlanks).expect("Plank recipe must exist");
        assert!(inv.can_craft(plank_rec));
        assert!(inv.craft(plank_rec));
        assert_eq!(inv.count_item(ItemType::WoodPlanks), 4);

        // 3. Test building Wood Planks on the terrain
        let place_res = world.place_block_at(20, 20, ItemType::WoodPlanks);
        assert!(place_res.is_ok(), "Must be able to place wood planks");
        assert_eq!(world.get_tile(20, 20, sz).unwrap().kind, TileKind::WoodPlanks);

        // 4. Test stacking Planks into a wall (+1 Z)
        let stack_res = world.place_block_at(20, 20, ItemType::WoodPlanks);
        assert!(stack_res.is_ok(), "Must be able to stack blocks into walls");
        assert_eq!(world.get_surface_z(20, 20), sz + 1);
        assert_eq!(world.get_tile(20, 20, sz + 1).unwrap().kind, TileKind::WoodPlanks);

        // 5. Test demolishing the top block
        let dem1 = world.demolish_at(20, 20);
        assert_eq!(dem1, Ok(ItemType::WoodPlanks));
        assert_eq!(world.get_surface_z(20, 20), sz);

        // 6. Test demolishing the floor plank
        let dem2 = world.demolish_at(20, 20);
        assert_eq!(dem2, Ok(ItemType::WoodPlanks));
        assert_eq!(world.get_tile(20, 20, sz).unwrap().kind, TileKind::Soil);

        // 7. Test digging earth with shovel
        let shovel_res = world.shovel_dirt_at(20, 20);
        assert_eq!(shovel_res, Ok(ItemType::DirtBlock));
        assert_eq!(world.get_surface_z(20, 20), sz - 1, "Shoveling lowers elevation");

        // 8. Test pickaxing stone
        let pz = world.get_surface_z(25, 25);
        let pidx = die_by_glory::world::generator::WorldGenerator::tile_index(25, 25, die_by_glory::world::generator::z_to_index(pz));
        world.tiles[pidx] = Tile::stone();
        let pick_res = world.pickaxe_rock_at(25, 25);
        assert!(pick_res.is_ok());
        let (mined_item, count) = pick_res.unwrap();
        assert_eq!(mined_item, ItemType::Cobblestone);
        assert_eq!(count, 2);

        // 9. Test building Cobblestone block and demolishing it
        let place_cobble = world.place_block_at(25, 25, ItemType::Cobblestone);
        assert!(place_cobble.is_ok());
        let new_pz = world.get_surface_z(25, 25);
        assert_eq!(world.get_tile(25, 25, new_pz).unwrap().kind, TileKind::Cobblestone);

        let dem_cobble = world.demolish_at(25, 25);
        assert_eq!(dem_cobble, Ok(ItemType::Cobblestone));
    }

    #[test]
    fn test_build_mode_materials_and_actions() {
        use die_by_glory::entity::{BuildMaterial, ItemType};

        // 1. Check material progression and cycling
        let mut mat = BuildMaterial::WoodPlanks;
        assert_eq!(mat.item_type(), ItemType::WoodPlanks);
        assert_eq!(mat.glyph(), '#');
        assert_eq!(mat.name_es(), "Tablas de Madera");

        mat = mat.next();
        assert_eq!(mat, BuildMaterial::Cobblestone);
        assert_eq!(mat.item_type(), ItemType::Cobblestone);
        assert_eq!(mat.glyph(), 'O');
        assert_eq!(mat.name_es(), "Adoquin");

        mat = mat.next();
        assert_eq!(mat, BuildMaterial::DirtBlock);
        assert_eq!(mat.item_type(), ItemType::DirtBlock);
        assert_eq!(mat.glyph(), '.');
        assert_eq!(mat.name_es(), "Tierra");

        mat = mat.next();
        assert_eq!(mat, BuildMaterial::WoodPlanks, "Cycling must wrap back to WoodPlanks");

        // 2. Test world interaction through BuildMaterial items
        let mut world = World::new(4321, BiomeKind::Meadow);
        let tx = 30;
        let ty = 30;
        let base_z = world.get_surface_z(tx, ty);

        // Place WoodPlanks
        assert!(world.place_block_at(tx, ty, BuildMaterial::WoodPlanks.item_type()).is_ok());
        assert_eq!(world.get_tile(tx, ty, base_z).unwrap().kind, TileKind::WoodPlanks);

        // Demolish WoodPlanks
        let dem = world.demolish_at(tx, ty);
        assert_eq!(dem, Ok(ItemType::WoodPlanks));

        // Place Cobblestone
        assert!(world.place_block_at(tx, ty, BuildMaterial::Cobblestone.item_type()).is_ok());
        assert_eq!(world.get_tile(tx, ty, base_z).unwrap().kind, TileKind::Cobblestone);

        // Demolish Cobblestone
        let dem_c = world.demolish_at(tx, ty);
        assert_eq!(dem_c, Ok(ItemType::Cobblestone));
    }

    #[test]
    fn test_hotbar_slots_and_demolish_toggle() {
        use die_by_glory::entity::{default_hotbar, BuildMaterial, Inventory, ItemType};

        let mut inv = Inventory::new();
        // 1. Verify default hotbar layout
        assert_eq!(inv.hotbar, default_hotbar());
        assert_eq!(inv.hotbar[0], Some(ItemType::WoodPlanks));
        assert_eq!(inv.hotbar[1], Some(ItemType::Cobblestone));
        assert_eq!(inv.hotbar[2], Some(ItemType::DirtBlock));
        assert_eq!(inv.hotbar[3], None);
        assert_eq!(inv.hotbar[4], None);

        // 2. Verify mapping to BuildMaterial
        assert_eq!(inv.get_hotbar_material(0), Some(BuildMaterial::WoodPlanks));
        assert_eq!(inv.get_hotbar_material(1), Some(BuildMaterial::Cobblestone));
        assert_eq!(inv.get_hotbar_material(2), Some(BuildMaterial::DirtBlock));
        assert_eq!(inv.get_hotbar_material(3), None);

        // 3. Test slot customization (assigning items from backpack)
        inv.set_hotbar_slot(3, Some(ItemType::Torch));
        assert_eq!(inv.hotbar[3], Some(ItemType::Torch));
        // Torch is not a building block, so get_hotbar_material returns None
        assert_eq!(inv.get_hotbar_material(3), None);

        inv.set_hotbar_slot(0, Some(ItemType::DirtBlock));
        assert_eq!(inv.get_hotbar_material(0), Some(BuildMaterial::DirtBlock));

        // 4. Verify serde backward-compatibility
        let json_without_hotbar = r#"{"slots":[null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null],"equipment":{"main_hand":null,"off_hand":null,"head":null,"chest":null,"legs":null},"crafting_grid":[null,null,null,null]}"#;
        let deserialized: Inventory = serde_json::from_str(json_without_hotbar).expect("Must deserialize without hotbar field");
        assert_eq!(deserialized.hotbar[0], Some(ItemType::WoodPlanks));
        assert_eq!(deserialized.hotbar[1], Some(ItemType::Cobblestone));
        assert_eq!(deserialized.hotbar[2], Some(ItemType::DirtBlock));
    }
}

