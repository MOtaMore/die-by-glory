use crate::camera::Camera;
use crate::entity::{BuildMaterial, ItemType, Player};
use crate::world::tile::Tile;
use crate::world::{World, Z_MIN, Z_SURFACE};
use macroquad::prelude::*;

pub struct UiState {
    pub log_messages: Vec<(String, f32)>,
}

impl UiState {
    pub fn new() -> Self {
        let mut state = Self {
            log_messages: Vec::new(),
        };
        state.add_log("Welcome to Die by Glory! ASCII survival simulation active.".to_string());
        state.add_log("You have safely embarked onto solid ground.".to_string());
        state.add_log("Use MOUSE WHEEL to cycle through all 100 Z-layers.".to_string());
        state.add_log("Hold CTRL + MOUSE WHEEL to zoom the field of view.".to_string());
        state.add_log("LEFT CLICK on any cell to navigate with A* Pathfinding.".to_string());
        state
    }

    pub fn add_log(&mut self, msg: String) {
        let now = get_time() as f32;
        self.log_messages.push((msg, now));
        if self.log_messages.len() > 25 {
            self.log_messages.remove(0);
        }
    }
}

pub struct UiResult {
    pub clicked_up_layer: bool,
    pub clicked_down_layer: bool,
    pub clicked_player_layer: bool,
    pub clicked_surface_layer: bool,
    pub clicked_drink: bool,
    pub clicked_fast_travel: bool,
    pub clicked_save: bool,
    pub clicked_exit_to_menu: bool,
    pub clicked_inventory: bool,
    pub clicked_radar: bool,
    pub clicked_hunt: bool,
    pub clicked_build_planks: bool,
    pub clicked_build_cobble: bool,
    pub clicked_harvest_demolish: bool,
    pub clicked_toggle_build_mode: bool,
    pub clicked_select_material: Option<BuildMaterial>,
    pub is_mouse_over_ui: bool,
}

pub fn render_ui(
    world: &World,
    player: &Player,
    camera: &Camera,
    hover_tile: Option<(i32, i32, i32, &Tile)>,
    ui_state: &UiState,
    font: Option<&Font>,
    is_build_mode: bool,
    build_material: BuildMaterial,
) -> UiResult {
    let screen_w = screen_width();
    let screen_h = screen_height();
    let sidebar_w = 330.0;
    let top_h = 38.0;
    let bottom_h = 28.0;

    let mouse_pos = mouse_position();
    let mx = mouse_pos.0;
    let my = mouse_pos.1;

    let mut result = UiResult {
        clicked_up_layer: false,
        clicked_down_layer: false,
        clicked_player_layer: false,
        clicked_surface_layer: false,
        clicked_drink: false,
        clicked_fast_travel: false,
        clicked_save: false,
        clicked_exit_to_menu: false,
        clicked_inventory: false,
        clicked_radar: false,
        clicked_hunt: false,
        clicked_build_planks: false,
        clicked_build_cobble: false,
        clicked_harvest_demolish: false,
        clicked_toggle_build_mode: false,
        clicked_select_material: None,
        is_mouse_over_ui: false,
    };

    let is_in_sidebar = mx >= (screen_w - sidebar_w);
    let is_in_topbar = my < top_h;
    let is_in_bottombar = my >= (screen_h - bottom_h);
    result.is_mouse_over_ui = is_in_sidebar || is_in_topbar || is_in_bottombar;

    let mouse_click = is_mouse_button_pressed(MouseButton::Left);

    // ==========================================
    // 1. ASCII TOP HEADER
    // ==========================================
    draw_rectangle(
        0.0,
        0.0,
        screen_w,
        top_h,
        Color::new(0.06, 0.08, 0.11, 1.0),
    );
    draw_line(
        0.0,
        top_h,
        screen_w,
        top_h,
        1.2,
        Color::new(0.25, 0.35, 0.48, 1.0),
    );

    // Header Title
    let title = "DIE BY GLORY :: ASCII SURVIVAL [RUST]";
    draw_text_ex(
        title,
        14.0,
        24.0,
        TextParams {
            font,
            font_size: 15,
            color: Color::new(0.95, 0.82, 0.25, 1.0),
            ..Default::default()
        },
    );

    // Time & Biome
    let time_str = format!("Time: {} | Biome: {}", world.formatted_time(), world.primary_biome.info().name);
    draw_text_ex(
        &time_str,
        340.0,
        24.0,
        TextParams {
            font,
            font_size: 13,
            color: Color::new(0.70, 0.85, 0.95, 1.0),
            ..Default::default()
        },
    );

    // Current View Layer
    let layer_name = if camera.view_z == Z_SURFACE {
        "Layer 0: Surface (Ground)".to_string()
    } else if camera.view_z > 0 {
        format!("Layer +{}: Sky / Atmosphere", camera.view_z)
    } else {
        format!("Layer {}: Subterranean Depth", camera.view_z)
    };

    draw_text_ex(
        &format!("View: {}", layer_name),
        630.0,
        24.0,
        TextParams {
            font,
            font_size: 13,
            color: Color::new(0.35, 0.90, 0.65, 1.0),
            ..Default::default()
        },
    );

    // Quick action buttons in top bar
    let sb_x = screen_w - sidebar_w;
    let menu_btn_w = 85.0;
    let save_btn_w = 95.0;
    let inv_btn_w = 120.0;
    let build_btn_w = 135.0;
    let menu_btn_x = sb_x - 12.0 - menu_btn_w;
    let save_btn_x = menu_btn_x - 8.0 - save_btn_w;
    let inv_btn_x = save_btn_x - 8.0 - inv_btn_w;
    let build_top_btn_x = inv_btn_x - 8.0 - build_btn_w;

    let build_top_text = if is_build_mode {
        "[ 🔨 BUILD: ON (B) ]"
    } else {
        "[ 🔨 BUILD (B) ]"
    };
    let build_bg = if is_build_mode {
        Some(Color::new(0.35, 0.22, 0.05, 0.95))
    } else {
        None
    };
    let build_border = if is_build_mode {
        Some(Color::new(1.0, 0.85, 0.20, 1.0))
    } else {
        None
    };
    let build_text_col = if is_build_mode {
        Some(Color::new(1.0, 0.95, 0.40, 1.0))
    } else {
        None
    };

    if draw_ascii_button_styled(
        build_top_text,
        build_top_btn_x,
        6.0,
        build_btn_w,
        25.0,
        mx,
        my,
        mouse_click,
        font,
        build_bg,
        build_border,
        build_text_col,
    ) {
        result.clicked_toggle_build_mode = true;
    }

    if draw_ascii_button(
        "[ 🎒 INVENTORY (I) ]",
        inv_btn_x,
        6.0,
        inv_btn_w,
        25.0,
        mx,
        my,
        mouse_click,
        font,
    ) {
        result.clicked_inventory = true;
    }

    if draw_ascii_button(
        "[ 💾 SAVE (F5) ]",
        save_btn_x,
        6.0,
        save_btn_w,
        25.0,
        mx,
        my,
        mouse_click,
        font,
    ) {
        result.clicked_save = true;
    }

    if draw_ascii_button(
        "[ 🚪 MENU ]",
        menu_btn_x,
        6.0,
        menu_btn_w,
        25.0,
        mx,
        my,
        mouse_click,
        font,
    ) {
        result.clicked_exit_to_menu = true;
    }

    // Floating blueprint HUD banner below header when Build Mode is active
    if is_build_mode {
        let mat_count = player.inventory.count_item(build_material.item_type());
        let banner_w = (screen_w - sidebar_w - 24.0).min(640.0);
        let banner_h = 24.0;
        let banner_x = 12.0;
        let banner_y = top_h + 6.0;
        draw_rectangle(banner_x, banner_y, banner_w, banner_h, Color::new(0.08, 0.07, 0.03, 0.94));
        draw_rectangle_lines(banner_x, banner_y, banner_w, banner_h, 1.5, Color::new(1.0, 0.80, 0.20, 1.0));
        let banner_str = format!(
            "🔨 MODO CONSTRUCCIÓN :: [{}] {} (x{}) | Clic Izq: Poner | Clic Der: Romper | R: Cambiar",
            build_material.glyph(), build_material.name_es(), mat_count
        );
        draw_text_ex(
            &banner_str,
            banner_x + 8.0,
            banner_y + 16.0,
            TextParams {
                font,
                font_size: 11,
                color: Color::new(1.0, 0.90, 0.40, 1.0),
                ..Default::default()
            },
        );
    }

    // ==========================================
    // 2. ASCII SIDEBAR
    // ==========================================
    draw_rectangle(
        sb_x,
        top_h,
        sidebar_w,
        screen_h - top_h - bottom_h,
        Color::new(0.05, 0.07, 0.10, 1.0),
    );
    draw_line(
        sb_x,
        top_h,
        sb_x,
        screen_h - bottom_h,
        1.2,
        Color::new(0.25, 0.35, 0.48, 1.0),
    );

    let mut cy = top_h + 12.0;

    // --- ASCII BOX: LAYER STRUCTURE (100 LEVELS) ---
    draw_ascii_header("+-- LAYER ELEVATOR (100 LEVELS) --+", sb_x + 10.0, cy, font);
    cy += 20.0;

    let z_view = camera.view_z;
    let z_player = player.z;

    let z_badge = format!("Current View: Z = {:+03}  |  Player: Z = {:+03}", z_view, z_player);
    draw_text_ex(
        &z_badge,
        sb_x + 14.0,
        cy + 10.0,
        TextParams {
            font,
            font_size: 12,
            color: Color::new(1.0, 0.85, 0.30, 1.0),
            ..Default::default()
        },
    );
    cy += 22.0;

    // ASCII Vertical / Horizontal Level Meter
    let meter_chars = 26;
    let norm_cam = ((z_view - Z_MIN) as f32 / 99.0).clamp(0.0, 1.0);
    let cam_idx = ((norm_cam * (meter_chars - 1) as f32).round() as usize).min(meter_chars - 1);
    let norm_plr = ((z_player - Z_MIN) as f32 / 99.0).clamp(0.0, 1.0);
    let plr_idx = ((norm_plr * (meter_chars - 1) as f32).round() as usize).min(meter_chars - 1);

    let mut gauge_str = String::with_capacity(meter_chars + 2);
    gauge_str.push('[');
    for i in 0..meter_chars {
        if i == cam_idx {
            gauge_str.push('▲'); // Camera needle
        } else if i == plr_idx {
            gauge_str.push('@'); // Player marker
        } else if i == 50 {
            gauge_str.push('|'); // Surface 0 marker
        } else if i < 50 {
            gauge_str.push('.'); // Underground dots
        } else {
            gauge_str.push('-'); // Sky lines
        }
    }
    gauge_str.push(']');

    draw_text_ex(
        &format!("-50 {} +49", gauge_str),
        sb_x + 12.0,
        cy + 10.0,
        TextParams {
            font,
            font_size: 12,
            color: Color::new(0.65, 0.80, 0.95, 1.0),
            ..Default::default()
        },
    );
    cy += 22.0;

    // ASCII Buttons for layers
    let btn_w = (sidebar_w - 32.0) / 2.0;
    let btn_h = 22.0;

    if draw_ascii_button(
        "[ ▲ UP LAYER ]",
        sb_x + 12.0,
        cy,
        btn_w,
        btn_h,
        mx,
        my,
        mouse_click,
        font,
    ) {
        result.clicked_up_layer = true;
    }

    if draw_ascii_button(
        "[ ▼ DN LAYER ]",
        sb_x + 20.0 + btn_w,
        cy,
        btn_w,
        btn_h,
        mx,
        my,
        mouse_click,
        font,
    ) {
        result.clicked_down_layer = true;
    }
    cy += btn_h + 5.0;

    if draw_ascii_button(
        "[ 0 SURFACE ]",
        sb_x + 12.0,
        cy,
        btn_w,
        btn_h,
        mx,
        my,
        mouse_click,
        font,
    ) {
        result.clicked_surface_layer = true;
    }

    if draw_ascii_button(
        "[ @ GOTO PLAYER ]",
        sb_x + 20.0 + btn_w,
        cy,
        btn_w,
        btn_h,
        mx,
        my,
        mouse_click,
        font,
    ) {
        result.clicked_player_layer = true;
    }
    cy += btn_h + 6.0;

    if draw_ascii_button(
        "[ 🗺️ FAST TRAVEL (Key M) ]",
        sb_x + 12.0,
        cy,
        sidebar_w - 24.0,
        22.0,
        mx,
        my,
        mouse_click,
        font,
    ) {
        result.clicked_fast_travel = true;
    }
    cy += 27.0;

    let half_w = (sidebar_w - 30.0) / 2.0;
    if draw_ascii_button(
        "[ 🎒 INVENTORY (I) ]",
        sb_x + 12.0,
        cy,
        half_w,
        22.0,
        mx,
        my,
        mouse_click,
        font,
    ) {
        result.clicked_inventory = true;
    }
    if draw_ascii_button(
        "[ 📡 RADAR (TAB) ]",
        sb_x + 18.0 + half_w,
        cy,
        half_w,
        22.0,
        mx,
        my,
        mouse_click,
        font,
    ) {
        result.clicked_radar = true;
    }
    cy += 28.0;

    // --- ASCII BOX: SURVIVOR STATUS ---
    draw_ascii_header("+-- SURVIVOR STATUS (@) -----------+", sb_x + 10.0, cy, font);
    cy += 18.0;

    let coords_str = format!("Position: X: {:03} | Y: {:03} | Z: {:+03}", player.x, player.y, player.z);
    draw_text_ex(
        &coords_str,
        sb_x + 14.0,
        cy + 10.0,
        TextParams {
            font,
            font_size: 12,
            color: Color::new(0.85, 0.90, 0.95, 1.0),
            ..Default::default()
        },
    );
    cy += 18.0;

    let main_hand_name = player
        .inventory
        .equipment
        .main_hand
        .map(|i| format!("[{}] {} (+{:.0} Atk)", i.glyph(), i.name(), i.weapon_damage()))
        .unwrap_or_else(|| "Fists (+4 Atk)".to_string());
    draw_text_ex(
        &format!("Weapon/Tool : {}", main_hand_name),
        sb_x + 14.0,
        cy + 10.0,
        TextParams {
            font,
            font_size: 11,
            color: Color::new(0.40, 0.85, 1.0, 1.0),
            ..Default::default()
        },
    );
    cy += 16.0;

    let armor_name = player
        .inventory
        .equipment
        .chest
        .map(|i| format!("[{}] {} (+{:.0} Def)", i.glyph(), i.name(), i.armor_defense()))
        .unwrap_or_else(|| "No Armor".to_string());
    draw_text_ex(
        &format!("Chest Armor : {}", armor_name),
        sb_x + 14.0,
        cy + 10.0,
        TextParams {
            font,
            font_size: 11,
            color: Color::new(0.60, 0.90, 0.65, 1.0),
            ..Default::default()
        },
    );
    cy += 18.0;

    // Pure ASCII Stat Bars: [████████░░]
    draw_ascii_stat_bar("HP ", player.health, player.max_health, sb_x + 14.0, cy, Color::new(0.9, 0.25, 0.25, 1.0), font);
    cy += 20.0;
    draw_ascii_stat_bar("HUN", player.hunger, player.max_hunger, sb_x + 14.0, cy, Color::new(0.95, 0.65, 0.15, 1.0), font);
    cy += 20.0;
    draw_ascii_stat_bar("THI", player.thirst, player.max_thirst, sb_x + 14.0, cy, Color::new(0.30, 0.70, 0.95, 1.0), font);
    cy += 20.0;
    draw_ascii_stat_bar("ENG", player.energy, player.max_energy, sb_x + 14.0, cy, Color::new(0.35, 0.85, 0.45, 1.0), font);
    cy += 22.0;

    // Hunt button in sidebar
    if draw_ascii_button(
        "[ 🏹 HUNT CREATURE (Key F) ]",
        sb_x + 12.0,
        cy,
        sidebar_w - 24.0,
        22.0,
        mx,
        my,
        mouse_click,
        font,
    ) {
        result.clicked_hunt = true;
    }
    cy += 26.0;

    // --- CONSTRUCCIÓN: MODO CONSTRUCCIÓN Y MATERIALES ---
    let b_btn_text = if is_build_mode {
        "[ 🔨 MODO CONSTRUCCIÓN: ACTIVO (B) ]"
    } else {
        "[ 🔨 MODO CONSTRUCCIÓN (Tecla B) ]"
    };
    let b_btn_bg = if is_build_mode {
        Some(Color::new(0.35, 0.22, 0.05, 0.95))
    } else {
        None
    };
    let b_btn_border = if is_build_mode {
        Some(Color::new(1.0, 0.85, 0.20, 1.0))
    } else {
        None
    };
    let b_btn_txt_col = if is_build_mode {
        Some(Color::new(1.0, 0.95, 0.40, 1.0))
    } else {
        None
    };

    if draw_ascii_button_styled(
        b_btn_text,
        sb_x + 12.0,
        cy,
        sidebar_w - 24.0,
        24.0,
        mx,
        my,
        mouse_click,
        font,
        b_btn_bg,
        b_btn_border,
        b_btn_txt_col,
    ) {
        result.clicked_toggle_build_mode = true;
    }
    cy += 28.0;

    // Selector de materiales con cantidades disponibles
    let planks_count = player.inventory.count_item(ItemType::WoodPlanks);
    let cobble_count = player.inventory.count_item(ItemType::Cobblestone);
    let dirt_count = player.inventory.count_item(ItemType::DirtBlock);

    let third_w = (sidebar_w - 36.0) / 3.0;

    let p_sel = is_build_mode && build_material == BuildMaterial::WoodPlanks;
    let c_sel = is_build_mode && build_material == BuildMaterial::Cobblestone;
    let d_sel = is_build_mode && build_material == BuildMaterial::DirtBlock;

    let p_bg = if p_sel { Some(Color::new(0.32, 0.22, 0.10, 0.95)) } else { None };
    let p_bor = if p_sel { Some(Color::new(0.95, 0.70, 0.30, 1.0)) } else { None };

    let c_bg = if c_sel { Some(Color::new(0.24, 0.25, 0.32, 0.95)) } else { None };
    let c_bor = if c_sel { Some(Color::new(0.75, 0.80, 0.95, 1.0)) } else { None };

    let d_bg = if d_sel { Some(Color::new(0.28, 0.20, 0.12, 0.95)) } else { None };
    let d_bor = if d_sel { Some(Color::new(0.80, 0.60, 0.35, 1.0)) } else { None };

    let p_text = format!("[🧱# Tab:{}]", planks_count);
    let c_text = format!("[🏰O Ado:{}]", cobble_count);
    let d_text = format!("[🚜. Tie:{}]", dirt_count);

    if draw_ascii_button_styled(
        &p_text,
        sb_x + 12.0,
        cy,
        third_w,
        22.0,
        mx,
        my,
        mouse_click,
        font,
        p_bg,
        p_bor,
        if p_sel { Some(Color::new(1.0, 0.90, 0.50, 1.0)) } else { None },
    ) {
        result.clicked_select_material = Some(BuildMaterial::WoodPlanks);
        result.clicked_build_planks = true;
    }

    if draw_ascii_button_styled(
        &c_text,
        sb_x + 18.0 + third_w,
        cy,
        third_w,
        22.0,
        mx,
        my,
        mouse_click,
        font,
        c_bg,
        c_bor,
        if c_sel { Some(Color::new(1.0, 0.90, 0.50, 1.0)) } else { None },
    ) {
        result.clicked_select_material = Some(BuildMaterial::Cobblestone);
        result.clicked_build_cobble = true;
    }

    if draw_ascii_button_styled(
        &d_text,
        sb_x + 24.0 + third_w * 2.0,
        cy,
        third_w,
        22.0,
        mx,
        my,
        mouse_click,
        font,
        d_bg,
        d_bor,
        if d_sel { Some(Color::new(1.0, 0.90, 0.50, 1.0)) } else { None },
    ) {
        result.clicked_select_material = Some(BuildMaterial::DirtBlock);
    }
    cy += 26.0;

    // Botón directo de Demoler / Romper
    if draw_ascii_button(
        "[ 💥 ROMPER / DEMOLER (Clic Der / X) ]",
        sb_x + 12.0,
        cy,
        sidebar_w - 24.0,
        22.0,
        mx,
        my,
        mouse_click,
        font,
    ) {
        result.clicked_harvest_demolish = true;
    }
    cy += 26.0;

    // Contextual Drink Water action
    if player.can_drink_water(world) {
        if draw_ascii_button(
            "[ 💧 DRINK FRESHWATER (Key E) ]",
            sb_x + 12.0,
            cy,
            sidebar_w - 24.0,
            24.0,
            mx,
            my,
            mouse_click,
            font,
        ) {
            result.clicked_drink = true;
        }
        cy += 28.0;
    } else {
        cy += 4.0;
    }

    // --- ASCII BOX: TERRAIN INSPECTOR ---
    draw_ascii_header("+-- TERRAIN INSPECTOR (CURSOR) -----+", sb_x + 10.0, cy, font);
    cy += 18.0;

    if let Some((hx, hy, hz, t)) = hover_tile {
        let name_str = format!("Tile:  {} [{}]", t.name, t.glyph);
        draw_text_ex(
            &name_str,
            sb_x + 14.0,
            cy + 10.0,
            TextParams {
                font,
                font_size: 13,
                color: Color::new(1.0, 1.0, 1.0, 1.0),
                ..Default::default()
            },
        );
        cy += 18.0;

        let biome_info = t.biome.info();
        let biome_str = format!("Biome: {}", biome_info.name);
        draw_text_ex(
            &biome_str,
            sb_x + 14.0,
            cy + 10.0,
            TextParams {
                font,
                font_size: 12,
                color: biome_info.ambient_color,
                ..Default::default()
            },
        );
        cy += 18.0;

        let walk_str = if t.walkable {
            format!("Path:  Passable (Cost: {:.1})", t.move_cost)
        } else {
            "Path:  Impassable / Blocked".to_string()
        };
        let walk_col = if t.walkable {
            Color::new(0.35, 0.90, 0.35, 1.0)
        } else {
            Color::new(0.95, 0.35, 0.35, 1.0)
        };
        draw_text_ex(
            &walk_str,
            sb_x + 14.0,
            cy + 10.0,
            TextParams {
                font,
                font_size: 12,
                color: walk_col,
                ..Default::default()
            },
        );
        cy += 18.0;

        let loc_str = format!("Coord: (X:{:03}, Y:{:03}, Z:{:+03})", hx, hy, hz);
        draw_text_ex(
            &loc_str,
            sb_x + 14.0,
            cy + 10.0,
            TextParams {
                font,
                font_size: 11,
                color: Color::new(0.65, 0.75, 0.85, 1.0),
                ..Default::default()
            },
        );
        cy += 18.0;

        // Biome description
        draw_text_ex(
            biome_info.description,
            sb_x + 14.0,
            cy + 10.0,
            TextParams {
                font,
                font_size: 10,
                color: Color::new(0.70, 0.75, 0.80, 0.90),
                ..Default::default()
            },
        );
        cy += 20.0;
    } else {
        draw_text_ex(
            "Hover mouse over any map cell",
            sb_x + 14.0,
            cy + 12.0,
            TextParams {
                font,
                font_size: 12,
                color: Color::new(0.50, 0.55, 0.60, 1.0),
                ..Default::default()
            },
        );
        cy += 40.0;
    }

    // --- ASCII BOX: SURVIVAL LOG ---
    draw_ascii_header("+-- SURVIVAL CHRONICLE -------------+", sb_x + 10.0, cy, font);
    cy += 18.0;

    let log_box_h = (screen_h - bottom_h - cy - 8.0).max(50.0);
    draw_rectangle(
        sb_x + 10.0,
        cy,
        sidebar_w - 20.0,
        log_box_h,
        Color::new(0.04, 0.05, 0.07, 0.90),
    );
    draw_rectangle_lines(
        sb_x + 10.0,
        cy,
        sidebar_w - 20.0,
        log_box_h,
        1.0,
        Color::new(0.20, 0.28, 0.38, 1.0),
    );

    let mut log_y = cy + 12.0;
    for (msg, _) in ui_state.log_messages.iter().rev().take(5) {
        if log_y + 12.0 > cy + log_box_h {
            break;
        }
        draw_text_ex(
            &format!("> {}", msg),
            sb_x + 16.0,
            log_y,
            TextParams {
                font,
                font_size: 11,
                color: Color::new(0.75, 0.82, 0.88, 0.95),
                ..Default::default()
            },
        );
        log_y += 15.0;
    }

    // ==========================================
    // 3. ASCII FOOTER / CONTROLS HINT
    // ==========================================
    let foot_y = screen_h - bottom_h;
    draw_rectangle(
        0.0,
        foot_y,
        screen_w,
        bottom_h,
        Color::new(0.05, 0.07, 0.10, 1.0),
    );
    draw_line(
        0.0,
        foot_y,
        screen_w,
        foot_y,
        1.0,
        Color::new(0.22, 0.30, 0.40, 1.0),
    );

    let help_text = if is_build_mode {
        "[🔨 MODO CONSTRUCCIÓN] [Clic Izq: Colocar bloque] [Clic Der: Romper/Demoler] [R: Cambiar Material] [B: Salir] [I: Inv]"
    } else {
        "[B: Modo Construcción] [Clic Izq: Mover/Ruta] [Clic Der: Pan Cámara] [E: Interact/Skin] [F: Cazar] [I: Inv]"
    };
    draw_text_ex(
        help_text,
        14.0,
        foot_y + 18.0,
        TextParams {
            font,
            font_size: 12,
            color: if is_build_mode { Color::new(1.0, 0.85, 0.35, 1.0) } else { Color::new(0.65, 0.75, 0.85, 1.0) },
            ..Default::default()
        },
    );

    result
}

fn draw_ascii_header(text: &str, x: f32, y: f32, font: Option<&Font>) {
    draw_text_ex(
        text,
        x,
        y,
        TextParams {
            font,
            font_size: 12,
            color: Color::new(0.40, 0.75, 0.95, 1.0),
            ..Default::default()
        },
    );
}

fn draw_ascii_stat_bar(
    label: &str,
    val: f32,
    max: f32,
    x: f32,
    y: f32,
    color: Color,
    font: Option<&Font>,
) {
    let ratio = (val / max).clamp(0.0, 1.0);
    let total_blocks = 14;
    let filled_blocks = ((ratio * total_blocks as f32).round() as usize).min(total_blocks);

    let mut bar_str = String::with_capacity(total_blocks + 2);
    bar_str.push('[');
    for i in 0..total_blocks {
        if i < filled_blocks {
            bar_str.push('█');
        } else {
            bar_str.push('░');
        }
    }
    bar_str.push(']');

    let display = format!("{}: {} {:3.0}/{:3.0}", label, bar_str, val, max);

    draw_text_ex(
        &display,
        x,
        y + 9.0,
        TextParams {
            font,
            font_size: 12,
            color,
            ..Default::default()
        },
    );
}

fn draw_ascii_button(
    text: &str,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    mx: f32,
    my: f32,
    click: bool,
    font: Option<&Font>,
) -> bool {
    draw_ascii_button_styled(text, x, y, w, h, mx, my, click, font, None, None, None)
}

fn draw_ascii_button_styled(
    text: &str,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    mx: f32,
    my: f32,
    click: bool,
    font: Option<&Font>,
    custom_bg: Option<Color>,
    custom_border: Option<Color>,
    custom_text_col: Option<Color>,
) -> bool {
    let hovered = mx >= x && mx <= (x + w) && my >= y && my <= (y + h);

    let bg = if hovered {
        Color::new(0.20, 0.30, 0.42, 1.0)
    } else if let Some(c) = custom_bg {
        c
    } else {
        Color::new(0.10, 0.14, 0.20, 1.0)
    };

    let border = if hovered {
        Color::new(0.95, 0.85, 0.30, 1.0)
    } else if let Some(b) = custom_border {
        b
    } else {
        Color::new(0.25, 0.35, 0.48, 1.0)
    };

    draw_rectangle(x, y, w, h, bg);
    draw_rectangle_lines(x, y, w, h, 1.0, border);

    let font_size = 11;
    let dim = measure_text(text, font, font_size, 1.0);
    let tx = x + (w - dim.width) * 0.5;
    let ty = y + (h + dim.height * 0.70) * 0.5;

    let text_col = if hovered {
        Color::new(1.0, 1.0, 1.0, 1.0)
    } else if let Some(tc) = custom_text_col {
        tc
    } else {
        Color::new(0.85, 0.90, 0.95, 1.0)
    };

    draw_text_ex(
        text,
        tx,
        ty,
        TextParams {
            font,
            font_size,
            color: text_col,
            ..Default::default()
        },
    );

    hovered && click
}
