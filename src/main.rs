mod camera;
mod entity;
mod pathfinding;
mod render;
mod save;
mod world;

use camera::Camera;
use entity::{BuildMaterial, CreatureType, ItemType, Player, PlayerEvent, Radar};
use pathfinding::find_path;
use render::{
    render_inventory_screen, render_main_menu, render_overworld_screen, render_radar_hud,
    render_ui, AsciiRenderer, InventoryUiState, MenuAction, MenuState, UiState,
};
use save::SaveGame;
use world::{Overworld, TileKind, World, MAP_HEIGHT, MAP_WIDTH, OW_HEIGHT, OW_WIDTH, Z_SURFACE};

use macroquad::prelude::*;

const FONT_DATA: &[u8] = include_bytes!("../assets/FiraMono-Regular.ttf");
const COVER_DATA: &[u8] = include_bytes!("../assets/cover.jpg");

fn window_conf() -> Conf {
    Conf {
        window_title: "Die by Glory - ASCII Survival (Rust)".to_string(),
        window_width: 1280,
        window_height: 768,
        high_dpi: false,
        window_resizable: true,
        ..Default::default()
    }
}

enum AppState {
    MainMenu {
        menu_state: MenuState,
    },
    WorldSelect {
        overworld: Overworld,
        is_fast_travel: bool,
    },
    Playing {
        world: World,
        player: Player,
        camera: Camera,
        overworld: Overworld,
        ui_state: UiState,
        radar: Radar,
        inventory_ui: InventoryUiState,
        destination_marker: Option<(i32, i32)>,
        last_drag_pos: Option<(f32, f32)>,
        is_build_mode: bool,
        is_demolish_mode: bool,
        build_material: BuildMaterial,
        active_hotbar_slot: usize,
    },
}

fn load_game_state(save: SaveGame) -> AppState {
    let mut ow = Overworld::new(save.overworld_seed);
    let safe_ow_x = save.ow_current_x.min(OW_WIDTH - 1);
    let safe_ow_y = save.ow_current_y.min(OW_HEIGHT - 1);

    // Auto-recuperación: si la partida guardada tenía coordenadas en océano, reubicar a tierra continental válida
    let (real_ow_x, real_ow_y) = ow.find_nearest_land(safe_ow_x, safe_ow_y);
    let is_relocated = real_ow_x != safe_ow_x || real_ow_y != safe_ow_y;
    ow.cursor_x = real_ow_x;
    ow.cursor_y = real_ow_y;
    ow.set_station(real_ow_x, real_ow_y);
    ow.visited_stations = save.discovered_regions.clone();
    if !ow.visited_stations.contains(&(real_ow_x, real_ow_y)) {
        ow.visited_stations.push((real_ow_x, real_ow_y));
    }

    let biome = if is_relocated {
        ow.get_tile(real_ow_x, real_ow_y).map(|t| t.biome).unwrap_or(save.current_biome)
    } else {
        save.current_biome
    };

    let mut world = World::new(save.overworld_seed, biome);
    world.time_of_day = save.time_of_day;

    let (px, py, pz) = if is_relocated {
        world.spawn_pos
    } else {
        let px = save.player_x.clamp(3, MAP_WIDTH as i32 - 4);
        let py = save.player_y.clamp(3, MAP_HEIGHT as i32 - 4);
        let pz = world.get_surface_z(px, py);
        (px, py, pz)
    };

    let mut player = Player::new((px, py, pz));
    player.health = save.health;
    player.hunger = save.hunger;
    player.thirst = save.thirst;
    player.energy = save.energy;
    if let Some(saved_inv) = save.inventory {
        player.inventory = saved_inv;
    }

    let camera = Camera::new(player.x, player.y, player.z);
    let mut ui_state = UiState::new();
    if is_relocated {
        ui_state.add_log(format!(
            "🌊 Auto-relocated to nearest mainland station at ({}, {}) in {}.",
            real_ow_x, real_ow_y, biome.info().name
        ));
        let updated_save = SaveGame::create_from_state(
            &world,
            &player,
            ow.seed,
            real_ow_x,
            real_ow_y,
            ow.visited_stations.clone(),
        );
        let _ = updated_save.save();
    } else {
        ui_state.add_log(format!(
            "Resumed saved expedition at Continent (X:{}, Y:{}) in {}.",
            save.ow_current_x, save.ow_current_y, save.current_biome.info().name
        ));
    }

    AppState::Playing {
        world,
        player,
        camera,
        overworld: ow,
        ui_state,
        radar: Radar::new(),
        inventory_ui: InventoryUiState::new(),
        destination_marker: None,
        last_drag_pos: None,
        is_build_mode: false,
        is_demolish_mode: false,
        build_material: BuildMaterial::WoodPlanks,
        active_hotbar_slot: 0,
    }
}

fn handle_place_block(
    world: &mut World,
    player: &mut Player,
    ui_state: &mut UiState,
    material: BuildMaterial,
    tx: i32,
    ty: i32,
    destination_marker: &mut Option<(i32, i32)>,
) {
    let item = material.item_type();
    if player.inventory.count_item(item) > 0 {
        let dist = ((player.x - tx).pow(2) as f32 + (player.y - ty).pow(2) as f32).sqrt();
        if dist <= 3.5 {
            match world.place_block_at(tx, ty, item) {
                Ok(_) => {
                    player.inventory.remove_item(item, 1);
                    player.energy = (player.energy - 2.5).max(0.0);
                    let left = player.inventory.count_item(item);
                    ui_state.add_log(format!(
                        "🔨 Colocado {} en ({}, {})! (Restantes en mochila: {})",
                        material.name_es(),
                        tx,
                        ty,
                        left
                    ));
                }
                Err(e) => {
                    ui_state.add_log(format!("No se puede construir aquí: {}", e));
                }
            }
        } else if let Some(path) = find_path(world, (player.x, player.y), (tx, ty)) {
            *destination_marker = Some((tx, ty));
            player.set_path(path);
            ui_state.add_log(format!(
                "Moviéndose hacia ({}, {}) para colocar {}...",
                tx,
                ty,
                material.name_es()
            ));
        } else {
            ui_state.add_log(format!("Destino ({}, {}) fuera de alcance o bloqueado.", tx, ty));
        }
    } else {
        ui_state.add_log(format!(
            "⚠️ ¡No tienes {} en la mochila! Craftea más [I] o reúne materiales.",
            material.name_es()
        ));
    }
}

fn handle_harvest_demolish(
    world: &mut World,
    player: &mut Player,
    ui_state: &mut UiState,
    tx: i32,
    ty: i32,
    destination_marker: &mut Option<(i32, i32)>,
) {
    let dist = ((player.x - tx).pow(2) as f32 + (player.y - ty).pow(2) as f32).sqrt();
    if dist <= 3.5 {
        let tz = world.get_surface_z(tx, ty);
        let t_kind = world.get_tile(tx, ty, tz).map(|t| t.kind).unwrap_or(TileKind::Air);

        match t_kind {
            TileKind::TreeTrunk | TileKind::PalmTree => {
                if player.has_hatchet() {
                    match world.chop_tree_at(tx, ty) {
                        Ok((logs, sticks)) => {
                            let rem_l = player.inventory.add_item(ItemType::WoodLog, logs);
                            let rem_s = player.inventory.add_item(ItemType::Stick, sticks);
                            if rem_l > 0 {
                                world.spawn_ground_item(ItemType::WoodLog, rem_l, tx, ty, tz);
                            }
                            if rem_s > 0 {
                                world.spawn_ground_item(ItemType::Stick, rem_s, tx, ty, tz);
                            }
                            player.energy = (player.energy - 6.0).max(0.0);
                            ui_state.add_log(format!(
                                "🪓 Árbol talado! Cosechados {}x Troncos y {}x Palos.",
                                logs, sticks
                            ));
                        }
                        Err(e) => ui_state.add_log(e.to_string()),
                    }
                } else {
                    ui_state.add_log("⚠️ ¡Necesitas un Hacha de Piedra para talar árboles! Craftea una en el inventario [I].".to_string());
                }
            }
            TileKind::MountainRock
            | TileKind::MountainPeak
            | TileKind::Stone
            | TileKind::Granite
            | TileKind::IronOre
            | TileKind::CoalOre => {
                if player.has_pickaxe() {
                    match world.pickaxe_rock_at(tx, ty) {
                        Ok((item, count)) => {
                            let rem = player.inventory.add_item(item, count);
                            if rem > 0 {
                                world.spawn_ground_item(item, rem, tx, ty, tz);
                            }
                            player.energy = (player.energy - 7.0).max(0.0);
                            ui_state.add_log(format!(
                                "⛏️ Piedra picada con Pico de Piedra! Cosechados {}x {}.",
                                count,
                                item.name()
                            ));
                        }
                        Err(e) => ui_state.add_log(e.to_string()),
                    }
                } else {
                    ui_state.add_log("⚠️ ¡Necesitas un Pico de Piedra para picar roca! Craftea uno en el inventario [I].".to_string());
                }
            }
            TileKind::Grass
            | TileKind::TallGrass
            | TileKind::Wildflower
            | TileKind::Soil
            | TileKind::Subsoil
            | TileKind::Sand
            | TileKind::DesertSand
            | TileKind::DesertDune
            | TileKind::BeachSand => {
                if player.has_shovel() {
                    match world.shovel_dirt_at(tx, ty) {
                        Ok(item) => {
                            let rem = player.inventory.add_item(item, 1);
                            if rem > 0 {
                                world.spawn_ground_item(item, rem, tx, ty, tz);
                            }
                            player.energy = (player.energy - 4.5).max(0.0);
                            ui_state.add_log("🚜 Tierra excavada con Pala de Piedra! Cosechado 1x Bloque de Tierra (Elevación reducida).".to_string());
                        }
                        Err(e) => ui_state.add_log(e.to_string()),
                    }
                } else {
                    ui_state.add_log("⚠️ ¡Necesitas una Pala de Piedra para cavar tierra! Craftea una en el inventario [I].".to_string());
                }
            }
            TileKind::WoodPlanks => {
                match world.demolish_at(tx, ty) {
                    Ok(item) => {
                        let rem = player.inventory.add_item(item, 1);
                        if rem > 0 {
                            world.spawn_ground_item(item, rem, tx, ty, tz);
                        }
                        player.energy = (player.energy - 3.0).max(0.0);
                        ui_state.add_log("💥 ¡Tablas de Madera demolidas! Recuperado 1x Tablas de Madera en mochila.".to_string());
                    }
                    Err(e) => ui_state.add_log(e.to_string()),
                }
            }
            TileKind::Cobblestone => {
                match world.demolish_at(tx, ty) {
                    Ok(item) => {
                        let rem = player.inventory.add_item(item, 1);
                        if rem > 0 {
                            world.spawn_ground_item(item, rem, tx, ty, tz);
                        }
                        player.energy = (player.energy - 4.0).max(0.0);
                        ui_state.add_log("💥 ¡Bloque de Adoquín demolido! Recuperado 1x Adoquín en mochila.".to_string());
                    }
                    Err(e) => ui_state.add_log(e.to_string()),
                }
            }
            _ => {
                ui_state.add_log("El objetivo no se puede cosechar ni demoler.".to_string());
            }
        }
    } else if let Some(path) = find_path(world, (player.x, player.y), (tx, ty)) {
        *destination_marker = Some((tx, ty));
        player.set_path(path);
        ui_state.add_log(format!(
            "Moviéndose hacia ({}, {}) para cosechar/demoler...",
            tx, ty
        ));
    } else {
        ui_state.add_log(format!("Destino ({}, {}) fuera de alcance o bloqueado.", tx, ty));
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let font = load_ttf_font_from_bytes(FONT_DATA).ok();
    let cover_texture = Texture2D::from_file_with_format(COVER_DATA, None);
    cover_texture.set_filter(FilterMode::Linear);
    let renderer = AsciiRenderer::new(font.clone());

    let mut app_state = AppState::MainMenu {
        menu_state: MenuState::new(),
    };

    loop {
        let dt = get_frame_time().min(0.1);

        match &mut app_state {
            AppState::MainMenu { menu_state } => {
                let action = render_main_menu(&cover_texture, menu_state, font.as_ref());
                match action {
                    MenuAction::NewGame => {
                        let seed = macroquad::rand::gen_range(1000, 99999);
                        app_state = AppState::WorldSelect {
                            overworld: Overworld::new(seed),
                            is_fast_travel: false,
                        };
                    }
                    MenuAction::LoadGame => {
                        if let Some(save) = SaveGame::load() {
                            app_state = load_game_state(save);
                        }
                    }
                    MenuAction::Exit => {
                        #[cfg(not(target_arch = "wasm32"))]
                        std::process::exit(0);
                    }
                    MenuAction::None => {}
                }
            }

            AppState::WorldSelect {
                overworld,
                is_fast_travel,
            } => {
                let res = render_overworld_screen(overworld, font.as_ref(), *is_fast_travel);

                // Volver al Menú Principal o cancelar Fast Travel
                if res.return_to_main_menu {
                    if *is_fast_travel {
                        if let Some(save) = SaveGame::load() {
                            app_state = load_game_state(save);
                            next_frame().await;
                            continue;
                        }
                    } else {
                        app_state = AppState::MainMenu {
                            menu_state: MenuState::new(),
                        };
                        next_frame().await;
                        continue;
                    }
                }

                if res.exit_to_main_menu_force {
                    app_state = AppState::MainMenu {
                        menu_state: MenuState::new(),
                    };
                    next_frame().await;
                    continue;
                }

                // Cargar partida guardada desde disco
                if res.load_saved_game {
                    if let Some(save) = SaveGame::load() {
                        app_state = load_game_state(save);
                        next_frame().await;
                        continue;
                    }
                }

                if res.regenerate && !*is_fast_travel {
                    overworld.regenerate();
                }

                if res.embark {
                    let seed = overworld.seed;
                    let selected_biome = res.selected_biome;

                    // Establecer la nueva estación base del continente ("tomando eso como el nuevo comienzo")
                    overworld.set_station(overworld.cursor_x, overworld.cursor_y);

                    // Generar el mundo 3D de 100 capas con relieve según el bioma elegido
                    let world = World::new(seed, selected_biome);
                    let mut player = Player::new(world.spawn_pos);

                    if *is_fast_travel {
                        if let Some(prev_save) = SaveGame::load() {
                            player.health = prev_save.health;
                            player.hunger = prev_save.hunger;
                            player.thirst = prev_save.thirst;
                            player.energy = prev_save.energy;
                            if let Some(inv) = prev_save.inventory {
                                player.inventory = inv;
                            }
                        }
                    }

                    let camera = Camera::new(player.x, player.y, player.z);

                    let mut ui_state = UiState::new();
                    let spawn_tile_name = world
                        .get_tile(player.x, player.y, player.z)
                        .map(|t| t.name)
                        .unwrap_or("Solid Ground");

                    ui_state.add_log(format!(
                        "Embarked at Continent (X:{}, Y:{}) into {} atop {}.",
                        overworld.cursor_x,
                        overworld.cursor_y,
                        selected_biome.info().name,
                        spawn_tile_name
                    ));

                    // Auto-guardado de la partida
                    let save = SaveGame::create_from_state(
                        &world,
                        &player,
                        overworld.seed,
                        overworld.cursor_x,
                        overworld.cursor_y,
                        overworld.visited_stations.clone(),
                    );
                    let _ = save.save();

                    let ow_clone = overworld.clone();

                    app_state = AppState::Playing {
                        world,
                        player,
                        camera,
                        overworld: ow_clone,
                        ui_state,
                        radar: Radar::new(),
                        inventory_ui: InventoryUiState::new(),
                        destination_marker: None,
                        last_drag_pos: None,
                        is_build_mode: false,
                        is_demolish_mode: false,
                        build_material: BuildMaterial::WoodPlanks,
                        active_hotbar_slot: 0,
                    };
                }
            }

            AppState::Playing {
                world,
                player,
                camera,
                overworld,
                ui_state,
                radar,
                inventory_ui,
                destination_marker,
                last_drag_pos,
                is_build_mode,
                is_demolish_mode,
                build_material,
                active_hotbar_slot,
            } => {
                camera.update_viewport(screen_width(), screen_height(), 330.0, 38.0, 28.0);
                world.update_time(dt);
                radar.update(dt, player.x, player.y, player.z, world);

                if let Some(event) = player.update(dt, world) {
                    match event {
                        PlayerEvent::ReachedDestination => {
                            *destination_marker = None;
                            ui_state.add_log("Arrived at target destination.".to_string());
                        }
                        PlayerEvent::DrankWater => {
                            ui_state.add_log("Drank cool fresh water.".to_string());
                        }
                        PlayerEvent::PickedUpItem(item, count) => {
                            ui_state.add_log(format!("Picked up {}x {}.", count, item.name()));
                        }
                        PlayerEvent::LowThirstWarning => {
                            ui_state.add_log("WARNING: You are dehydrated! Seek fresh water.".to_string());
                        }
                        PlayerEvent::LowHungerWarning => {
                            ui_state.add_log("WARNING: Hunger is weakening your body!".to_string());
                        }
                        PlayerEvent::Exhausted => {
                            ui_state.add_log("You are exhausted. Rest to recover stamina.".to_string());
                        }
                    }
                }

                // Auto-pickup ground items when player steps over them
                if let Some(picked) = world.pickup_item_at(player.x, player.y, 0.75) {
                    let leftover = player.inventory.add_item(picked.item, picked.count);
                    if leftover == 0 {
                        ui_state.add_log(format!("Collected {}x {} from ground.", picked.count, picked.item.name()));
                    } else {
                        world.spawn_ground_item(picked.item, leftover, player.x, player.y, player.z);
                    }
                }

                camera.smooth_follow(player.x as f32 + 0.5, player.y as f32 + 0.5, dt);

                let mouse_pos = mouse_position();
                let (mx, my) = (mouse_pos.0, mouse_pos.1);

                let (hover_gx, hover_gy) = camera.screen_to_world(mx, my);
                let is_hover_in_bounds = hover_gx >= 0
                    && hover_gx < MAP_WIDTH as i32
                    && hover_gy >= 0
                    && hover_gy < MAP_HEIGHT as i32;

                let hover_tile_info = if is_hover_in_bounds && camera.is_point_in_viewport(mx, my) {
                    world
                        .find_visible_surface_down(hover_gx, hover_gy, camera.view_z)
                        .map(|(found_z, t)| (hover_gx, hover_gy, found_z, t.clone()))
                } else {
                    None
                };

                let hover_coord = if is_hover_in_bounds && camera.is_point_in_viewport(mx, my) {
                    Some((hover_gx, hover_gy))
                } else {
                    None
                };

                let build_reticle = if *is_build_mode {
                    Some((build_material.glyph(), build_material.color(), *is_demolish_mode))
                } else {
                    None
                };

                // 1. RENDERIZADO DEL MUNDO (Capa 0: dibujado primero dentro del viewport)
                renderer.render(
                    world,
                    player,
                    camera,
                    hover_coord,
                    *destination_marker,
                    build_reticle,
                );

                // 2. RENDERIZADO DE LA INTERFAZ PRINCIPAL (Capa 1: dibujado ENCIMA del mundo)
                let ui_res = render_ui(
                    world,
                    player,
                    camera,
                    hover_tile_info.as_ref().map(|(gx, gy, gz, t)| (*gx, *gy, *gz, t)),
                    ui_state,
                    font.as_ref(),
                    *is_build_mode,
                    *is_demolish_mode,
                    *build_material,
                    *active_hotbar_slot,
                );

                // ==============================================================
                // GUARDADO RÁPIDO CON TECLA F5 O BOTÓN DE UI
                // ==============================================================
                if is_key_pressed(KeyCode::F5) || ui_res.clicked_save {
                    let save = SaveGame::create_from_state(
                        world,
                        player,
                        overworld.seed,
                        overworld.cursor_x,
                        overworld.cursor_y,
                        overworld.visited_stations.clone(),
                    );
                    if save.save().is_ok() {
                        ui_state.add_log("Expedition state saved to disk [F5].".to_string());
                    }
                }

                // ==============================================================
                // RETORNAR AL MENÚ PRINCIPAL DESDE BOTÓN DE UI
                // ==============================================================
                if ui_res.clicked_exit_to_menu {
                    let save = SaveGame::create_from_state(
                        world,
                        player,
                        overworld.seed,
                        overworld.cursor_x,
                        overworld.cursor_y,
                        overworld.visited_stations.clone(),
                    );
                    let _ = save.save();

                    app_state = AppState::MainMenu {
                        menu_state: MenuState::new(),
                    };
                    next_frame().await;
                    continue;
                }

                // ==============================================================
                // VIAJE RÁPIDO: SALIDA DEL CUADRADO DE JUEGO (BORDES O BOTÓN M)
                // ==============================================================
                let at_west_edge = player.x <= 0;
                let at_east_edge = player.x >= (MAP_WIDTH as i32 - 1);
                let at_north_edge = player.y <= 0;
                let at_south_edge = player.y >= (MAP_HEIGHT as i32 - 1);
                let is_at_edge = at_west_edge || at_east_edge || at_north_edge || at_south_edge;
                let trigger_manual = ui_res.clicked_fast_travel || is_key_pressed(KeyCode::M);

                if is_at_edge {
                    // Determinar qué región continental vecina corresponde a esta dirección
                    let (target_ow_x, target_ow_y) = if at_west_edge {
                        (overworld.cursor_x.saturating_sub(1), overworld.cursor_y)
                    } else if at_east_edge {
                        ((overworld.cursor_x + 1).min(OW_WIDTH - 1), overworld.cursor_y)
                    } else if at_north_edge {
                        (overworld.cursor_x, overworld.cursor_y.saturating_sub(1))
                    } else {
                        (overworld.cursor_x, (overworld.cursor_y + 1).min(OW_HEIGHT - 1))
                    };

                    let target_is_land = overworld
                        .get_tile(target_ow_x, target_ow_y)
                        .map(|t| t.is_land)
                        .unwrap_or(false);

                    player.cancel_movement();
                    *destination_marker = None;
                    player.x = player.x.clamp(2, MAP_WIDTH as i32 - 3);
                    player.y = player.y.clamp(2, MAP_HEIGHT as i32 - 3);

                    // Guardar progreso con posición segura adentro del mapa
                    let save = SaveGame::create_from_state(
                        world,
                        player,
                        overworld.seed,
                        overworld.cursor_x,
                        overworld.cursor_y,
                        overworld.visited_stations.clone(),
                    );
                    let _ = save.save();

                    if target_is_land && !(target_ow_x == overworld.cursor_x && target_ow_y == overworld.cursor_y) {
                        // Hay tierra continental adyacente: abrir Fast Travel hacia esa región
                        overworld.cursor_x = target_ow_x;
                        overworld.cursor_y = target_ow_y;
                    } else {
                        // El borde da al océano o límite: expedición marítima a tierra más cercana
                        let (nearest_lx, nearest_ly) = overworld.find_nearest_land(target_ow_x, target_ow_y);
                        overworld.cursor_x = nearest_lx;
                        overworld.cursor_y = nearest_ly;
                    }

                    let ow_clone = overworld.clone();
                    app_state = AppState::WorldSelect {
                        overworld: ow_clone,
                        is_fast_travel: true,
                    };
                    next_frame().await;
                    continue;
                } else if trigger_manual {
                    // Activación manual desde tecla M o botón de Fast Travel
                    player.cancel_movement();
                    *destination_marker = None;
                    player.x = player.x.clamp(2, MAP_WIDTH as i32 - 3);
                    player.y = player.y.clamp(2, MAP_HEIGHT as i32 - 3);

                    let save = SaveGame::create_from_state(
                        world,
                        player,
                        overworld.seed,
                        overworld.cursor_x,
                        overworld.cursor_y,
                        overworld.visited_stations.clone(),
                    );
                    let _ = save.save();

                    // Poner el cursor sobre la estación actual para que can_embark sea válido
                    if let Some((sx, sy)) = overworld.current_station {
                        overworld.cursor_x = sx;
                        overworld.cursor_y = sy;
                    }

                    let ow_clone = overworld.clone();
                    app_state = AppState::WorldSelect {
                        overworld: ow_clone,
                        is_fast_travel: true,
                    };
                    next_frame().await;
                    continue;
                }

                // ==============================================================
                // ENTRADA: RUEDA DEL RATÓN (CTRL = ZOOM FOV  |  NORMAL = CAPA Z)
                // ==============================================================
                let is_ctrl_down = is_key_down(KeyCode::LeftControl)
                    || is_key_down(KeyCode::RightControl);
                let (_, wheel_y) = mouse_wheel();

                if wheel_y != 0.0 {
                    if is_ctrl_down {
                        if wheel_y > 0.0 {
                            camera.zoom_by(1.15);
                            ui_state.add_log("Zoomed in (Field of view focused).".to_string());
                        } else {
                            camera.zoom_by(0.85);
                            ui_state.add_log("Zoomed out (Field of view widened).".to_string());
                        }
                    } else {
                        if wheel_y > 0.0 {
                            camera.change_view_z(1);
                            ui_state.add_log(format!("Viewing Layer Z = {:+03}", camera.view_z));
                        } else {
                            camera.change_view_z(-1);
                            ui_state.add_log(format!("Viewing Layer Z = {:+03}", camera.view_z));
                        }
                    }
                }

                if ui_res.clicked_up_layer {
                    camera.change_view_z(1);
                    ui_state.add_log(format!("Viewing Layer Z = {:+03}", camera.view_z));
                }
                if ui_res.clicked_down_layer {
                    camera.change_view_z(-1);
                    ui_state.add_log(format!("Viewing Layer Z = {:+03}", camera.view_z));
                }
                if ui_res.clicked_surface_layer {
                    camera.set_view_z(Z_SURFACE);
                    ui_state.add_log("Camera reset to Layer 0 (Surface).".to_string());
                }
                if ui_res.clicked_player_layer {
                    camera.set_view_z(player.z);
                    camera.follow_player = true;
                    ui_state.add_log(format!("Camera centered on player (Layer Z = {}).", player.z));
                }
                if ui_res.clicked_drink {
                    if player.drink_water(world) {
                        ui_state.add_log("Drank freshwater. Thirst quenched.".to_string());
                    }
                }

                // Abrir/cerrar Inventario y Crafteo [I]
                if is_key_pressed(KeyCode::I) || ui_res.clicked_inventory {
                    inventory_ui.toggle();
                }

                // Alternar visibilidad de la lista del Radar [TAB]
                if is_key_pressed(KeyCode::Tab) || ui_res.clicked_radar {
                    radar.toggle_list_visibility();
                }

                // Recoger item cercano, despellejar presa animal o interactuar [E]
                if is_key_pressed(KeyCode::E) && !inventory_ui.is_open {
                    // Prioridad 1: Despellejar / Carnear si hay un cadáver de presa animal cerca
                    if let Some(_carcass) = world.pickup_carcass_at(player.x, player.y, 1.8) {
                        let has_knife = player.inventory.has_item(ItemType::FlintKnife, 1)
                            || player.inventory.equipment.main_hand == Some(ItemType::FlintKnife);
                        let (leather, bones, meat) = if has_knife {
                            (3, 3, 3)
                        } else {
                            (1, 2, 1)
                        };
                        let rem_l = player.inventory.add_item(ItemType::Leather, leather);
                        let rem_b = player.inventory.add_item(ItemType::Bone, bones);
                        let rem_m = player.inventory.add_item(ItemType::RawMeat, meat);
                        if rem_l > 0 { world.spawn_ground_item(ItemType::Leather, rem_l, player.x, player.y, player.z); }
                        if rem_b > 0 { world.spawn_ground_item(ItemType::Bone, rem_b, player.x, player.y, player.z); }
                        if rem_m > 0 { world.spawn_ground_item(ItemType::RawMeat, rem_m, player.x, player.y, player.z); }

                        if has_knife {
                            ui_state.add_log(format!(
                                "🔪 Skinned and butchered carcass with Flint Knife! Harvested {}x Leather, {}x Bones, and {}x Raw Meat.",
                                leather, bones, meat
                            ));
                        } else {
                            ui_state.add_log(format!(
                                "Roughly butchered carcass by hand: harvested {}x Leather, {}x Bones, and {}x Raw Meat (Equip a Flint Knife for clean pelts).",
                                leather, bones, meat
                            ));
                        }
                    } else if let Some(picked) = world.pickup_item_at(player.x, player.y, 1.8) {
                        let leftover = player.inventory.add_item(picked.item, picked.count);
                        if leftover == 0 {
                            ui_state.add_log(format!("Picked up {}x {} [E].", picked.count, picked.item.name()));
                        } else {
                            let added = picked.count - leftover;
                            world.spawn_ground_item(picked.item, leftover, player.x, player.y, player.z);
                            ui_state.add_log(format!("Picked up {}x {} (Backpack full).", added, picked.item.name()));
                        }
                    } else if player.can_drink_water(world) {
                        if player.drink_water(world) {
                            ui_state.add_log("Drank cool freshwater via [E] key.".to_string());
                        }
                    } else {
                        ui_state.add_log("No items or animal carcasses within reach to pick up or skin.".to_string());
                    }
                }

                // Cazar animal cercano [F / Botón HUNT]
                if (is_key_pressed(KeyCode::F) || ui_res.clicked_hunt) && !inventory_ui.is_open {
                    let has_bow = player.has_ranged_weapon();
                    let attack_range = if has_bow { 8.0 } else { 1.8 };

                    if let Some(c_idx) = world.find_creature_near(player.x, player.y, attack_range) {
                        let c_name = world.creatures[c_idx].kind.name();
                        let cx = world.creatures[c_idx].x;
                        let cy = world.creatures[c_idx].y;
                        let cz = world.creatures[c_idx].z;
                        let kind = world.creatures[c_idx].kind;

                        if has_bow {
                            if player.has_ammo() {
                                player.inventory.remove_item(ItemType::FlintArrow, 1);
                                let is_dead = world.creatures[c_idx].take_damage(16.0);
                                let arrows_left = player.inventory.count_item(ItemType::FlintArrow);
                                ui_state.add_log(format!("🏹 Shot Flint Arrow at {} for 16 damage! (Arrows left: {})", c_name, arrows_left));
                                if is_dead {
                                    world.spawn_ground_item(ItemType::AnimalCarcass, 1, cx, cy, cz);
                                    ui_state.add_log(format!("⚔️ Slew {}! Hunt carcass lies ready for skinning [E].", c_name));
                                } else {
                                    world.creature_flee(c_idx, player.x, player.y);
                                }
                            } else {
                                ui_state.add_log("Out of arrows! Craft Flint Arrows using Sticks, Flint and Grass.".to_string());
                            }
                        } else {
                            let dmg = player.get_attack_damage();
                            let weapon_name = player.inventory.equipment.main_hand.map(|w| w.name()).unwrap_or("Bare Hands");
                            let is_dead = world.creatures[c_idx].take_damage(dmg);
                            ui_state.add_log(format!("⚔️ Struck {} with {} for {:.0} damage!", c_name, weapon_name, dmg));
                            if is_dead {
                                world.spawn_ground_item(ItemType::AnimalCarcass, 1, cx, cy, cz);
                                ui_state.add_log(format!("⚔️ Slew {}! Hunt carcass lies ready for skinning [E].", c_name));
                            } else if kind == CreatureType::Boar {
                                let dmg_taken = player.take_damage(10.0);
                                ui_state.add_log(format!("🐗 Enraged Boar charges and bites you! (-{:.0} HP)", dmg_taken));
                            } else {
                                world.creature_flee(c_idx, player.x, player.y);
                            }
                        }
                    } else if let Some(closest_idx) = world.find_creature_near(player.x, player.y, 25.0) {
                        let c = &world.creatures[closest_idx];
                        if let Some(path) = find_path(world, (player.x, player.y), (c.x, c.y)) {
                            let target_pos = (c.x, c.y);
                            *destination_marker = Some(target_pos);
                            player.set_path(path);
                            ui_state.add_log(format!("Hunting: stalking {} at ({}, {})...", c.kind.name(), c.x, c.y));
                        }
                    } else {
                        ui_state.add_log("No animals in vicinity to hunt.".to_string());
                    }
                }

                // -------------------------------------------------------------
                // MODO CONSTRUCCIÓN: ACTIVAR / DESACTIVAR [Tecla B / Botón UI]
                // -------------------------------------------------------------
                if (is_key_pressed(KeyCode::B) || ui_res.clicked_toggle_build_mode) && !inventory_ui.is_open {
                    *is_build_mode = !*is_build_mode;
                    if *is_build_mode {
                        *is_demolish_mode = false;
                        let cnt = player.inventory.count_item(build_material.item_type());
                        ui_state.add_log(format!(
                            "🔨 [MODO CONSTRUCCIÓN ACTIVADO] Clic Izq: Colocar {} [{}] (x{}) | Presiona [X]: Alternar Destrucción | Clic Der: Moverse | B: Salir",
                            build_material.name_es(),
                            build_material.glyph(),
                            cnt
                        ));
                    } else {
                        ui_state.add_log("🛡️ Modo Construcción desactivado. Navegación normal activa.".to_string());
                    }
                }

                // -------------------------------------------------------------
                // MODO DESTRUCCIÓN / COLOCAR ALTERNANTE [Tecla X / Botón UI]
                // -------------------------------------------------------------
                if (is_key_pressed(KeyCode::X) || ui_res.clicked_toggle_demolish) && !inventory_ui.is_open {
                    if !*is_build_mode {
                        *is_build_mode = true;
                        *is_demolish_mode = true;
                        ui_state.add_log("💥 [MODO DESTRUCCIÓN ACTIVADO] Clic Izquierdo para demoler bloques o cosechar. Presiona [X] para modo Colocar.".to_string());
                    } else {
                        *is_demolish_mode = !*is_demolish_mode;
                        if *is_demolish_mode {
                            ui_state.add_log("💥 [MODO DESTRUCCIÓN ACTIVADO] Clic Izquierdo para demoler o cosechar. Presiona [X] para volver a Colocar.".to_string());
                        } else {
                            let cnt = player.inventory.count_item(build_material.item_type());
                            ui_state.add_log(format!(
                                "🔨 [MODO COLOCAR ACTIVADO] Clic Izquierdo para colocar {} [{}] (x{}). Presiona [X] para modo Destrucción.",
                                build_material.name_es(),
                                build_material.glyph(),
                                cnt
                            ));
                        }
                    }
                }

                // -------------------------------------------------------------
                // HOTBAR DE CONSTRUCCIÓN: SELECCIÓN [Teclas 1..5 / Clic UI]
                // -------------------------------------------------------------
                let mut requested_hotbar_slot = ui_res.clicked_hotbar_slot;
                if !inventory_ui.is_open {
                    if is_key_pressed(KeyCode::Key1) { requested_hotbar_slot = Some(0); }
                    else if is_key_pressed(KeyCode::Key2) { requested_hotbar_slot = Some(1); }
                    else if is_key_pressed(KeyCode::Key3) { requested_hotbar_slot = Some(2); }
                    else if is_key_pressed(KeyCode::Key4) { requested_hotbar_slot = Some(3); }
                    else if is_key_pressed(KeyCode::Key5) { requested_hotbar_slot = Some(4); }
                }

                if let Some(slot_idx) = requested_hotbar_slot {
                    *active_hotbar_slot = slot_idx;
                    if let Some(mat) = player.inventory.get_hotbar_material(slot_idx) {
                        *build_material = mat;
                        *is_build_mode = true;
                        *is_demolish_mode = false;
                        let cnt = player.inventory.count_item(mat.item_type());
                        ui_state.add_log(format!(
                            "🧱 Hotbar [{}]: {} [{}] seleccionado (x{} en mochila). Modo Colocar ACTIVO.",
                            slot_idx + 1,
                            mat.name_es(),
                            mat.glyph(),
                            cnt
                        ));
                    } else if let Some(item) = player.inventory.hotbar[slot_idx] {
                        let cnt = player.inventory.count_item(item);
                        ui_state.add_log(format!(
                            "🎒 Hotbar [{}]: {} (x{} en mochila).",
                            slot_idx + 1,
                            item.name(),
                            cnt
                        ));
                    } else {
                        ui_state.add_log(format!(
                            "ℹ️ Hotbar [{}] está vacío. Abre el Inventario [I] y asigna bloques con las teclas 1-5.",
                            slot_idx + 1
                        ));
                    }
                }

                // Ciclar material de construcción [Tecla R]
                if is_key_pressed(KeyCode::R) && !inventory_ui.is_open {
                    *build_material = build_material.next();
                    *is_demolish_mode = false;
                    let cnt = player.inventory.count_item(build_material.item_type());
                    ui_state.add_log(format!(
                        "🧱 Material activo: {} [{}] (x{} en mochila)",
                        build_material.name_es(),
                        build_material.glyph(),
                        cnt
                    ));
                }

                // Selección de material desde la barra lateral
                if let Some(mat) = ui_res.clicked_select_material {
                    *build_material = mat;
                    *is_build_mode = true;
                    *is_demolish_mode = false;
                    let cnt = player.inventory.count_item(mat.item_type());
                    ui_state.add_log(format!(
                        "🧱 Material seleccionado: {} [{}] (x{} en mochila). Modo Construcción ACTIVO.",
                        mat.name_es(),
                        mat.glyph(),
                        cnt
                    ));
                }

                // -------------------------------------------------------------
                // CONSTRUCCIÓN: COLOCAR TABLAS DE MADERA [C / Botón PLANK]
                // -------------------------------------------------------------
                if (is_key_pressed(KeyCode::C) || ui_res.clicked_build_planks) && !inventory_ui.is_open {
                    *build_material = BuildMaterial::WoodPlanks;
                    *is_build_mode = true;
                    *is_demolish_mode = false;
                    if is_hover_in_bounds {
                        handle_place_block(world, player, ui_state, BuildMaterial::WoodPlanks, hover_gx, hover_gy, destination_marker);
                    } else {
                        ui_state.add_log("Modo Construcción (Tablas de Madera) activo. Haz clic sobre una casilla para construir.".to_string());
                    }
                }

                // -------------------------------------------------------------
                // CONSTRUCCIÓN: COLOCAR ADOQUÍN / COBBLESTONE [V / Botón COBBLE]
                // -------------------------------------------------------------
                if (is_key_pressed(KeyCode::V) || ui_res.clicked_build_cobble) && !inventory_ui.is_open {
                    *build_material = BuildMaterial::Cobblestone;
                    *is_build_mode = true;
                    *is_demolish_mode = false;
                    if is_hover_in_bounds {
                        handle_place_block(world, player, ui_state, BuildMaterial::Cobblestone, hover_gx, hover_gy, destination_marker);
                    } else {
                        ui_state.add_log("Modo Construcción (Adoquín) activo. Haz clic sobre una casilla para construir.".to_string());
                    }
                }

                // -------------------------------------------------------------
                // COSECHA / MINERÍA / TALA / DEMOLICIÓN [Tecla T / Botón HARV/DEM]
                // -------------------------------------------------------------
                if (is_key_pressed(KeyCode::T) || ui_res.clicked_harvest_demolish) && !inventory_ui.is_open {
                    let target_opt = if is_hover_in_bounds {
                        Some((hover_gx, hover_gy))
                    } else {
                        let mut adj = None;
                        for dy in -1..=1 {
                            for dx in -1..=1 {
                                if dx == 0 && dy == 0 { continue; }
                                let ax = player.x + dx;
                                let ay = player.y + dy;
                                let az = world.get_surface_z(ax, ay);
                                if let Some(t) = world.get_tile(ax, ay, az) {
                                    if t.kind == TileKind::TreeTrunk
                                        || t.kind == TileKind::PalmTree
                                        || t.kind == TileKind::Stone
                                        || t.kind == TileKind::MountainRock
                                        || t.kind == TileKind::WoodPlanks
                                        || t.kind == TileKind::Cobblestone
                                    {
                                        adj = Some((ax, ay));
                                        break;
                                    }
                                }
                            }
                            if adj.is_some() { break; }
                        }
                        adj
                    };

                    if let Some((tx, ty)) = target_opt {
                        handle_harvest_demolish(world, player, ui_state, tx, ty, destination_marker);
                    } else {
                        ui_state.add_log("Pasa el cursor sobre un árbol, roca o bloque para cosechar/demoler [T].".to_string());
                    }
                }

                // Teclas de capas
                if is_key_pressed(KeyCode::PageUp) {
                    camera.change_view_z(1);
                    ui_state.add_log(format!("Viewing Layer Z = {:+03}", camera.view_z));
                }
                if is_key_pressed(KeyCode::PageDown) || is_key_pressed(KeyCode::Q) {
                    camera.change_view_z(-1);
                    ui_state.add_log(format!("Viewing Layer Z = {:+03}", camera.view_z));
                }

                if is_key_pressed(KeyCode::Space) {
                    camera.follow_player = true;
                    camera.set_view_z(player.z);
                    ui_state.add_log("Camera re-centered on player.".to_string());
                }

                if is_key_pressed(KeyCode::Escape) {
                    if inventory_ui.is_open {
                        inventory_ui.is_open = false;
                    } else if !player.path.is_empty() {
                        player.cancel_movement();
                        *destination_marker = None;
                        ui_state.add_log("Movement canceled.".to_string());
                    } else {
                        // Guardar y retornar al menú principal
                        let save = SaveGame::create_from_state(
                            world,
                            player,
                            overworld.seed,
                            overworld.cursor_x,
                            overworld.cursor_y,
                            overworld.visited_stations.clone(),
                        );
                        let _ = save.save();

                        app_state = AppState::MainMenu {
                            menu_state: MenuState::new(),
                        };
                        next_frame().await;
                        continue;
                    }
                }

                if is_key_pressed(KeyCode::Equal) || is_key_pressed(KeyCode::KpAdd) {
                    camera.zoom_by(1.15);
                }
                if is_key_pressed(KeyCode::Minus) || is_key_pressed(KeyCode::KpSubtract) {
                    camera.zoom_by(0.85);
                }

                let pan_speed = 22.0 * dt;
                if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
                    camera.x -= pan_speed;
                    camera.follow_player = false;
                }
                if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
                    camera.x += pan_speed;
                    camera.follow_player = false;
                }
                if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
                    camera.y -= pan_speed;
                    camera.follow_player = false;
                }
                if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
                    camera.y += pan_speed;
                    camera.follow_player = false;
                }

                if is_mouse_button_down(MouseButton::Middle) {
                    if let Some((prev_x, prev_y)) = *last_drag_pos {
                        let (cw, ch) = camera.effective_cell_size();
                        camera.x -= (mx - prev_x) / cw;
                        camera.y -= (my - prev_y) / ch;
                        camera.follow_player = false;
                    }
                    *last_drag_pos = Some((mx, my));
                } else {
                    *last_drag_pos = None;
                }

                let is_mouse_over_radar = {
                    let in_badge = mx >= 12.0 && mx <= 272.0 && my >= 44.0 && my <= 70.0;
                    let in_dossier = radar.is_list_visible && mx >= 12.0 && mx <= 342.0 && my >= 76.0 && my <= (76.0 + 490.0);
                    in_badge || in_dossier
                };

                // ==============================================================
                // CLIC DERECHO: SIEMPRE ES PARA MOVERSE (PATHFINDING A*)
                // ==============================================================
                if is_mouse_button_pressed(MouseButton::Right)
                    && !ui_res.is_mouse_over_ui
                    && !inventory_ui.is_open
                    && !is_mouse_over_radar
                    && camera.is_point_in_viewport(mx, my)
                {
                    if is_hover_in_bounds {
                        if let Some(path) = find_path(world, (player.x, player.y), (hover_gx, hover_gy)) {
                            *destination_marker = Some((hover_gx, hover_gy));
                            let steps = path.len();
                            player.set_path(path);
                            ui_state.add_log(format!(
                                "Moviéndose hacia ({}, {}): {} pasos.",
                                hover_gx, hover_gy, steps
                            ));
                        } else {
                            ui_state.add_log(format!(
                                "Destino ({}, {}) bloqueado o fuera de alcance.",
                                hover_gx, hover_gy
                            ));
                        }
                    }
                }

                // ==============================================================
                // CLIC IZQUIERDO: CIRCUNSTANCIAL
                // - En Modo Construcción (B): Coloca bloque (por defecto) o Demuele/Rompe si Modo Destrucción (X) está activo
                // - Fuera de Construcción: Caza de criaturas, faenado de presas, tala/minería/excavación con herramientas
                // ==============================================================
                if is_mouse_button_pressed(MouseButton::Left)
                    && !ui_res.is_mouse_over_ui
                    && !inventory_ui.is_open
                    && !is_mouse_over_radar
                    && camera.is_point_in_viewport(mx, my)
                {
                    if is_hover_in_bounds {
                        if *is_build_mode {
                            if *is_demolish_mode {
                                handle_harvest_demolish(world, player, ui_state, hover_gx, hover_gy, destination_marker);
                            } else {
                                handle_place_block(world, player, ui_state, *build_material, hover_gx, hover_gy, destination_marker);
                            }
                        } else if let Some(c_idx) = world.find_creature_near(hover_gx, hover_gy, 1.2) {
                            let c_name = world.creatures[c_idx].kind.name();
                            let cx = world.creatures[c_idx].x;
                            let cy = world.creatures[c_idx].y;
                            let cz = world.creatures[c_idx].z;
                            let kind = world.creatures[c_idx].kind;
                            let dist = ((player.x - cx).pow(2) as f32 + (player.y - cy).pow(2) as f32).sqrt();
                            let has_bow = player.has_ranged_weapon();
                            let in_range = if has_bow { dist <= 8.0 } else { dist <= 1.8 };

                            if in_range {
                                if has_bow {
                                    if player.has_ammo() {
                                        player.inventory.remove_item(ItemType::FlintArrow, 1);
                                        let is_dead = world.creatures[c_idx].take_damage(16.0);
                                        let arrows_left = player.inventory.count_item(ItemType::FlintArrow);
                                        ui_state.add_log(format!("🏹 Disparo de Flecha de Sílex a {} por 16 daño! (Flechas restantes: {})", c_name, arrows_left));
                                        if is_dead {
                                            world.spawn_ground_item(ItemType::AnimalCarcass, 1, cx, cy, cz);
                                            ui_state.add_log(format!("⚔️ ¡Derrotado {}! Cadáver listo para despellejar [E].", c_name));
                                        } else {
                                            world.creature_flee(c_idx, player.x, player.y);
                                        }
                                    } else {
                                        ui_state.add_log("¡Sin flechas! Craftea Flechas de Sílex en el inventario [I].".to_string());
                                    }
                                } else {
                                    let dmg = player.get_attack_damage();
                                    let weapon_name = player.inventory.equipment.main_hand.map(|w| w.name()).unwrap_or("Puños Desnudos");
                                    let is_dead = world.creatures[c_idx].take_damage(dmg);
                                    ui_state.add_log(format!("⚔️ Golpe a {} con {} por {:.0} daño!", c_name, weapon_name, dmg));
                                    if is_dead {
                                        world.spawn_ground_item(ItemType::AnimalCarcass, 1, cx, cy, cz);
                                        ui_state.add_log(format!("⚔️ ¡Derrotado {}! Cadáver listo para despellejar [E].", c_name));
                                    } else if kind == CreatureType::Boar {
                                        let dmg_taken = player.take_damage(10.0);
                                        ui_state.add_log(format!("🐗 ¡El jabalí enfurecido te embiste! (-{:.0} PS)", dmg_taken));
                                    } else {
                                        world.creature_flee(c_idx, player.x, player.y);
                                    }
                                }
                            } else {
                                // Fuera de rango: mover hacia la presa
                                if let Some(path) = find_path(world, (player.x, player.y), (cx, cy)) {
                                    *destination_marker = Some((cx, cy));
                                    player.set_path(path);
                                    ui_state.add_log(format!("Acechando a {} en ({}, {})...", c_name, cx, cy));
                                }
                            }
                        } else if let Some(_carcass) = world.pickup_carcass_at(hover_gx, hover_gy, 1.8) {
                            let has_knife = player.inventory.has_item(ItemType::FlintKnife, 1)
                                || player.inventory.equipment.main_hand == Some(ItemType::FlintKnife);
                            let (leather, bones, meat) = if has_knife { (3, 3, 3) } else { (1, 2, 1) };
                            let rem_l = player.inventory.add_item(ItemType::Leather, leather);
                            let rem_b = player.inventory.add_item(ItemType::Bone, bones);
                            let rem_m = player.inventory.add_item(ItemType::RawMeat, meat);
                            let hz = world.get_surface_z(hover_gx, hover_gy);
                            if rem_l > 0 { world.spawn_ground_item(ItemType::Leather, rem_l, hover_gx, hover_gy, hz); }
                            if rem_b > 0 { world.spawn_ground_item(ItemType::Bone, rem_b, hover_gx, hover_gy, hz); }
                            if rem_m > 0 { world.spawn_ground_item(ItemType::RawMeat, rem_m, hover_gx, hover_gy, hz); }
                            ui_state.add_log(format!("🔪 Despellejado cadáver: +{}x Cuero, +{}x Huesos, +{}x Carne.", leather, bones, meat));
                        } else if {
                            let hz = world.get_surface_z(hover_gx, hover_gy);
                            let t_kind = world.get_tile(hover_gx, hover_gy, hz).map(|t| t.kind).unwrap_or(TileKind::Air);
                            ((t_kind == TileKind::TreeTrunk || t_kind == TileKind::PalmTree) && player.has_hatchet())
                                || ((t_kind == TileKind::Stone || t_kind == TileKind::MountainRock || t_kind == TileKind::Granite || t_kind == TileKind::IronOre || t_kind == TileKind::CoalOre) && player.has_pickaxe())
                                || ((t_kind == TileKind::Soil || t_kind == TileKind::Grass || t_kind == TileKind::Sand || t_kind == TileKind::TallGrass || t_kind == TileKind::Wildflower || t_kind == TileKind::Subsoil) && player.has_shovel())
                                || ((t_kind == TileKind::WoodPlanks || t_kind == TileKind::Cobblestone) && (player.has_hatchet() || player.has_pickaxe()))
                        } {
                            handle_harvest_demolish(world, player, ui_state, hover_gx, hover_gy, destination_marker);
                        } else if let Some(picked) = world.pickup_item_at(hover_gx, hover_gy, 1.8) {
                            let leftover = player.inventory.add_item(picked.item, picked.count);
                            if leftover == 0 {
                                ui_state.add_log(format!("Recogido {}x {} del suelo.", picked.count, picked.item.name()));
                            } else {
                                let hz = world.get_surface_z(hover_gx, hover_gy);
                                world.spawn_ground_item(picked.item, leftover, hover_gx, hover_gy, hz);
                                ui_state.add_log(format!("Recogido {}x {} (Mochila llena).", picked.count - leftover, picked.item.name()));
                            }
                        } else if player.can_drink_water(world) && ((player.x - hover_gx).pow(2) + (player.y - hover_gy).pow(2)) <= 4 {
                            if player.drink_water(world) {
                                ui_state.add_log("Bebiendo agua fresca. Sed saciada.".to_string());
                            }
                        } else {
                            ui_state.add_log("ℹ️ Clic Derecho SIEMPRE mueve. Clic Izquierdo realiza acciones o construye [B].".to_string());
                        }
                    }
                }


                // Renderizado del HUD del Radar y lista desplegable
                let radar_res = render_radar_hud(radar, font.as_ref(), player.x, player.y);
                if let Some((tx, ty)) = radar_res.clicked_target_coord {
                    if !inventory_ui.is_open {
                        if let Some(path) = find_path(world, (player.x, player.y), (tx, ty)) {
                            *destination_marker = Some((tx, ty));
                            let steps = path.len();
                            player.set_path(path);
                            ui_state.add_log(format!(
                                "Navigating to radar target at ({}, {}): {} steps.",
                                tx, ty, steps
                            ));
                        }
                    }
                }
                if radar_res.toggle_visibility {
                    radar.toggle_list_visibility();
                }

                // Renderizado del modal de Inventario y Crafteo [I] (Estilo Minecraft)
                if inventory_ui.is_open {
                    let inv_res = render_inventory_screen(player, world, inventory_ui, font.as_ref());
                    if let Some(dropped) = inv_res.dropped_item {
                        ui_state.add_log(format!(
                            "Dropped {}x {} onto terrain.",
                            dropped.count,
                            dropped.item.name()
                        ));
                    }
                }
            }
        }

        next_frame().await;
    }
}
