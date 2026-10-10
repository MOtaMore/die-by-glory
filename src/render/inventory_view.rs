use crate::entity::inventory::get_all_recipes;
use crate::entity::item::{EquipSlot, ItemCategory, ItemStack, ItemType};
use crate::entity::Player;
use crate::world::World;
use macroquad::prelude::*;

pub struct InventoryUiState {
    pub is_open: bool,
    pub selected_slot: Option<usize>,
    pub selected_recipe: usize,
    pub recipe_scroll: usize,
    pub log_message: Option<(String, f32)>,
}

impl InventoryUiState {
    pub fn new() -> Self {
        Self {
            is_open: false,
            selected_slot: None,
            selected_recipe: 0,
            recipe_scroll: 0,
            log_message: None,
        }
    }

    pub fn toggle(&mut self) {
        self.is_open = !self.is_open;
        if self.is_open && self.selected_slot.is_none() {
            self.selected_slot = Some(0);
        }
    }

    pub fn set_log(&mut self, msg: String) {
        self.log_message = Some((msg, get_time() as f32));
    }
}

pub struct InventoryUiResult {
    pub closed: bool,
    pub dropped_item: Option<ItemStack>,
}

pub fn render_inventory_screen(
    player: &mut Player,
    world: &mut World,
    state: &mut InventoryUiState,
    font: Option<&Font>,
) -> InventoryUiResult {
    let mut result = InventoryUiResult {
        closed: false,
        dropped_item: None,
    };

    let sw = screen_width();
    let sh = screen_height();
    let time = get_time() as f32;

    let mouse_pos = mouse_position();
    let (mx, my) = (mouse_pos.0, mouse_pos.1);
    let mouse_clicked = is_mouse_button_pressed(MouseButton::Left);

    // Dark semi-transparent scrim over the game world
    draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.02, 0.03, 0.05, 0.78));

    // Modal card container
    let card_w = 980.0f32.min(sw - 30.0);
    let card_h = 600.0f32.min(sh - 40.0);
    let card_x = (sw - card_w) * 0.5;
    let card_y = (sh - card_h) * 0.5;

    // Card background & borders
    draw_rectangle(card_x, card_y, card_w, card_h, Color::new(0.05, 0.07, 0.11, 0.98));
    draw_rectangle_lines(card_x, card_y, card_w, card_h, 2.0, Color::new(0.40, 0.65, 0.88, 1.0));
    draw_rectangle_lines(card_x + 3.0, card_y + 3.0, card_w - 6.0, card_h - 6.0, 1.0, Color::new(0.20, 0.32, 0.48, 0.8));

    // Header bar
    let header_h = 32.0;
    draw_rectangle(card_x, card_y, card_w, header_h, Color::new(0.09, 0.14, 0.22, 1.0));
    draw_line(card_x, card_y + header_h, card_x + card_w, card_y + header_h, 1.2, Color::new(0.35, 0.55, 0.75, 1.0));

    let header_title = "╔═[ 🎒 SURVIVOR BACKPACK & CRAFTING STATION :: MINECRAFT STYLE ]═╗";
    draw_text_ex(
        header_title,
        card_x + 18.0,
        card_y + 22.0,
        TextParams {
            font,
            font_size: 13,
            color: Color::new(1.0, 0.88, 0.35, 1.0),
            ..Default::default()
        },
    );

    // Close button [X]
    let close_btn_x = card_x + card_w - 32.0;
    let close_btn_y = card_y + 5.0;
    if draw_button("[X]", close_btn_x, close_btn_y, 24.0, 22.0, mx, my, mouse_clicked, font, Color::new(0.9, 0.3, 0.3, 1.0))
        || is_key_pressed(KeyCode::Escape)
    {
        state.is_open = false;
        result.closed = true;
        return result;
    }

    let content_y = card_y + header_h + 12.0;
    let col1_w = 230.0;
    let col2_w = 390.0;
    let col3_w = card_w - col1_w - col2_w - 40.0;

    let col1_x = card_x + 14.0;
    let col2_x = col1_x + col1_w + 12.0;
    let col3_x = col2_x + col2_w + 12.0;

    // =========================================================================
    // 1. LEFT COLUMN: CHARACTER SILHOUETTE & EQUIPMENT SLOTS
    // =========================================================================
    draw_ascii_box("SURVIVOR EQUIPMENT", col1_x, content_y, col1_w, 480.0, font);

    // ASCII Portrait
    let portrait = [
        r"      .---.      ",
        r"     ( @.@ )     ",
        r"    .-'- -'-.    ",
        r"   /|   |   |\   ",
        r"  / |   |   | \  ",
        r"    |---|---|    ",
        r"    |   |   |    ",
        r"    '---'---'    ",
    ];
    let mut py = content_y + 26.0;
    for line in &portrait {
        let dim = measure_text(line, font, 11, 1.0);
        draw_text_ex(
            line,
            col1_x + (col1_w - dim.width) * 0.5,
            py,
            TextParams {
                font,
                font_size: 11,
                color: Color::new(0.65, 0.85, 0.95, 0.85),
                ..Default::default()
            },
        );
        py += 12.0;
    }
    py += 8.0;

    // Render equipment slots
    let eq_slots = [
        (EquipSlot::MainHand, "🗡 MAIN HAND", player.inventory.equipment.main_hand),
        (EquipSlot::OffHand, "🛡 OFF HAND", player.inventory.equipment.off_hand),
        (EquipSlot::Chest, "🎽 CHEST ARMOR", player.inventory.equipment.chest),
    ];

    for (slot_type, label, eq_item) in eq_slots {
        draw_text_ex(label, col1_x + 14.0, py, TextParams { font, font_size: 11, color: Color::new(0.40, 0.75, 0.95, 1.0), ..Default::default() });
        py += 14.0;

        let slot_btn_w = col1_w - 28.0;
        let slot_btn_h = 32.0;
        let is_equipped = eq_item.is_some();

        let (text, col) = if let Some(item) = eq_item {
            (format!("[{}] {}", item.glyph(), item.name()), item.color())
        } else {
            ("[ Empty Slot ]".to_string(), Color::new(0.40, 0.45, 0.50, 1.0))
        };

        if draw_button(
            &text,
            col1_x + 14.0,
            py,
            slot_btn_w,
            slot_btn_h,
            mx,
            my,
            mouse_clicked,
            font,
            col,
        ) {
            if is_equipped {
                if player.inventory.unequip(slot_type) {
                    state.set_log(format!("Unequipped {}.", text));
                }
            }
        }
        py += slot_btn_h + 10.0;
    }

    // Vitals summary badge
    py = content_y + 400.0;
    draw_line(col1_x + 10.0, py, col1_x + col1_w - 10.0, py, 1.0, Color::new(0.20, 0.32, 0.45, 0.8));
    py += 16.0;
    draw_text_ex(
        &format!("HP: {:03}% | STAM: {:03}%", player.health as i32, player.energy as i32),
        col1_x + 14.0,
        py,
        TextParams { font, font_size: 11, color: Color::new(0.9, 0.4, 0.4, 1.0), ..Default::default() },
    );
    py += 16.0;
    draw_text_ex(
        &format!("HUN: {:03}% | THI: {:03}%", player.hunger as i32, player.thirst as i32),
        col1_x + 14.0,
        py,
        TextParams { font, font_size: 11, color: Color::new(0.4, 0.8, 1.0, 1.0), ..Default::default() },
    );

    // =========================================================================
    // 2. CENTER COLUMN: STORAGE BACKPACK (20 SLOTS)
    // =========================================================================
    draw_ascii_box("BACKPACK STORAGE (20 SLOTS)", col2_x, content_y, col2_w, 480.0, font);

    let grid_x = col2_x + 14.0;
    let grid_y = content_y + 26.0;
    let slot_w = 70.0;
    let slot_h = 58.0;
    let pad = 6.0;

    for row in 0..4 {
        for col in 0..5 {
            let idx = row * 5 + col;
            let sx = grid_x + col as f32 * (slot_w + pad);
            let sy = grid_y + row as f32 * (slot_h + pad);

            let is_selected = state.selected_slot == Some(idx);
            let is_hovered = mx >= sx && mx <= (sx + slot_w) && my >= sy && my <= (sy + slot_h);

            if is_hovered && mouse_clicked {
                state.selected_slot = Some(idx);
            }

            let bg_col = if is_selected {
                Color::new(0.18, 0.28, 0.44, 0.95)
            } else if is_hovered {
                Color::new(0.12, 0.18, 0.26, 0.9)
            } else {
                Color::new(0.06, 0.08, 0.12, 0.9)
            };

            let border_col = if is_selected {
                Color::new(1.0, 0.85, 0.30, 1.0)
            } else if is_hovered {
                Color::new(0.50, 0.70, 0.90, 1.0)
            } else {
                Color::new(0.25, 0.35, 0.48, 0.8)
            };

            draw_rectangle(sx, sy, slot_w, slot_h, bg_col);
            draw_rectangle_lines(sx, sy, slot_w, slot_h, if is_selected { 2.0 } else { 1.0 }, border_col);

            // Slot number in corner
            draw_text_ex(
                &format!("{:02}", idx + 1),
                sx + 4.0,
                sy + 12.0,
                TextParams {
                    font,
                    font_size: 9,
                    color: Color::new(0.40, 0.50, 0.60, 0.8),
                    ..Default::default()
                },
            );

            // Draw item inside slot if occupied
            if let Some(ref stack) = player.inventory.slots[idx] {
                let glyph_str = stack.item.glyph().to_string();
                let g_dim = measure_text(&glyph_str, font, 16, 1.0);
                draw_text_ex(
                    &glyph_str,
                    sx + (slot_w - g_dim.width) * 0.5,
                    sy + 32.0,
                    TextParams {
                        font,
                        font_size: 16,
                        color: stack.item.color(),
                        ..Default::default()
                    },
                );

                // Count badge
                let count_str = format!("x{:02}", stack.count);
                let c_dim = measure_text(&count_str, font, 10, 1.0);
                draw_text_ex(
                    &count_str,
                    sx + slot_w - c_dim.width - 4.0,
                    sy + slot_h - 6.0,
                    TextParams {
                        font,
                        font_size: 10,
                        color: Color::new(0.95, 0.95, 0.95, 1.0),
                        ..Default::default()
                    },
                );
            }
        }
    }

    // Selected item detail panel at bottom of center column
    let detail_y = grid_y + 4.0 * (slot_h + pad) + 8.0;
    let detail_h = 160.0;
    draw_rectangle(col2_x + 14.0, detail_y, col2_w - 28.0, detail_h, Color::new(0.04, 0.06, 0.09, 0.9));
    draw_rectangle_lines(col2_x + 14.0, detail_y, col2_w - 28.0, detail_h, 1.0, Color::new(0.25, 0.38, 0.52, 0.8));

    if let Some(s_idx) = state.selected_slot {
        if let Some(ref stack) = player.inventory.slots[s_idx].clone() {
            let item = stack.item;
            draw_text_ex(
                &format!("[{}] {} (Stack: {})", item.glyph(), item.name(), stack.count),
                col2_x + 22.0,
                detail_y + 20.0,
                TextParams { font, font_size: 13, color: item.color(), ..Default::default() },
            );

            draw_text_ex(
                item.description(),
                col2_x + 22.0,
                detail_y + 38.0,
                TextParams { font, font_size: 10, color: Color::new(0.75, 0.85, 0.95, 0.9), ..Default::default() },
            );

            // Action buttons
            let action_btn_y = detail_y + 54.0;
            let can_equip = item.equip_slot().is_some();
            let is_carcass = item == ItemType::AnimalCarcass;
            let is_consumable = item.category() == ItemCategory::Consumable;

            if can_equip {
                if draw_button("[ 🗡 EQUIP (Key E) ]", col2_x + 22.0, action_btn_y, 150.0, 24.0, mx, my, mouse_clicked, font, Color::new(0.4, 0.85, 1.0, 1.0))
                    || is_key_pressed(KeyCode::E)
                {
                    if player.inventory.equip_from_slot(s_idx) {
                        state.set_log(format!("Equipped {}.", item.name()));
                    }
                }
            } else if is_carcass {
                if draw_button("[ 🔪 BUTCHER (Key E) ]", col2_x + 22.0, action_btn_y, 150.0, 24.0, mx, my, mouse_clicked, font, Color::new(0.95, 0.45, 0.45, 1.0))
                    || is_key_pressed(KeyCode::E)
                {
                    let has_knife = player.inventory.has_item(ItemType::FlintKnife, 1)
                        || player.inventory.equipment.main_hand == Some(ItemType::FlintKnife);
                    let (leather, bones, meat) = player.inventory.butcher_carcass(has_knife);
                    if leather > 0 {
                        state.set_log(format!(
                            "🔪 Butchered carcass! Yield: {}x Leather, {}x Bones, and {}x Raw Meat.",
                            leather, bones, meat
                        ));
                    }
                }
            } else if is_consumable {
                let action_name = if item == ItemType::PrimitiveBandage { "[ ➕ HEAL (Key E) ]" } else { "[ 🍖 EAT FOOD (Key E) ]" };
                if draw_button(action_name, col2_x + 22.0, action_btn_y, 150.0, 24.0, mx, my, mouse_clicked, font, Color::new(0.5, 0.95, 0.6, 1.0))
                    || is_key_pressed(KeyCode::E)
                {
                    if item == ItemType::PrimitiveBandage {
                        player.health = (player.health + 25.0).min(player.max_health);
                        player.inventory.remove_item(item, 1);
                        state.set_log("Used bandage! Health restored +25.".to_string());
                    } else if item == ItemType::RawMeat {
                        player.hunger = (player.hunger + 18.0).min(player.max_hunger);
                        player.health = (player.health + 5.0).min(player.max_health);
                        player.inventory.remove_item(item, 1);
                        state.set_log("Ate raw meat. Restored +18 Hunger, +5 Health.".to_string());
                    } else if item == ItemType::CookedMeat {
                        player.hunger = (player.hunger + 40.0).min(player.max_hunger);
                        player.health = (player.health + 15.0).min(player.max_health);
                        player.inventory.remove_item(item, 1);
                        state.set_log("Ate savory roasted meat! Restored +40 Hunger, +15 Health.".to_string());
                    }
                }
            }

            // Drop button
            if draw_button("[ ⬇ DROP (Key D) ]", col2_x + 180.0, action_btn_y, 160.0, 24.0, mx, my, mouse_clicked, font, Color::new(0.9, 0.6, 0.3, 1.0))
                || is_key_pressed(KeyCode::D)
            {
                if let Some(dropped) = player.inventory.drop_from_slot(s_idx) {
                    world.spawn_ground_item(dropped.item, dropped.count, player.x, player.y, player.z);
                    state.set_log(format!("Dropped {} onto ground.", dropped.item.name()));
                    result.dropped_item = Some(dropped);
                }
            }

            // Hotbar Assignment Controls
            let hb_label_y = detail_y + 88.0;
            draw_text_ex(
                "🧱 ASIGNAR A HOTBAR (Haz clic o pulsa teclas 1-5):",
                col2_x + 22.0,
                hb_label_y,
                TextParams { font, font_size: 11, color: Color::new(0.95, 0.80, 0.35, 1.0), ..Default::default() },
            );

            let hb_keys = [
                (0, KeyCode::Key1),
                (1, KeyCode::Key2),
                (2, KeyCode::Key3),
                (3, KeyCode::Key4),
                (4, KeyCode::Key5),
            ];
            let hb_btn_y = detail_y + 102.0;
            for (h_idx, key) in hb_keys {
                let hx = col2_x + 22.0 + h_idx as f32 * 68.0;
                let current_in_slot = player.inventory.hotbar[h_idx];
                let btn_text = if let Some(ci) = current_in_slot {
                    format!("[{}] {}", h_idx + 1, ci.glyph())
                } else {
                    format!("[{}] --", h_idx + 1)
                };

                let is_current = current_in_slot == Some(item);
                let btn_col = if is_current {
                    Color::new(0.95, 0.85, 0.30, 1.0)
                } else {
                    Color::new(0.50, 0.70, 0.90, 1.0)
                };

                if draw_button(&btn_text, hx, hb_btn_y, 64.0, 26.0, mx, my, mouse_clicked, font, btn_col)
                    || is_key_pressed(key)
                {
                    player.inventory.set_hotbar_slot(h_idx, Some(item));
                    state.set_log(format!("Asignado {} al Slot {} de la Hotbar.", item.name(), h_idx + 1));
                }
            }
        } else {
            draw_text_ex(
                "[ Empty Slot Selected ]",
                col2_x + 24.0,
                detail_y + 24.0,
                TextParams { font, font_size: 12, color: Color::new(0.45, 0.50, 0.60, 1.0), ..Default::default() },
            );
            draw_text_ex(
                "Gather wild resources or craft items to fill your backpack.",
                col2_x + 24.0,
                detail_y + 45.0,
                TextParams { font, font_size: 10, color: Color::new(0.40, 0.45, 0.52, 1.0), ..Default::default() },
            );

            // Display current hotbar overview even when empty slot is selected
            draw_text_ex(
                "🧱 HOTBAR DE CONSTRUCCIÓN ACTUAL (Slots 1-5):",
                col2_x + 24.0,
                detail_y + 75.0,
                TextParams { font, font_size: 11, color: Color::new(0.85, 0.75, 0.35, 1.0), ..Default::default() },
            );

            let hb_btn_y = detail_y + 92.0;
            for h_idx in 0..5 {
                let hx = col2_x + 24.0 + h_idx as f32 * 68.0;
                let current_in_slot = player.inventory.hotbar[h_idx];
                let label = if let Some(ci) = current_in_slot {
                    format!("[{}] {}", h_idx + 1, ci.glyph())
                } else {
                    format!("[{}] -", h_idx + 1)
                };
                draw_button(&label, hx, hb_btn_y, 64.0, 24.0, mx, my, false, font, Color::new(0.40, 0.55, 0.70, 0.8));
            }
        }
    }

    // =========================================================================
    // 3. RIGHT COLUMN: SURVIVAL CRAFTING (RECIPES & 2x2 MINECRAFT GRID)
    // =========================================================================
    let recipes = get_all_recipes();
    let total_recipes = recipes.len();
    let max_visible = 6;

    // Check mouse wheel scroll over col3
    let is_mouse_over_col3 = mx >= col3_x && mx <= (col3_x + col3_w) && my >= content_y && my <= (content_y + 480.0);
    let (_, wy) = mouse_wheel();
    if is_mouse_over_col3 && wy != 0.0 {
        if wy > 0.0 && state.recipe_scroll > 0 {
            state.recipe_scroll = state.recipe_scroll.saturating_sub(1);
        } else if wy < 0.0 && state.recipe_scroll + max_visible < total_recipes {
            state.recipe_scroll = (state.recipe_scroll + 1).min(total_recipes.saturating_sub(max_visible));
        }
    }

    let box_title = format!("CRAFTING RECIPES ({}/{})", state.recipe_scroll + 1, total_recipes);
    draw_ascii_box(&box_title, col3_x, content_y, col3_w, 480.0, font);

    // Navigation sub-bar for recipes
    let nav_y = content_y + 20.0;
    let nav_w = (col3_w - 28.0) * 0.5;
    if state.recipe_scroll > 0 {
        if draw_button("[ ▲ PREV ]", col3_x + 12.0, nav_y, nav_w, 20.0, mx, my, mouse_clicked, font, Color::new(0.6, 0.8, 1.0, 1.0)) {
            state.recipe_scroll = state.recipe_scroll.saturating_sub(1);
        }
    }
    if state.recipe_scroll + max_visible < total_recipes {
        if draw_button("[ ▼ NEXT ]", col3_x + 16.0 + nav_w, nav_y, nav_w, 20.0, mx, my, mouse_clicked, font, Color::new(0.6, 0.8, 1.0, 1.0)) {
            state.recipe_scroll = (state.recipe_scroll + 1).min(total_recipes.saturating_sub(max_visible));
        }
    }

    let mut ry = content_y + 44.0;
    let visible_slice = recipes.iter().enumerate().skip(state.recipe_scroll).take(max_visible);

    for (r_idx, recipe) in visible_slice {
        let can_craft = player.inventory.can_craft(recipe);
        let is_sel = state.selected_recipe == r_idx;
        let btn_w = col3_w - 24.0;
        let btn_h = 42.0;

        let is_hov = mx >= (col3_x + 12.0) && mx <= (col3_x + 12.0 + btn_w) && my >= ry && my <= (ry + btn_h);
        if is_hov && mouse_clicked {
            state.selected_recipe = r_idx;
        }

        let bg = if is_sel {
            Color::new(0.18, 0.28, 0.42, 0.95)
        } else if is_hov {
            Color::new(0.12, 0.18, 0.25, 0.9)
        } else {
            Color::new(0.07, 0.09, 0.13, 0.9)
        };

        let border = if can_craft {
            Color::new(0.35, 0.85, 0.50, 1.0)
        } else {
            Color::new(0.25, 0.32, 0.42, 0.8)
        };

        draw_rectangle(col3_x + 12.0, ry, btn_w, btn_h, bg);
        draw_rectangle_lines(col3_x + 12.0, ry, btn_w, btn_h, if is_sel { 2.0 } else { 1.0 }, border);

        let title_col = if can_craft {
            Color::new(1.0, 0.95, 0.55, 1.0)
        } else {
            Color::new(0.55, 0.60, 0.68, 1.0)
        };

        draw_text_ex(
            &format!("[{}] {}", recipe.result.item.glyph(), recipe.name),
            col3_x + 20.0,
            ry + 15.0,
            TextParams { font, font_size: 11, color: title_col, ..Default::default() },
        );

        // Required ingredients string
        let req_str = recipe
            .ingredients
            .iter()
            .map(|(item, count)| format!("{}x {}", count, item.name()))
            .collect::<Vec<_>>()
            .join(" + ");

        let req_col = if can_craft {
            Color::new(0.40, 0.90, 0.60, 1.0)
        } else {
            Color::new(0.75, 0.45, 0.45, 1.0)
        };

        draw_text_ex(
            &req_str,
            col3_x + 20.0,
            ry + 30.0,
            TextParams { font, font_size: 9, color: req_col, ..Default::default() },
        );

        // Click to craft if clicked on Craft action
        if is_hov && mouse_clicked && can_craft {
            if player.inventory.craft(recipe) {
                state.set_log(format!("Crafted 1x {}!", recipe.name));
            }
        }

        ry += btn_h + 6.0;
    }

    // Selected recipe craft button
    if state.selected_recipe < recipes.len() {
        let active_recipe = &recipes[state.selected_recipe];
        let can_craft = player.inventory.can_craft(active_recipe);
        let c_btn_y = content_y + 410.0;
        let c_btn_w = col3_w - 24.0;

        let (c_text, c_col) = if can_craft {
            (format!("🔨 CRAFT 1x {} [Key C]", active_recipe.name), Color::new(0.4, 0.95, 0.5, 1.0))
        } else {
            ("❌ MISSING MATERIALS".to_string(), Color::new(0.5, 0.5, 0.5, 1.0))
        };

        if (draw_button(&c_text, col3_x + 12.0, c_btn_y, c_btn_w, 32.0, mx, my, mouse_clicked, font, c_col)
            || is_key_pressed(KeyCode::C)
            || is_key_pressed(KeyCode::Enter))
            && can_craft
        {
            if player.inventory.craft(active_recipe) {
                state.set_log(format!("Successfully crafted {}!", active_recipe.name));
            }
        }
    }

    // Bottom notification log line
    let foot_y = card_y + card_h - 24.0;
    if let Some((ref msg, t)) = state.log_message {
        if time - t < 4.0 {
            draw_text_ex(
                &format!("> {}", msg),
                card_x + 18.0,
                foot_y,
                TextParams {
                    font,
                    font_size: 11,
                    color: Color::new(0.35, 0.95, 0.65, 1.0),
                    ..Default::default()
                },
            );
        }
    } else {
        draw_text_ex(
            "[Click Slot: Select] [E: Equip/Use] [D: Drop] [C/Click: Craft] [I/ESC: Close Inventory]",
            card_x + 18.0,
            foot_y,
            TextParams {
                font,
                font_size: 11,
                color: Color::new(0.55, 0.65, 0.78, 0.9),
                ..Default::default()
            },
        );
    }

    result
}

fn draw_ascii_box(title: &str, x: f32, y: f32, w: f32, h: f32, font: Option<&Font>) {
    draw_rectangle(x, y, w, h, Color::new(0.06, 0.08, 0.12, 0.9));
    draw_rectangle_lines(x, y, w, h, 1.2, Color::new(0.25, 0.38, 0.52, 1.0));

    let title_badge = format!("[ {} ]", title);
    draw_text_ex(
        &title_badge,
        x + 10.0,
        y + 14.0,
        TextParams {
            font,
            font_size: 11,
            color: Color::new(0.40, 0.75, 0.95, 1.0),
            ..Default::default()
        },
    );
}

fn draw_button(
    text: &str,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    mx: f32,
    my: f32,
    clicked: bool,
    font: Option<&Font>,
    accent_color: Color,
) -> bool {
    let hovered = mx >= x && mx <= (x + w) && my >= y && my <= (y + h);

    let bg = if hovered {
        Color::new(0.18, 0.28, 0.42, 0.95)
    } else {
        Color::new(0.09, 0.12, 0.18, 0.85)
    };

    let border = if hovered {
        Color::new(1.0, 0.85, 0.30, 1.0)
    } else {
        Color::new(accent_color.r * 0.7, accent_color.g * 0.7, accent_color.b * 0.7, 0.8)
    };

    draw_rectangle(x, y, w, h, bg);
    draw_rectangle_lines(x, y, w, h, if hovered { 1.8 } else { 1.0 }, border);

    let font_size = 11;
    let dim = measure_text(text, font, font_size, 1.0);
    let tx = x + (w - dim.width) * 0.5;
    let ty = y + (h + dim.height * 0.70) * 0.5;

    draw_text_ex(
        text,
        tx,
        ty,
        TextParams {
            font,
            font_size,
            color: if hovered { Color::new(1.0, 1.0, 1.0, 1.0) } else { accent_color },
            ..Default::default()
        },
    );

    hovered && clicked
}
