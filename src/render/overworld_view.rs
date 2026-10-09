use crate::save::SaveGame;
use crate::world::overworld::{Overworld, OW_HEIGHT, OW_WIDTH};
use crate::world::tile::BiomeKind;
use macroquad::prelude::*;

pub struct OverworldViewResult {
    pub embark: bool,
    pub selected_biome: BiomeKind,
    pub regenerate: bool,
    pub load_saved_game: bool,
    pub return_to_main_menu: bool,
}

pub fn render_overworld_screen(
    ow: &mut Overworld,
    font: Option<&Font>,
    is_fast_travel: bool,
) -> OverworldViewResult {
    let screen_w = screen_width();
    let screen_h = screen_height();
    let time = get_time() as f32;

    // Dark terminal ASCII background
    draw_rectangle(0.0, 0.0, screen_w, screen_h, Color::new(0.04, 0.05, 0.07, 1.0));

    let mut result = OverworldViewResult {
        embark: false,
        selected_biome: ow.current_selected_tile().map(|t| t.biome).unwrap_or(BiomeKind::Meadow),
        regenerate: false,
        load_saved_game: false,
        return_to_main_menu: false,
    };

    let mouse_pos = mouse_position();
    let (mx, my) = (mouse_pos.0, mouse_pos.1);
    let mouse_clicked = is_mouse_button_pressed(MouseButton::Left);

    // ==========================================
    // 1. ASCII HEADER BANNER
    // ==========================================
    let top_y = 12.0;
    let header_text = "+======================================================================================================+";
    let title_line = if is_fast_travel {
        "|                    DIE BY GLORY :: CONTINENTAL FAST TRAVEL (VIAJE RAPIDO)                            |"
    } else {
        "|                               DIE BY GLORY :: OVERWORLD EXPEDITION                                   |"
    };
    let sub_line = if is_fast_travel {
        "|     Move to an ADJACENT region from your current station. Next embark becomes your new baseline!      |"
    } else {
        "|      Select a continent or biome to embark into a 100-layer voxel simulation (ASCII Roguelike)       |"
    };
    let footer_line = "+======================================================================================================+";

    draw_ascii_line(header_text, 18.0, top_y, Color::new(0.40, 0.70, 0.90, 1.0), font, 13);
    draw_ascii_line(title_line, 18.0, top_y + 16.0, Color::new(1.0, 0.85, 0.25, 1.0), font, 13);
    draw_ascii_line(sub_line, 18.0, top_y + 32.0, Color::new(0.70, 0.80, 0.90, 1.0), font, 13);
    draw_ascii_line(footer_line, 18.0, top_y + 48.0, Color::new(0.40, 0.70, 0.90, 1.0), font, 13);

    // ==========================================
    // 2. OVERWORLD ASCII MAP
    // ==========================================
    let map_start_x = 24.0;
    let map_start_y = 80.0;
    let cell_w = 12.0;
    let cell_h = 17.0;

    let map_pixel_w = OW_WIDTH as f32 * cell_w;
    let map_pixel_h = OW_HEIGHT as f32 * cell_h;

    draw_rectangle_lines(
        map_start_x - 3.0,
        map_start_y - 3.0,
        map_pixel_w + 6.0,
        map_pixel_h + 6.0,
        1.5,
        Color::new(0.30, 0.45, 0.60, 1.0),
    );

    let mut hovered_coord = None;
    if mx >= map_start_x && mx < (map_start_x + map_pixel_w)
        && my >= map_start_y && my < (map_start_y + map_pixel_h)
    {
        let hx = ((mx - map_start_x) / cell_w) as usize;
        let hy = ((my - map_start_y) / cell_h) as usize;
        if hx < OW_WIDTH && hy < OW_HEIGHT {
            hovered_coord = Some((hx, hy));
            if mouse_clicked {
                ow.cursor_x = hx;
                ow.cursor_y = hy;
            }
        }
    }

    // Render overworld tiles
    for y in 0..OW_HEIGHT {
        for x in 0..OW_WIDTH {
            if let Some(t) = ow.get_tile(x, y) {
                let px = map_start_x + x as f32 * cell_w;
                let py = map_start_y + y as f32 * cell_h;

                let is_cursor = x == ow.cursor_x && y == ow.cursor_y;
                let is_hover = hovered_coord == Some((x, y));
                let is_station = ow.current_station == Some((x, y));
                let is_valid_step = is_fast_travel && ow.is_adjacent_to_current_station(x, y) && t.is_land;

                let bg = if is_cursor {
                    let pulse = ((time * 5.0).sin() * 0.25) + 0.65;
                    Color::new(0.95, 0.80, 0.20, pulse)
                } else if is_station {
                    let pulse = ((time * 4.0).sin() * 0.20) + 0.60;
                    Color::new(0.3, 0.7, 1.0, pulse)
                } else if is_valid_step {
                    Color::new(0.12, 0.35, 0.22, 0.90) // Highlight valid neighboring regions
                } else if is_hover {
                    Color::new(0.30, 0.40, 0.55, 0.90)
                } else {
                    t.bg_color
                };

                draw_rectangle(px, py, cell_w, cell_h, bg);

                // If valid step, draw subtle outline
                if is_valid_step && !is_cursor {
                    draw_rectangle_lines(px, py, cell_w, cell_h, 1.0, Color::new(0.3, 0.9, 0.5, 0.6));
                }

                let (glyph, fg) = if is_cursor {
                    (t.glyph, Color::new(0.05, 0.05, 0.05, 1.0))
                } else if is_station {
                    ('@', Color::new(1.0, 1.0, 1.0, 1.0)) // Current player camp on continent
                } else {
                    (t.glyph, t.fg_color)
                };

                let char_str = glyph.to_string();
                let text_dim = measure_text(&char_str, font, 14, 1.0);
                let tx = px + (cell_w - text_dim.width) * 0.5;
                let ty = py + (cell_h + text_dim.height * 0.70) * 0.5;

                draw_text_ex(
                    &char_str,
                    tx,
                    ty,
                    TextParams {
                        font,
                        font_size: 14,
                        color: fg,
                        ..Default::default()
                    },
                );
            }
        }
    }

    // ==========================================
    // 3. ASCII SIDE PANEL (INSPECTOR & CONTROLS)
    // ==========================================
    let panel_x = map_start_x + map_pixel_w + 24.0;
    let panel_w = screen_w - panel_x - 24.0;
    let mut py = map_start_y;

    let selected_tile = ow.current_selected_tile();
    let current_biome = selected_tile.map(|t| t.biome).unwrap_or(BiomeKind::Meadow);
    result.selected_biome = current_biome;
    let biome_info = current_biome.info();

    let (can_embark, embark_reason) = ow.can_embark_at(ow.cursor_x, ow.cursor_y, is_fast_travel);

    // Inspector Box
    draw_ascii_box("SELECTED REGION", panel_x, py, panel_w, 160.0, font);
    py += 24.0;

    let coord_str = format!("Coordinates: X: {:02} | Y: {:02}", ow.cursor_x, ow.cursor_y);
    draw_ascii_line(&coord_str, panel_x + 12.0, py, Color::new(0.8, 0.85, 0.9, 1.0), font, 13);
    py += 20.0;

    let biome_str = format!("Biome: {} [{}]", biome_info.name, biome_info.glyph);
    draw_ascii_line(&biome_str, panel_x + 12.0, py, biome_info.ambient_color, font, 14);
    py += 20.0;

    // Movement adjacency status
    let status_col = if can_embark {
        Color::new(0.3, 0.9, 0.4, 1.0)
    } else {
        Color::new(0.95, 0.35, 0.35, 1.0)
    };
    draw_ascii_line(embark_reason, panel_x + 12.0, py, status_col, font, 11);
    py += 22.0;

    draw_ascii_line(biome_info.description, panel_x + 12.0, py, Color::new(0.7, 0.75, 0.8, 1.0), font, 11);
    py += 80.0;

    // Fast travel or Quick Biome box
    if is_fast_travel {
        draw_ascii_box("FAST TRAVEL RULES", panel_x, py, panel_w, 110.0, font);
        py += 24.0;

        let rule1 = "> Base Station: [@]";
        let rule2 = "> Valid Targets: adjacent neighboring cells";
        let rule3 = "> Embarking sets your NEW continental origin!";
        draw_ascii_line(rule1, panel_x + 12.0, py, Color::new(0.4, 0.8, 1.0, 1.0), font, 12);
        py += 20.0;
        draw_ascii_line(rule2, panel_x + 12.0, py, Color::new(0.3, 0.9, 0.5, 1.0), font, 11);
        py += 20.0;
        draw_ascii_line(rule3, panel_x + 12.0, py, Color::new(0.9, 0.85, 0.4, 1.0), font, 11);
        py += 40.0;
    } else {
        draw_ascii_box("QUICK BIOME SELECT", panel_x, py, panel_w, 190.0, font);
        py += 24.0;

        if draw_ascii_button("[1] Meadow (Open Grass)", panel_x + 12.0, py, panel_w - 24.0, 22.0, mx, my, mouse_clicked, font)
            || is_key_pressed(KeyCode::Key1) {
            ow.select_first_biome(BiomeKind::Meadow);
        }
        py += 26.0;

        if draw_ascii_button("[2] Ancient Forest (Oaks)", panel_x + 12.0, py, panel_w - 24.0, 22.0, mx, my, mouse_clicked, font)
            || is_key_pressed(KeyCode::Key2) {
            ow.select_first_biome(BiomeKind::Forest);
        }
        py += 26.0;

        if draw_ascii_button("[3] Rugged Mountains (Peaks)", panel_x + 12.0, py, panel_w - 24.0, 22.0, mx, my, mouse_clicked, font)
            || is_key_pressed(KeyCode::Key3) {
            ow.select_first_biome(BiomeKind::Mountain);
        }
        py += 26.0;

        if draw_ascii_button("[4] Sunscorched Desert (Dunes)", panel_x + 12.0, py, panel_w - 24.0, 22.0, mx, my, mouse_clicked, font)
            || is_key_pressed(KeyCode::Key4) {
            ow.select_first_biome(BiomeKind::Desert);
        }
        py += 26.0;

        if draw_ascii_button("[5] Coastal Beach (Shores)", panel_x + 12.0, py, panel_w - 24.0, 22.0, mx, my, mouse_clicked, font)
            || is_key_pressed(KeyCode::Key5) {
            ow.select_first_biome(BiomeKind::Beach);
        }
        py += 26.0;

        if draw_ascii_button("[6] Crystal River Valley", panel_x + 12.0, py, panel_w - 24.0, 22.0, mx, my, mouse_clicked, font)
            || is_key_pressed(KeyCode::Key6) {
            ow.select_first_biome(BiomeKind::River);
        }
        py += 36.0;
    }

    // Load Game option if savegame exists and not in fast travel
    if !is_fast_travel && SaveGame::exists() {
        if draw_ascii_button(
            "[ 💾 LOAD SAVED EXPEDITION (Key L) ]",
            panel_x + 12.0,
            py,
            panel_w - 24.0,
            26.0,
            mx,
            my,
            mouse_clicked,
            font,
        ) || is_key_pressed(KeyCode::L) {
            result.load_saved_game = true;
        }
        py += 32.0;
    }

    // Regenerate button (only in new game mode)
    if !is_fast_travel {
        if draw_ascii_button("[R] Regenerate Continents", panel_x + 12.0, py, panel_w - 24.0, 22.0, mx, my, mouse_clicked, font)
            || is_key_pressed(KeyCode::R) {
            result.regenerate = true;
        }
        py += 26.0;

        if draw_ascii_button("[ESC] Return to Main Menu", panel_x + 12.0, py, panel_w - 24.0, 22.0, mx, my, mouse_clicked, font)
            || is_key_pressed(KeyCode::Escape) {
            result.return_to_main_menu = true;
        }
        py += 30.0;
    } else {
        if draw_ascii_button("[ESC / M] Cancel Fast Travel", panel_x + 12.0, py, panel_w - 24.0, 22.0, mx, my, mouse_clicked, font)
            || is_key_pressed(KeyCode::Escape) {
            result.return_to_main_menu = true;
        }
        py += 30.0;
    }

    // EMBARK BUTTON
    let embark_text = if can_embark {
        "=== [ ENTER / SPACE : EMBARK HERE ] ==="
    } else {
        "=== [ CANNOT TRAVEL: ADJACENT TILES ONLY ] ==="
    };

    let embark_clicked = if can_embark {
        draw_ascii_button(
            embark_text,
            panel_x + 12.0,
            py,
            panel_w - 24.0,
            34.0,
            mx,
            my,
            mouse_clicked,
            font,
        )
    } else {
        // Disabled button style
        draw_rectangle(panel_x + 12.0, py, panel_w - 24.0, 34.0, Color::new(0.12, 0.12, 0.15, 1.0));
        draw_rectangle_lines(panel_x + 12.0, py, panel_w - 24.0, 34.0, 1.0, Color::new(0.3, 0.3, 0.35, 1.0));
        let dim = measure_text(embark_text, font, 11, 1.0);
        draw_text_ex(
            embark_text,
            panel_x + 12.0 + (panel_w - 24.0 - dim.width) * 0.5,
            py + (34.0 + dim.height * 0.7) * 0.5,
            TextParams {
                font,
                font_size: 11,
                color: Color::new(0.5, 0.5, 0.5, 1.0),
                ..Default::default()
            },
        );
        false
    };

    if (embark_clicked || is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space)) && can_embark {
        result.embark = true;
    }

    // ==========================================
    // 4. BOTTOM INSTRUCTIONS
    // ==========================================
    let foot_y = screen_h - 26.0;
    let hint_str = if is_fast_travel {
        "[Click/Keys: Choose Adjacent Region] [Space/Enter: Embark to New Station] [ESC: Cancel Fast Travel]"
    } else {
        "[Click: Select Region] [1..6: Choose Biome] [R: New Seed] [Space/Enter: Embark] [ESC: Back to Menu]"
    };
    draw_ascii_line(hint_str, 24.0, foot_y, Color::new(0.60, 0.70, 0.85, 1.0), font, 12);

    result
}

fn draw_ascii_line(text: &str, x: f32, y: f32, color: Color, font: Option<&Font>, font_size: u16) {
    draw_text_ex(
        text,
        x,
        y,
        TextParams {
            font,
            font_size,
            color,
            ..Default::default()
        },
    );
}

fn draw_ascii_box(title: &str, x: f32, y: f32, w: f32, h: f32, font: Option<&Font>) {
    draw_rectangle(x, y, w, h, Color::new(0.06, 0.08, 0.12, 0.95));
    draw_rectangle_lines(x, y, w, h, 1.5, Color::new(0.25, 0.38, 0.52, 1.0));

    let title_badge = format!("[ {} ]", title);
    draw_text_ex(
        &title_badge,
        x + 12.0,
        y + 14.0,
        TextParams {
            font,
            font_size: 12,
            color: Color::new(0.40, 0.80, 1.0, 1.0),
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
    clicked: bool,
    font: Option<&Font>,
) -> bool {
    let hovered = mx >= x && mx <= (x + w) && my >= y && my <= (y + h);

    let bg = if hovered {
        Color::new(0.20, 0.30, 0.45, 1.0)
    } else {
        Color::new(0.10, 0.14, 0.20, 1.0)
    };

    let border = if hovered {
        Color::new(0.95, 0.85, 0.30, 1.0)
    } else {
        Color::new(0.30, 0.42, 0.56, 1.0)
    };

    draw_rectangle(x, y, w, h, bg);
    draw_rectangle_lines(x, y, w, h, 1.2, border);

    let font_size = 11;
    let dim = measure_text(text, font, font_size, 1.0);
    let tx = x + (w - dim.width) * 0.5;
    let ty = y + (h + dim.height * 0.70) * 0.5;

    let text_col = if hovered {
        Color::new(1.0, 1.0, 0.9, 1.0)
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

    hovered && clicked
}
