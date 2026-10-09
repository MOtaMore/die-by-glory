use crate::camera::Camera;
use crate::entity::Player;
use crate::world::tile::{Tile, TileKind};
use crate::world::{World, MAP_HEIGHT, MAP_WIDTH};
use macroquad::prelude::*;

pub struct AsciiRenderer {
    pub font: Option<Font>,
}

impl AsciiRenderer {
    pub fn new(font: Option<Font>) -> Self {
        Self { font }
    }

    pub fn render(
        &self,
        world: &World,
        player: &Player,
        camera: &Camera,
        hover_coord: Option<(i32, i32)>,
        destination_marker: Option<(i32, i32)>,
    ) {
        let (cw, ch) = camera.effective_cell_size();
        let font_size = (ch * 0.90) as u16;

        // Limitar el renderizado a las baldosas visibles en la ventana gráfica
        let min_world = camera.screen_to_world(camera.viewport_x - cw, camera.viewport_y - ch);
        let max_world = camera.screen_to_world(
            camera.viewport_x + camera.viewport_w + cw * 2.0,
            camera.viewport_y + camera.viewport_h + ch * 2.0,
        );

        let min_x = min_world.0.max(0).min(MAP_WIDTH as i32);
        let max_x = (max_world.0 + 1).max(0).min(MAP_WIDTH as i32);
        let min_y = min_world.1.max(0).min(MAP_HEIGHT as i32);
        let max_y = (max_world.1 + 1).max(0).min(MAP_HEIGHT as i32);

        // Clip viewport rect
        // Dibujamos fondo de la zona de juego
        draw_rectangle(
            camera.viewport_x,
            camera.viewport_y,
            camera.viewport_w,
            camera.viewport_h,
            Color::new(0.04, 0.05, 0.07, 1.0),
        );

        let time = get_time() as f32;
        let view_z = camera.view_z;

        // 1. RENDERIZADO DE BALDOSAS DEL MUNDO CON CAPAS TRANSLÚCIDAS
        for y in min_y..max_y {
            for x in min_x..max_x {
                let (sx, sy) = camera.world_to_screen(x as f32, y as f32);

                // Comprobar si la casilla en la capa de cámara tiene contenido
                let tile_at_view = world.get_tile(x, y, view_z);

                if let Some(tile) = tile_at_view {
                    if tile.kind != TileKind::Air {
                        // Capa actual: Renderizado a OPACIDAD TOTAL (100%)
                        self.draw_cell(x, y, tile, sx, sy, cw, ch, font_size, 1.0, 0, time);
                    } else {
                        // Es aire: buscar hacia abajo y renderizar con VISIÓN TRANSLÚCIDA
                        if let Some((found_z, lower_tile)) =
                            world.find_visible_surface_down(x, y, view_z - 1)
                        {
                            let depth = (view_z - found_z).max(1);
                            // Cálculo de translucidez / decaimiento por profundidad
                            let alpha = (0.86_f32).powi(depth).max(0.12);
                            self.draw_cell(
                                x, y, lower_tile, sx, sy, cw, ch, font_size, alpha, depth, time,
                            );
                        } else {
                            // Abismo profundo sin fondo visible
                            draw_rectangle(
                                sx,
                                sy,
                                cw,
                                ch,
                                Color::new(0.02, 0.02, 0.04, 1.0),
                            );
                        }
                    }
                }
            }
        }

        // 2. RENDERIZADO DEL CAMINO DE PATHFINDING
        if !player.path.is_empty() {
            let path_len = player.path.len();
            for (step_idx, &(px, py)) in player.path.iter().enumerate() {
                let (psx, psy) = camera.world_to_screen(px as f32, py as f32);
                let center_x = psx + cw * 0.5;
                let center_y = psy + ch * 0.5;

                let pulse = (time * 6.0 + step_idx as f32 * 0.4).sin() * 0.2 + 0.8;
                let path_color = Color::new(0.30, 0.85, 1.0, 0.65 * pulse);

                // Dibujar punto luminoso en el centro de la celda
                draw_circle(center_x, center_y, (cw * 0.22).max(2.5), path_color);

                // Conectar con el siguiente punto con una línea sutil
                if step_idx + 1 < path_len {
                    let next = player.path[step_idx + 1];
                    let (nsx, nsy) = camera.world_to_screen(next.0 as f32, next.1 as f32);
                    draw_line(
                        center_x,
                        center_y,
                        nsx + cw * 0.5,
                        nsy + ch * 0.5,
                        1.5,
                        Color::new(0.30, 0.85, 1.0, 0.35),
                    );
                }
            }
        }

        // 3. MARCADOR DE DESTINO SELECCIONADO (CLIC)
        if let Some((dx, dy)) = destination_marker {
            let (dsx, dsy) = camera.world_to_screen(dx as f32, dy as f32);
            let ring_radius = (cw * 0.6) + ((time * 4.0).sin() * 2.0);
            draw_circle_lines(
                dsx + cw * 0.5,
                dsy + ch * 0.5,
                ring_radius.max(3.0),
                2.0,
                Color::new(1.0, 0.85, 0.25, 0.85),
            );
        }

        // 3b. RENDERIZADO DE ITEMS EN EL SUELO
        for item in &world.ground_items {
            if item.x >= min_x && item.x < max_x && item.y >= min_y && item.y < max_y {
                let diff_z = view_z - item.z;
                if diff_z == 0 {
                    let (isx, isy) = camera.world_to_screen(item.x as f32, item.y as f32);
                    self.draw_ground_item(item, isx, isy, cw, ch, font_size, 1.0, time);
                } else if diff_z > 0 && diff_z <= 8 {
                    let alpha = (0.84_f32).powi(diff_z).max(0.20);
                    let (isx, isy) = camera.world_to_screen(item.x as f32, item.y as f32);
                    self.draw_ground_item(item, isx, isy, cw, ch, font_size, alpha, time);
                }
            }
        }

        // 3c. RENDERIZADO DE CRIATURAS
        for creature in &world.creatures {
            if creature.x >= min_x && creature.x < max_x && creature.y >= min_y && creature.y < max_y {
                let diff_z = view_z - creature.z;
                if diff_z == 0 {
                    let (csx, csy) = camera.world_to_screen(creature.x as f32, creature.y as f32);
                    self.draw_creature(creature, csx, csy, cw, ch, font_size, 1.0, time);
                } else if diff_z > 0 && diff_z <= 8 {
                    let alpha = (0.84_f32).powi(diff_z).max(0.20);
                    let (csx, csy) = camera.world_to_screen(creature.x as f32, creature.y as f32);
                    self.draw_creature(creature, csx, csy, cw, ch, font_size, alpha, time);
                }
            }
        }

        // 4. RENDERIZADO DEL PERSONAJE PRINCIPAL (@)
        self.draw_player(player, camera, cw, ch, font_size, time);

        // 5. RESALTADO DE RATÓN (HOVER RETICLE)
        if let Some((hx, hy)) = hover_coord {
            if hx >= 0 && hx < MAP_WIDTH as i32 && hy >= 0 && hy < MAP_HEIGHT as i32 {
                let (hsx, hsy) = camera.world_to_screen(hx as f32, hy as f32);
                draw_rectangle_lines(
                    hsx,
                    hsy,
                    cw,
                    ch,
                    2.0,
                    Color::new(1.0, 1.0, 1.0, 0.65),
                );
            }
        }
    }

    fn draw_cell(
        &self,
        _x: i32,
        _y: i32,
        tile: &Tile,
        sx: f32,
        sy: f32,
        cw: f32,
        ch: f32,
        font_size: u16,
        alpha: f32,
        depth: i32,
        time: f32,
    ) {
        // Modulación dinámica para el río
        let mut fg = tile.fg_color;
        let mut bg = tile.bg_color;

        if tile.kind == TileKind::WaterShallow || tile.kind == TileKind::WaterDeep {
            let ripple = ((time * 2.5 + sx * 0.05 + sy * 0.05).sin() * 0.08) + 0.92;
            fg.r = (fg.r * ripple).min(1.0);
            fg.g = (fg.g * ripple).min(1.0);
            fg.b = (fg.b * ripple).min(1.0);
        }

        // Si es una capa inferior translucida, atenuar y añadir niebla de profundidad
        if depth > 0 {
            let depth_factor = (depth as f32 * 0.035).min(0.60);
            // Tinte atmosférico azulado/oscuro hacia las profundidades
            bg.r = (bg.r * (1.0 - depth_factor)).max(0.02);
            bg.g = (bg.g * (1.0 - depth_factor)).max(0.03);
            bg.b = ((bg.b * (1.0 - depth_factor)) + depth_factor * 0.25).min(1.0);
            fg.r *= 0.85;
            fg.g *= 0.85;
            fg.b *= 0.90;
        }

        bg.a *= alpha;
        fg.a *= alpha;

        // Dibujar fondo de la casilla
        draw_rectangle(sx, sy, cw, ch, bg);

        // Dibujar glifo de la casilla
        if tile.glyph != ' ' {
            let char_str = tile.glyph.to_string();
            let text_params = TextParams {
                font: self.font.as_ref(),
                font_size,
                font_scale: 1.0,
                color: fg,
                ..Default::default()
            };

            let text_dim = measure_text(&char_str, self.font.as_ref(), font_size, 1.0);
            let tx = sx + (cw - text_dim.width) * 0.5;
            let ty = sy + (ch + text_dim.height * 0.70) * 0.5;

            draw_text_ex(&char_str, tx, ty, text_params);
        }
    }

    fn draw_player(
        &self,
        player: &Player,
        camera: &Camera,
        cw: f32,
        ch: f32,
        font_size: u16,
        time: f32,
    ) {
        let (psx, psy) = camera.world_to_screen(player.x as f32, player.y as f32);
        let center_x = psx + cw * 0.5;
        let center_y = psy + ch * 0.5;

        let view_z = camera.view_z;
        let p_z = player.z;

        if p_z == view_z {
            // Torch glow if torch equipped in Main or Off Hand
            let has_torch = player.inventory.equipment.main_hand == Some(crate::entity::item::ItemType::Torch)
                || player.inventory.equipment.off_hand == Some(crate::entity::item::ItemType::Torch);
            if has_torch {
                let flicker = ((time * 7.5).sin() * 0.12) + 0.88;
                draw_circle(
                    center_x,
                    center_y,
                    cw * 4.2 * flicker,
                    Color::new(1.0, 0.65, 0.20, 0.20),
                );
            }

            // El personaje está exactamente en la capa actual de la cámara
            // Efecto de halo pulsante
            let pulse = ((time * 4.0).sin() * 0.25) + 0.75;
            draw_circle(
                center_x,
                center_y,
                (cw * 0.65).max(5.0),
                Color::new(1.0, 0.85, 0.20, 0.28 * pulse),
            );

            // Carácter '@' en dorado brillante
            let player_fg = Color::new(1.0, 0.95, 0.35, 1.0);
            let text_params = TextParams {
                font: self.font.as_ref(),
                font_size: (font_size as f32 * 1.15) as u16,
                font_scale: 1.0,
                color: player_fg,
                ..Default::default()
            };

            let text_dim = measure_text("@", self.font.as_ref(), text_params.font_size, 1.0);
            let tx = psx + (cw - text_dim.width) * 0.5;
            let ty = psy + (ch + text_dim.height * 0.70) * 0.5;

            draw_text_ex("@", tx, ty, text_params);
        } else if p_z < view_z {
            // El personaje está en una capa inferior: se ve a través del aire translúcido
            let depth = (view_z - p_z).max(1);
            let alpha = (0.86_f32).powi(depth).max(0.18);

            // Círculo indicador translúcido para no perder al personaje
            draw_circle_lines(
                center_x,
                center_y,
                cw * 0.55,
                1.5,
                Color::new(1.0, 0.85, 0.20, alpha),
            );

            let player_fg = Color::new(1.0, 0.90, 0.30, alpha);
            let text_params = TextParams {
                font: self.font.as_ref(),
                font_size,
                font_scale: 1.0,
                color: player_fg,
                ..Default::default()
            };

            let text_dim = measure_text("@", self.font.as_ref(), font_size, 1.0);
            let tx = psx + (cw - text_dim.width) * 0.5;
            let ty = psy + (ch + text_dim.height * 0.70) * 0.5;

            draw_text_ex("@", tx, ty, text_params);
        } else {
            // El personaje está en una capa SUPERIOR a la cámara (ej. cámara bajo tierra)
            // Dibujar un marcador fantasma indicando que el jugador está arriba
            let pulse = ((time * 3.0).sin() * 0.2) + 0.6;
            draw_circle_lines(
                center_x,
                center_y,
                cw * 0.6,
                1.5,
                Color::new(0.4, 0.8, 1.0, 0.5 * pulse),
            );

            let text_params = TextParams {
                font: self.font.as_ref(),
                font_size: (font_size as f32 * 0.85) as u16,
                font_scale: 1.0,
                color: Color::new(0.5, 0.9, 1.0, 0.75),
                ..Default::default()
            };
            let text_dim = measure_text("▲", self.font.as_ref(), text_params.font_size, 1.0);
            draw_text_ex(
                "▲",
                psx + (cw - text_dim.width) * 0.5,
                psy + (ch + text_dim.height * 0.70) * 0.5,
                text_params,
            );
        }
    }

    fn draw_ground_item(
        &self,
        item: &crate::entity::GroundItem,
        sx: f32,
        sy: f32,
        cw: f32,
        ch: f32,
        font_size: u16,
        alpha: f32,
        time: f32,
    ) {
        let mut col = item.item.color();
        let pulse = ((time * 4.0 + (item.id as f32)).sin() * 0.15) + 0.85;
        col.a = (col.a * alpha * pulse).clamp(0.0, 1.0);

        let glyph_str = item.item.glyph().to_string();
        let text_dim = measure_text(&glyph_str, self.font.as_ref(), font_size, 1.0);
        let tx = sx + (cw - text_dim.width) * 0.5;
        let ty = sy + (ch + text_dim.height * 0.70) * 0.5;

        // Subtle glowing dot under item
        draw_circle(
            sx + cw * 0.5,
            sy + ch * 0.5,
            (cw * 0.22).max(2.0),
            Color::new(col.r, col.g, col.b, 0.25 * alpha),
        );

        draw_text_ex(
            &glyph_str,
            tx,
            ty,
            TextParams {
                font: self.font.as_ref(),
                font_size,
                color: col,
                ..Default::default()
            },
        );
    }

    fn draw_creature(
        &self,
        creature: &crate::entity::Creature,
        sx: f32,
        sy: f32,
        cw: f32,
        ch: f32,
        font_size: u16,
        alpha: f32,
        time: f32,
    ) {
        if creature.health <= 0.0 {
            return;
        }

        let mut col = if creature.is_alert && creature.kind == crate::entity::creature::CreatureType::Boar {
            let pulse = ((time * 8.0).sin() * 0.2) + 0.8;
            Color::new(1.0, 0.28 * pulse, 0.28 * pulse, 1.0)
        } else if creature.is_alert {
            Color::new(1.0, 0.88, 0.35, 1.0)
        } else {
            creature.kind.color()
        };
        col.a = (col.a * alpha).clamp(0.0, 1.0);

        let glyph_str = creature.kind.glyph().to_string();
        let text_dim = measure_text(&glyph_str, self.font.as_ref(), font_size, 1.0);
        let tx = sx + (cw - text_dim.width) * 0.5;
        let ty = sy + (ch + text_dim.height * 0.70) * 0.5;

        // Draw small mini health bar if creature took damage
        if creature.health < creature.max_health {
            let bar_w = (cw * 0.8).max(6.0);
            let bar_h = 2.0;
            let bx = sx + (cw - bar_w) * 0.5;
            let by = sy - 3.0;
            draw_rectangle(bx, by, bar_w, bar_h, Color::new(0.3, 0.1, 0.1, 0.85 * alpha));
            let hp_pct = (creature.health / creature.max_health).clamp(0.0, 1.0);
            draw_rectangle(bx, by, bar_w * hp_pct, bar_h, Color::new(0.95, 0.25, 0.25, 0.95 * alpha));
        }

        draw_text_ex(
            &glyph_str,
            tx,
            ty,
            TextParams {
                font: self.font.as_ref(),
                font_size,
                color: col,
                ..Default::default()
            },
        );
    }
}

