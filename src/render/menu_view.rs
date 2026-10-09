use crate::save::SaveGame;
use macroquad::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MenuAction {
    None,
    NewGame,
    LoadGame,
    Exit,
}

pub struct MenuState {
    pub selected_index: usize,
}

impl MenuState {
    pub fn new() -> Self {
        Self { selected_index: 0 }
    }
}

pub fn render_main_menu(
    cover_texture: &Texture2D,
    state: &mut MenuState,
    font: Option<&Font>,
) -> MenuAction {
    let sw = screen_width();
    let sh = screen_height();
    let time = get_time() as f32;

    let mouse_pos = mouse_position();
    let (mx, my) = (mouse_pos.0, mouse_pos.1);
    let mouse_clicked = is_mouse_button_pressed(MouseButton::Left);

    // =========================================================================
    // 1. DRAW COVER JPG ARTWORK (Full Screen Atmospheric Portada)
    // =========================================================================
    // Clear screen to deep abyss darkness
    draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.02, 0.03, 0.05, 1.0));

    let img_w = cover_texture.width();
    let img_h = cover_texture.height();

    if img_w > 0.0 && img_h > 0.0 {
        // Scale to cover or fit seamlessly with cinematic aspect ratio
        let scale = (sw / img_w).max(sh / img_h);
        let draw_w = img_w * scale;
        let draw_h = img_h * scale;
        let draw_x = (sw - draw_w) * 0.5;
        let draw_y = (sh - draw_h) * 0.5;

        // Draw cover sprite with vibrant atmospheric lighting
        draw_texture_ex(
            cover_texture,
            draw_x,
            draw_y,
            Color::new(0.92, 0.92, 0.96, 0.90),
            DrawTextureParams {
                dest_size: Some(vec2(draw_w, draw_h)),
                ..Default::default()
            },
        );
    }

    // Atmospheric scrim / dark vignette overlay for maximum ASCII contrast
    draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.02, 0.03, 0.06, 0.52));

    // Top & bottom letterbox bars
    draw_rectangle(0.0, 0.0, sw, 32.0, Color::new(0.01, 0.02, 0.04, 0.90));
    draw_rectangle(0.0, sh - 28.0, sw, 28.0, Color::new(0.01, 0.02, 0.04, 0.92));

    // Decorative top header line
    let top_info = "DIE BY GLORY :: 100-LAYER 3D VOLUMETRIC ASCII SURVIVAL ENGINE :: RUST EDITION v0.2.0-ALPHA";
    let top_dim = measure_text(top_info, font, 12, 1.0);
    draw_text_ex(
        top_info,
        (sw - top_dim.width) * 0.5,
        21.0,
        TextParams {
            font,
            font_size: 12,
            color: Color::new(0.40, 0.75, 0.95, 1.0),
            ..Default::default()
        },
    );

    // =========================================================================
    // 2. CHECK PERSISTENT SAVEGAME STATE
    // =========================================================================
    let save_game_opt = SaveGame::load();
    let has_save = save_game_opt.is_some();

    // =========================================================================
    // 3. KEYBOARD NAVIGATION
    // =========================================================================
    if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) {
        if state.selected_index == 0 {
            state.selected_index = 2;
        } else {
            state.selected_index -= 1;
        }
        // If option 1 (Load) is selected but no save exists, skip it
        if state.selected_index == 1 && !has_save {
            state.selected_index = 0;
        }
    }

    if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) {
        if state.selected_index >= 2 {
            state.selected_index = 0;
        } else {
            state.selected_index += 1;
        }
        // If option 1 (Load) is selected but no save exists, skip it
        if state.selected_index == 1 && !has_save {
            state.selected_index = 2;
        }
    }

    // Direct Hotkeys
    if is_key_pressed(KeyCode::N) || is_key_pressed(KeyCode::Key1) {
        return MenuAction::NewGame;
    }
    if (is_key_pressed(KeyCode::L) || is_key_pressed(KeyCode::Key2)) && has_save {
        return MenuAction::LoadGame;
    }
    if is_key_pressed(KeyCode::Q) || is_key_pressed(KeyCode::Key3) || is_key_pressed(KeyCode::Escape) {
        return MenuAction::Exit;
    }

    let activate_selected = is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space);
    if activate_selected {
        match state.selected_index {
            0 => return MenuAction::NewGame,
            1 => {
                if has_save {
                    return MenuAction::LoadGame;
                }
            }
            2 => return MenuAction::Exit,
            _ => {}
        }
    }

    // =========================================================================
    // 4. MAIN MENU TERMINAL CARD (ASCII Centered Panel)
    // =========================================================================
    let card_w = 760.0f32.min(sw - 36.0);
    let card_h = 570.0f32.min(sh - 70.0);
    let card_x = (sw - card_w) * 0.5;
    let card_y = 36.0 + (sh - 64.0 - card_h) * 0.5;

    // Card background fill with rich dark slate glassmorphism
    draw_rectangle(card_x, card_y, card_w, card_h, Color::new(0.04, 0.05, 0.08, 0.90));
    // ASCII Outer border
    draw_rectangle_lines(card_x, card_y, card_w, card_h, 2.0, Color::new(0.35, 0.55, 0.78, 1.0));
    // Inner accent line
    draw_rectangle_lines(card_x + 3.0, card_y + 3.0, card_w - 6.0, card_h - 6.0, 1.0, Color::new(0.18, 0.28, 0.42, 0.75));

    let mut cy = card_y + 18.0;

    // --- ASCII TITLE LOGO ---
    let logo_lines = [
        r" ____  ___ _____   ______   __   ____ _     ___  ______   __ ",
        r"|  _ \|_ _| ____| | __ ) \ / /  / ___| |   / _ \|  _ \ \ / / ",
        r"| | | || ||  _|   |  _ \\ V /  | |  _| |  | | | | |_) \ V /  ",
        r"| |_| || || |___  | |_) || |   | |_| | |__| |_| |  _ < | |   ",
        r"|____/|___|_____| |____/ |_|    \____|_____\___/|_| \_\|_|   ",
    ];

    // Pulsing subtle glow
    let glow_pulse = (time * 2.0).sin() * 0.08;
    let title_color = Color::new(1.0, 0.85 + glow_pulse, 0.30 + glow_pulse, 1.0);

    for line in &logo_lines {
        let dim = measure_text(line, font, 12, 1.0);
        let lx = card_x + (card_w - dim.width) * 0.5;
        // Subtle drop shadow
        draw_text_ex(
            line,
            lx + 1.0,
            cy + 1.0,
            TextParams {
                font,
                font_size: 12,
                color: Color::new(0.0, 0.0, 0.0, 0.8),
                ..Default::default()
            },
        );
        draw_text_ex(
            line,
            lx,
            cy,
            TextParams {
                font,
                font_size: 12,
                color: title_color,
                ..Default::default()
            },
        );
        cy += 14.0;
    }

    cy += 6.0;
    let subtitle = ":: 100-LAYER VOXEL SURVIVAL :: 3D TERRAIN RELIEF :: OVERWORLD CONTINENTS ::";
    let sub_dim = measure_text(subtitle, font, 11, 1.0);
    draw_text_ex(
        subtitle,
        card_x + (card_w - sub_dim.width) * 0.5,
        cy,
        TextParams {
            font,
            font_size: 11,
            color: Color::new(0.45, 0.78, 0.95, 1.0),
            ..Default::default()
        },
    );
    cy += 14.0;

    // Horizontal separator
    draw_line(card_x + 20.0, cy, card_x + card_w - 20.0, cy, 1.0, Color::new(0.25, 0.38, 0.52, 0.8));
    cy += 16.0;

    // =========================================================================
    // 5. INTERACTIVE MENU BUTTONS
    // =========================================================================
    let btn_w = card_w - 60.0;
    let btn_h = 44.0;
    let btn_x = card_x + 30.0;

    // --- BUTTON 0: NEW GAME ---
    let b0_active = state.selected_index == 0;
    let (b0_hovered, b0_clicked) = draw_menu_button(
        btn_x,
        cy,
        btn_w,
        btn_h,
        "⚔  START NEW EXPEDITION  [Key N / 1]",
        "Generate pristine continents, explore biomes, and embark into the 100-layer world.",
        b0_active,
        true,
        mx,
        my,
        mouse_clicked,
        font,
    );
    if b0_hovered {
        state.selected_index = 0;
    }
    if b0_clicked {
        return MenuAction::NewGame;
    }
    cy += btn_h + 10.0;

    // --- BUTTON 1: LOAD GAME ---
    let b1_active = state.selected_index == 1;
    let load_title = if has_save {
        "💾  LOAD SAVED EXPEDITION  [Key L / 2]"
    } else {
        "💾  LOAD EXPEDITION  [NO SAVE FILE FOUND]"
    };
    let load_desc = if let Some(ref save) = save_game_opt {
        format!(
            "Resume: Continent (X:{:02}, Y:{:02}) • {} • Clock: {} • HP: {}%",
            save.ow_current_x,
            save.ow_current_y,
            save.current_biome.info().name,
            save.saved_at_display,
            save.health as i32
        )
    } else {
        "No savegame.json file was detected. Start a new expedition first.".to_string()
    };

    let (b1_hovered, b1_clicked) = draw_menu_button(
        btn_x,
        cy,
        btn_w,
        btn_h,
        load_title,
        &load_desc,
        b1_active,
        has_save,
        mx,
        my,
        mouse_clicked,
        font,
    );
    if b1_hovered && has_save {
        state.selected_index = 1;
    }
    if b1_clicked && has_save {
        return MenuAction::LoadGame;
    }
    cy += btn_h + 10.0;

    // --- BUTTON 2: EXIT GAME ---
    let b2_active = state.selected_index == 2;
    let (b2_hovered, b2_clicked) = draw_menu_button(
        btn_x,
        cy,
        btn_w,
        btn_h,
        "❌  EXIT GAME  [Key Q / ESC / 3]",
        "Close Die by Glory and return cleanly to desktop.",
        b2_active,
        true,
        mx,
        my,
        mouse_clicked,
        font,
    );
    if b2_hovered {
        state.selected_index = 2;
    }
    if b2_clicked {
        return MenuAction::Exit;
    }
    cy += btn_h + 14.0;

    // =========================================================================
    // 6. DETECTED SAVE EXPEDITION DOSSIER CARD (If available)
    // =========================================================================
    if let Some(ref save) = save_game_opt {
        let dossier_h = 100.0;
        draw_rectangle(
            btn_x,
            cy,
            btn_w,
            dossier_h,
            Color::new(0.06, 0.08, 0.12, 0.85),
        );
        draw_rectangle_lines(
            btn_x,
            cy,
            btn_w,
            dossier_h,
            1.0,
            Color::new(0.28, 0.42, 0.58, 0.9),
        );

        let d_title = "+-- [ 📜 LAST SAVED EXPEDITION DOSSIER ] ------------------------------------+";
        draw_text_ex(
            d_title,
            btn_x + 10.0,
            cy + 16.0,
            TextParams {
                font,
                font_size: 11,
                color: Color::new(0.40, 0.80, 1.0, 1.0),
                ..Default::default()
            },
        );

        let row1 = format!(
            "• Station: Continent (X:{:02}, Y:{:02})  |  Biome: {}  |  Seed: {}",
            save.ow_current_x, save.ow_current_y, save.current_biome.info().name, save.overworld_seed
        );
        let row2 = format!(
            "• Position: Stage (X:{:03}, Y:{:03}, Z:{:+03})  |  Local Time: {}",
            save.player_x, save.player_y, save.player_z, save.saved_at_display
        );
        let row3 = format!(
            "• Survivor Vitals: HP {}% | Hunger {}% | Thirst {}% | Stamina {}%",
            save.health as i32, save.hunger as i32, save.thirst as i32, save.energy as i32
        );

        draw_text_ex(
            &row1,
            btn_x + 14.0,
            cy + 36.0,
            TextParams {
                font,
                font_size: 11,
                color: Color::new(0.85, 0.90, 0.95, 1.0),
                ..Default::default()
            },
        );
        draw_text_ex(
            &row2,
            btn_x + 14.0,
            cy + 56.0,
            TextParams {
                font,
                font_size: 11,
                color: Color::new(0.70, 0.82, 0.92, 1.0),
                ..Default::default()
            },
        );
        draw_text_ex(
            &row3,
            btn_x + 14.0,
            cy + 76.0,
            TextParams {
                font,
                font_size: 11,
                color: Color::new(0.55, 0.95, 0.65, 1.0),
                ..Default::default()
            },
        );
    } else {
        // Tip box when no save exists
        let tip_h = 70.0;
        draw_rectangle(
            btn_x,
            cy,
            btn_w,
            tip_h,
            Color::new(0.06, 0.08, 0.12, 0.70),
        );
        draw_rectangle_lines(
            btn_x,
            cy,
            btn_w,
            tip_h,
            1.0,
            Color::new(0.25, 0.35, 0.45, 0.7),
        );
        let tip_title = "+-- [ ℹ SURVIVAL EXPEDITION BRIEFING ] ---------------------------------------+";
        let tip_body1 = "• Begin by choosing your landing continent on the Overworld map.";
        let tip_body2 = "• Press F5 anytime in-game or travel beyond borders to save your voyage.";
        draw_text_ex(tip_title, btn_x + 10.0, cy + 16.0, TextParams { font, font_size: 11, color: Color::new(0.4, 0.75, 0.95, 1.0), ..Default::default() });
        draw_text_ex(tip_body1, btn_x + 14.0, cy + 36.0, TextParams { font, font_size: 11, color: Color::new(0.8, 0.85, 0.9, 1.0), ..Default::default() });
        draw_text_ex(tip_body2, btn_x + 14.0, cy + 54.0, TextParams { font, font_size: 11, color: Color::new(0.7, 0.8, 0.85, 1.0), ..Default::default() });
    }

    // =========================================================================
    // 7. BOTTOM HINTS & FOOTER
    // =========================================================================
    let hints = "[ ↑/↓ or W/S: Navigate ]  [ ENTER / SPACE: Select ]  [ N: New Game ]  [ L: Load ]  [ Q / ESC: Quit ]";
    draw_text_ex(
        hints,
        (sw - measure_text(hints, font, 11, 1.0).width) * 0.5,
        sh - 10.0,
        TextParams {
            font,
            font_size: 11,
            color: Color::new(0.60, 0.72, 0.85, 1.0),
            ..Default::default()
        },
    );

    MenuAction::None
}

fn draw_menu_button(
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    title: &str,
    desc: &str,
    is_active: bool,
    is_enabled: bool,
    mx: f32,
    my: f32,
    clicked: bool,
    font: Option<&Font>,
) -> (bool, bool) {
    let hovered = is_enabled && mx >= x && mx <= (x + w) && my >= y && my <= (y + h);
    let highlighted = is_active || hovered;

    let (bg_color, border_color, title_color, desc_color) = if !is_enabled {
        (
            Color::new(0.08, 0.09, 0.12, 0.7),
            Color::new(0.20, 0.22, 0.28, 0.8),
            Color::new(0.45, 0.48, 0.55, 0.8),
            Color::new(0.35, 0.38, 0.44, 0.7),
        )
    } else if highlighted {
        (
            Color::new(0.18, 0.28, 0.44, 0.95),
            Color::new(1.0, 0.85, 0.30, 1.0),
            Color::new(1.0, 0.98, 0.60, 1.0),
            Color::new(0.85, 0.95, 1.0, 1.0),
        )
    } else {
        (
            Color::new(0.09, 0.13, 0.20, 0.85),
            Color::new(0.30, 0.45, 0.62, 0.85),
            Color::new(0.88, 0.92, 0.98, 1.0),
            Color::new(0.60, 0.72, 0.85, 0.85),
        )
    };

    draw_rectangle(x, y, w, h, bg_color);
    let border_thickness = if highlighted && is_enabled { 2.0 } else { 1.2 };
    draw_rectangle_lines(x, y, w, h, border_thickness, border_color);

    // Indicator cursor on left side
    let indicator = if highlighted && is_enabled { "▶ " } else { "  " };
    let full_title = format!("{}{}", indicator, title);

    draw_text_ex(
        &full_title,
        x + 14.0,
        y + 18.0,
        TextParams {
            font,
            font_size: 13,
            color: title_color,
            ..Default::default()
        },
    );

    draw_text_ex(
        desc,
        x + 28.0,
        y + 34.0,
        TextParams {
            font,
            font_size: 10,
            color: desc_color,
            ..Default::default()
        },
    );

    (hovered, hovered && clicked)
}
