use crate::entity::radar::Radar;
use macroquad::prelude::*;

pub struct RadarViewResult {
    pub clicked_target_coord: Option<(i32, i32)>,
    pub toggle_visibility: bool,
}

pub fn render_radar_hud(
    radar: &Radar,
    font: Option<&Font>,
    player_x: i32,
    player_y: i32,
) -> RadarViewResult {
    let mut result = RadarViewResult {
        clicked_target_coord: None,
        toggle_visibility: false,
    };

    let time = get_time() as f32;
    let mouse_pos = mouse_position();
    let (mx, my) = (mouse_pos.0, mouse_pos.1);
    let mouse_clicked = is_mouse_button_pressed(MouseButton::Left);

    let top_y = 44.0;
    let hud_x = 12.0;

    // =========================================================================
    // 1. RADAR STATUS BADGE & TOGGLE BUTTON
    // =========================================================================
    let badge_w = 260.0;
    let badge_h = 26.0;

    let is_active = radar.is_active;
    let pulse = ((time * 5.0).sin() * 0.25) + 0.75;

    let (badge_text, badge_col, bg_col) = if is_active {
        let text = format!(
            "📡 RADAR: ACTIVE ({} items, {} fauna)",
            radar.detected_items.len(),
            radar.detected_creatures.len()
        );
        (
            text,
            Color::new(0.35 * pulse, 0.95 * pulse, 0.65 * pulse, 1.0),
            Color::new(0.08, 0.16, 0.12, 0.92),
        )
    } else {
        (
            "📡 RADAR: SCANNING (No targets)".to_string(),
            Color::new(0.55, 0.65, 0.75, 0.85),
            Color::new(0.06, 0.08, 0.12, 0.85),
        )
    };

    let is_badge_hov = mx >= hud_x && mx <= (hud_x + badge_w) && my >= top_y && my <= (top_y + badge_h);
    if is_badge_hov && mouse_clicked {
        result.toggle_visibility = true;
    }

    draw_rectangle(hud_x, top_y, badge_w, badge_h, bg_col);
    draw_rectangle_lines(
        hud_x,
        top_y,
        badge_w,
        badge_h,
        if is_active { 1.5 } else { 1.0 },
        if is_active { Color::new(0.30, 0.85, 0.55, 0.9) } else { Color::new(0.25, 0.35, 0.45, 0.8) },
    );

    draw_text_ex(
        &badge_text,
        hud_x + 8.0,
        top_y + 17.0,
        TextParams {
            font,
            font_size: 11,
            color: badge_col,
            ..Default::default()
        },
    );

    // Visibility toggle indicator [TAB]
    let tab_hint = if radar.is_list_visible { "[TAB: HIDE]" } else { "[TAB: SHOW]" };
    let dim_tab = measure_text(tab_hint, font, 9, 1.0);
    draw_text_ex(
        tab_hint,
        hud_x + badge_w - dim_tab.width - 6.0,
        top_y + 16.0,
        TextParams {
            font,
            font_size: 9,
            color: Color::new(0.70, 0.80, 0.90, 0.8),
            ..Default::default()
        },
    );

    // If list is hidden, don't draw the dossier window
    if !radar.is_list_visible {
        return result;
    }

    // =========================================================================
    // 2. DETAILED RADAR DOSSIER WINDOW
    // =========================================================================
    let win_w = 330.0;
    let win_h = 490.0f32.min(screen_height() - top_y - badge_h - 45.0);
    let win_y = top_y + badge_h + 6.0;

    // Window background & borders
    draw_rectangle(hud_x, win_y, win_w, win_h, Color::new(0.04, 0.06, 0.09, 0.92));
    draw_rectangle_lines(hud_x, win_y, win_w, win_h, 1.5, Color::new(0.28, 0.45, 0.65, 0.95));

    let mut py = win_y + 16.0;

    // Title banner
    let title = "╔═[ 📡 RADAR SCOPE & VISION SCANNER ]═╗";
    draw_text_ex(
        title,
        hud_x + 10.0,
        py,
        TextParams {
            font,
            font_size: 11,
            color: Color::new(0.40, 0.85, 1.0, 1.0),
            ..Default::default()
        },
    );
    py += 14.0;

    // Circular mini radar scope in ASCII
    let radar_scope_cx = hud_x + win_w - 45.0;
    let radar_scope_cy = win_y + 45.0;
    let radar_radius = 28.0;

    draw_circle(radar_scope_cx, radar_scope_cy, radar_radius, Color::new(0.02, 0.08, 0.06, 0.9));
    draw_circle_lines(radar_scope_cx, radar_scope_cy, radar_radius, 1.2, Color::new(0.20, 0.70, 0.40, 0.8));
    draw_circle_lines(radar_scope_cx, radar_scope_cy, radar_radius * 0.5, 1.0, Color::new(0.15, 0.50, 0.30, 0.5));
    // Player center dot
    draw_circle(radar_scope_cx, radar_scope_cy, 2.0, Color::new(1.0, 0.9, 0.3, 1.0));

    // Rotating sweep line
    let sweep_x = radar_scope_cx + radar.sweep_angle.cos() * radar_radius;
    let sweep_y = radar_scope_cy + radar.sweep_angle.sin() * radar_radius;
    draw_line(radar_scope_cx, radar_scope_cy, sweep_x, sweep_y, 1.2, Color::new(0.3, 0.95, 0.5, 0.6));

    // Blips on the mini-radar
    for item in &radar.detected_items {
        let dx = (item.x - player_x) as f32;
        let dy = (item.y - player_y) as f32;
        let blip_x = radar_scope_cx + (dx / radar.vision_radius) * radar_radius;
        let blip_y = radar_scope_cy + (dy / radar.vision_radius) * radar_radius;
        draw_circle(blip_x, blip_y, 1.5, item.item.color());
    }
    for c in &radar.detected_creatures {
        let dx = (c.x - player_x) as f32;
        let dy = (c.y - player_y) as f32;
        let blip_x = radar_scope_cx + (dx / radar.vision_radius) * radar_radius;
        let blip_y = radar_scope_cy + (dy / radar.vision_radius) * radar_radius;
        draw_circle(blip_x, blip_y, 2.0, c.kind.color());
    }

    // --- SECTION A: REGION & ENVIRONMENT ---
    let reg = &radar.region_info;
    draw_text_ex(
        "+-- [ 🌍 CURRENT REGION ] ----------------+",
        hud_x + 10.0,
        py,
        TextParams { font, font_size: 10, color: Color::new(0.35, 0.70, 0.90, 1.0), ..Default::default() },
    );
    py += 14.0;

    let r_line1 = format!("• Biome : {}", reg.biome_name);
    let r_line2 = format!("• Layer : Z {:+03} ({})", reg.elevation_z, reg.tile_name);
    let r_line3 = format!("• Time  : {} • {}", reg.time_str, reg.atmosphere);

    draw_text_ex(&r_line1, hud_x + 14.0, py, TextParams { font, font_size: 10, color: Color::new(0.85, 0.90, 0.95, 1.0), ..Default::default() });
    py += 13.0;
    draw_text_ex(&r_line2, hud_x + 14.0, py, TextParams { font, font_size: 10, color: Color::new(0.70, 0.85, 0.95, 1.0), ..Default::default() });
    py += 13.0;
    draw_text_ex(&r_line3, hud_x + 14.0, py, TextParams { font, font_size: 10, color: Color::new(0.55, 0.75, 0.85, 1.0), ..Default::default() });
    py += 20.0;

    // --- SECTION B: DETECTED GROUND ITEMS ---
    let item_header = format!("+-- [ 🌾 DETECTED ITEMS ({}) ] -----------+", radar.detected_items.len());
    draw_text_ex(
        &item_header,
        hud_x + 10.0,
        py,
        TextParams { font, font_size: 10, color: Color::new(0.40, 0.85, 0.55, 1.0), ..Default::default() },
    );
    py += 15.0;

    if radar.detected_items.is_empty() {
        draw_text_ex(
            "  (No resources visible in 16m radius)",
            hud_x + 14.0,
            py,
            TextParams { font, font_size: 10, color: Color::new(0.45, 0.50, 0.55, 0.9), ..Default::default() },
        );
        py += 16.0;
    } else {
        // Show up to 7 closest items with clickable travel
        for item in radar.detected_items.iter().take(7) {
            let item_row_y = py;
            let item_row_h = 18.0;

            let is_row_hov = mx >= (hud_x + 10.0) && mx <= (hud_x + win_w - 10.0) && my >= item_row_y - 12.0 && my <= item_row_y + 4.0;
            if is_row_hov {
                draw_rectangle(hud_x + 10.0, item_row_y - 11.0, win_w - 20.0, item_row_h, Color::new(0.15, 0.25, 0.35, 0.8));
                if mouse_clicked {
                    result.clicked_target_coord = Some((item.x, item.y));
                }
            }

            let item_line = format!(
                "• [{}] {:<10} x{} (Z{:+} | {:.1}m {})",
                item.item.glyph(),
                item.item.name(),
                item.count,
                item.z,
                item.distance,
                item.direction
            );

            draw_text_ex(
                &item_line,
                hud_x + 14.0,
                item_row_y,
                TextParams {
                    font,
                    font_size: 10,
                    color: if is_row_hov { Color::new(1.0, 0.95, 0.6, 1.0) } else { item.item.color() },
                    ..Default::default()
                },
            );
            py += 18.0;
        }
    }

    py += 8.0;

    // --- SECTION C: DETECTED CREATURES / FAUNA ---
    let fauna_header = format!("+-- [ 🦌 WILD FAUNA ({}) ] ---------------+", radar.detected_creatures.len());
    draw_text_ex(
        &fauna_header,
        hud_x + 10.0,
        py,
        TextParams { font, font_size: 10, color: Color::new(0.95, 0.75, 0.35, 1.0), ..Default::default() },
    );
    py += 15.0;

    if radar.detected_creatures.is_empty() {
        draw_text_ex(
            "  (No animals visible in 16m radius)",
            hud_x + 14.0,
            py,
            TextParams { font, font_size: 10, color: Color::new(0.45, 0.50, 0.55, 0.9), ..Default::default() },
        );
    } else {
        for c in radar.detected_creatures.iter().take(4) {
            let c_row_y = py;
            let c_row_h = 18.0;

            let is_row_hov = mx >= (hud_x + 10.0) && mx <= (hud_x + win_w - 10.0) && my >= c_row_y - 12.0 && my <= c_row_y + 4.0;
            if is_row_hov {
                draw_rectangle(hud_x + 10.0, c_row_y - 11.0, win_w - 20.0, c_row_h, Color::new(0.20, 0.20, 0.30, 0.8));
                if mouse_clicked {
                    result.clicked_target_coord = Some((c.x, c.y));
                }
            }

            let c_line = format!(
                "• [{}] {:<5} [{:.0}HP {}] (Z{:+} | {:.1}m {})",
                c.kind.glyph(),
                c.kind.name(),
                c.health,
                c.status,
                c.z,
                c.distance,
                c.direction
            );

            draw_text_ex(
                &c_line,
                hud_x + 14.0,
                c_row_y,
                TextParams {
                    font,
                    font_size: 10,
                    color: if is_row_hov { Color::new(1.0, 1.0, 0.8, 1.0) } else { c.kind.color() },
                    ..Default::default()
                },
            );
            py += 18.0;
        }
    }

    // Footer tip inside radar box
    let foot_tip = "[Click entry: Navigate to Target] [TAB: Hide]";
    draw_text_ex(
        foot_tip,
        hud_x + 14.0,
        win_y + win_h - 10.0,
        TextParams {
            font,
            font_size: 9,
            color: Color::new(0.45, 0.60, 0.75, 0.85),
            ..Default::default()
        },
    );

    result
}
