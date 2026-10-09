# Die by Glory - ASCII Survival Game (Rust)

A deep, procedurally generated 3D voxel survival game built with **Rust**, featuring pure **ASCII roguelike aesthetics**, persistent game memory storage, hardware-accelerated 60 FPS rendering via Macroquad, and crisp bundled monospace vector typography.

---

## 🌟 Key Features

### 1. Main Menu with Cover JPG Portada (Menú de Inicio)
- **Cover Art Showcase**: The game launches into a cinematic title screen featuring the high-resolution pixel art cover sprite (`assets/cover.jpg`) as an atmospheric background portada, framed by pure ASCII terminal styling.
- **Start New Expedition [N / 1]**: Generates new continents and opens the Overworld map where you choose your landing biome.
- **Load Saved Expedition [L / 2]**: Seamlessly inspects and deserializes `savegame.json` from disk, previewing continental station, coordinates, biome, time, and vitals. Pressing **[L]** or clicking the button instantly resumes your saved adventure.
- **Exit Game [Q / ESC / 3]**: Cleanly terminates the application and returns to desktop.
- **Interactive Navigation**: Full mouse support (hover highlights, click activation) and keyboard controls (Arrow Keys / WASD, Enter / Space, direct hotkeys).

### 2. Game Memory & Persistent State Storage (Memoria de Partida)
- **Automatic & Manual Saving**: The game persists your exact game memory into `savegame.json`:
  - **Continental Location**: Seed, current continental coordinates `(ow_x, ow_y)`, and discovered regions.
  - **Local Stage Location**: Exact `(player_x, player_y, player_z)` relief coordinates.
  - **Vitals & Stats**: Health, Hunger, Thirst, Stamina/Energy.
  - **World Simulation Clock & Biome**: Exact time of day and regional biome.
- **Auto-Save**: Saves automatically when embarking into new regions, exiting boundaries, fast traveling, or exiting to menu.
- **Quick Save**: Press **[F5]** or click **[ 💾 SAVE ]** in the top bar at any time during gameplay.
- **Return to Menu**: Click **[ 🚪 MENU ]** in the top bar or press **[Escape]** when idle to save and return cleanly to the Main Menu.

### 3. Step-by-Step Continental Travel (Aledañas como Nuevo Comienzo)
- **Adjacent-Only Continental Movement**:
  - When you exit the local play area into the Overworld, you can **only travel to neighboring adjacent regions** relative to where you were stationed (`[@]`).
  - Regions too far away are blocked with a clear warning: `[ ⛔ TOO FAR: YOU CAN ONLY TRAVEL TO ADJACENT NEIGHBORING REGIONS ]`.
  - Valid neighboring regions are highlighted with glowing borders on the continental map.
- **New Baseline Progression**:
  - This allows progressive, realistic overland exploration step-by-step across continents and archipelagos.

### 4. Minecraft-Style Inventory & Crafting System [I]
- **Storage Backpack (20 Slots)**: Collect and manage resources with individual stack sizes (up to 64 per stack).
- **Equipment Slots**:
  - **Main Hand**: Tools and weapons giving attack and utility power (Stone Hatchet, Flint Knife, Bone Dagger, Hunting Bow).
  - **Off Hand**: Utility items and illumination (Flint Firestarter, Torch).
  - **Chest Armor**: Wear protective gear like Woven Grass Cloak (`#`) or Cured Leather Armor (`#`) for defense.
  - **Head & Legs**: Expandable armor slots for advanced survival gear.
- **Craftable Survival Tools, Torches & Construction Blocks**:
  - **Stone Pickaxe** (`T`): 3x Small Stones + 2x Sticks + 2x Fiber Cords. Essential for quarrying, mining stone, and demolishing masonry.
  - **Stone Shovel** (`]`): 1x Small Stone + 2x Sticks + 1x Fiber Cord. Digs clay, sand, soil, and excavates trenches.
  - **Stone Hatchet** (`P`): 1x Small Stone + 2x Sticks + 1x Fiber Cord. Fells trees, splits lumber, and +10 melee damage.
  - **Flint Knife** (`/`): 1x Flint + 1x Stick + 1x Fiber Cord. Razor-sharp blade for efficient skinning and +6 melee damage.
  - **Hunting Bow** (`{`): 3x Sticks + 3x Fiber Cords. High-power ranged weapon for hunting game from afar (+14 ranged damage).
  - **Flint Arrows** (`^` x4): 1x Flint + 2x Sticks + 1x Fiber Cord. Ammunition for hunting bows.
  - **Torches** (`i` x2): 1x Stick + 1x Fiber Cord. Portable light source. **When equipped in Main or Off Hand**, it projects a dynamic radiant light halo around the player illuminating the surrounding terrain!
  - **Wood Planks** (`#` x4): 1x Wood Log. Cut wooden construction planks for paving floors and stacking walls.
  - **Cobblestone Block** (`#` x1): 3x Small Stones (or mined directly from rock). Dense chiseled stone block for solid masonry.
  - **Sticks from Planks** (`/` x4): 1x Wood Planks. Whittle planks down into tool handles and arrow shafts.
  - **Woven Grass Cloak** (`#`): 4x Tall Grass + 1x Fiber Cord (+4 Defense).
  - **Cured Leather Armor** (`#`): 4x Leather + 2x Fiber Cords (+12 Defense).
  - **Bone Dagger** (`/`): 2x Bones + 1x Fiber Cord (+8 Melee Damage).
  - **Roasted Cooked Meat** (`%`): 1x Raw Meat + 1x Torch / Heat Source. Satiates hunger (+30) and heals (+15 HP).
  - **Herbal Bandage** (`+`): 2x Tall Grass (+25 HP Heal).

### 5. Forestry, Quarrying, Excavation & 3D Building [C / V / X]
- **Chopping Trees with the Hatchet [Key T / X / Left-Click]**:
  - Stand adjacent to a tree trunk (`T`) with a **Stone Hatchet** equipped.
  - Felling the tree removes the trunk and clears the surrounding canopy foliage, yielding **2x Wood Logs** (`=`) and **2x Sticks** (`/`).
- **Shoveling Earth with the Shovel [Key X / Left-Click]**:
  - Target soil, grass, sand, or dunes with a **Stone Shovel** equipped.
  - Excavates the tile and **lowers the surface relief by 1 Z-level** (creating pits, canals, moats, and terraced steps), yielding **1x Dirt Block** (`o`).
- **Quarrying Rock with the Pickaxe [Key X / Left-Click]**:
  - Target mountain rock, peaks, boulders, or subterranean stone with a **Stone Pickaxe** equipped.
  - Mines the mineral formation, yielding **2x Cobblestone** blocks (`#`) or ores.
- **3D Block Placement & Construction [Key C / V / Right-Click]**:
  - **Build Wood Planks [Key C]**: Paves the targeted cell with warm timber wood planks (`#`).
  - **Build Cobblestone [Key V]**: Paves the targeted cell with fortified cobblestone masonry (`#`).
  - **Vertical Stacking (Walls & Structures)**: Placing a block on top of an already paved tile **raises the relief elevation (+1 Z)**, creating vertical walls, pillars, defensive ramparts, and shelters!
  - **Quick Right-Click**: Right-clicking any cell within reach (3.5m) places the currently held block (or Planks/Cobblestone from your backpack).
- **Demolishing & Resource Recovery [Key X / Left-Click]**:
  - Target any placed structure: demolishing vertical walls reduces the elevation back down; demolishing floors removes the pavement and restores natural subsoil.
  - **100% Resource Refund**: Demolishing wood planks returns 1x `WoodPlanks`; demolishing cobblestone returns 1x `Cobblestone`.
- **Wild Animal Species & Vitals**:
  - **Wild Deer** (`D`, 30 HP): Skittish herbivore, quick to flee at high speed when startled.
  - **Rabbit** (`r`, 12 HP): Small and nimble, hard to catch without a bow or knife.
  - **Wild Boar** (`B`, 45 HP): Tough and territorial. **Enraged Retaliation**: When attacked, boars do not flee—they turn red, enter an enraged state, charge the player and retaliate with melee strikes!
  - **Mountain Goat** (`G`, 35 HP): Agile cliff-dweller of the rocky slopes.
- **Hunting Mechanics**:
  - **Key [F] / [ 🏹 HUNT CREATURE ]**: Targets the nearest wild animal. If equipped with a Hunting Bow and Flint Arrows, shoots the creature from up to 8 tiles away; otherwise, draws your equipped melee weapon and strikes if adjacent, or stalks and closes distance automatically.
  - **Target Click**: Left-clicking directly on a creature also engages hunting.
  - **Damage Calculation**: Equipped weapons scale damage: Bows (+14), Hatchets (+10), Bone Daggers (+8), Flint Knives (+6), or bare hands (+2). Player armor reduces incoming retaliation damage.

### 6. Skinning & Butchering Yields (Cuero, Huesos y Carne)
- **Animal Carcasses** (`%`): Once an animal is defeated, it drops an `Animal Carcass` on the terrain.
- **Field Butchering [Key E]**:
  - Standing adjacent to a carcass and pressing **[E]** immediately skins and dresses the animal on the spot.
  - Alternatively, pick up the carcass and select **[ 🔪 BUTCHER (Key E) ]** inside the Inventory screen.
- **Resource Yields**:
  - **Leather** (`%`, Tawny Brown): Durable tanned hides for crafting **Cured Leather Armor**.
  - **Bones** (`%`, Bone White): Sturdy animal bones for crafting **Bone Daggers** and primitive tools.
  - **Raw Meat** (`%`, Coral Red): Fresh butchered game meat. Can be eaten raw (risky/small nourishment) or roasted over fire into **Roasted Cooked Meat**.
  - **Tool Advantage**: Butchering with a sharp **Flint Knife** or **Bone Dagger** equipped yields maximum resources (3x Leather, 3x Bone, 3x Raw Meat) compared to crude bare-hand dressing.

### 7. Tactical Radar HUD & Proximity Vision Scanner [Tab]
- **Vision Area Activation**: The radar automatically pulses active (`ACTIVE`) whenever resources, items, or wild fauna enter the player's 16-meter line-of-sight.
- **Mini-Radar ASCII Scope**: Features an animated 360° sweeping beam with blips representing the player (`@`), ground resources, and wildlife.
- **Detected Items Dossier**:
  - Displays item glyph, name, quantity, elevation layer (`Z`), precise metric distance, and compass direction (e.g., `North-East [NE]`, `South-West [SW]`).
  - **Click-to-Target**: Click any detected item in the list to automatically pathfind straight to it!
- **Wild Fauna Tracker**:
  - Monitors surrounding animals, showing current HP / max HP bars and active status (Grazing, Alert, Fleeing, Enraged).
- **Region & Terrain Diagnostics**:
  - Live readout of current biome, Z elevation, surface tile material, time of day, and weather atmosphere (e.g. *Sunny Wild Frontier*, *High Altitude Winds*, *Subterranean Damp Air*).
- **Toggle Visibility**: Press **[Tab]** or click the radar banner in the HUD to collapse or expand the dossier.

### 8. Natural Ground Resources & Terrain Gathering
- **Ground Resources Generated Across Terrain**:
  - **Tall Grass** (`"`, Emerald Green): Fiber for cordage and bandages.
  - **Sticks** (`/`, Wood Brown): Branches for tool handles and arrow shafts.
  - **Flint** (`^`, Steel Blue): Hard mineral for sharp knapped blades and arrowheads.
  - **Small Stones** (`•`, Pebble Grey): Smooth river stones for pickaxes, hatchets, and shovels.
- **Interaction**: Simply walk over an item or press **[E]** to pick up adjacent resources. Items can also be dropped back onto the terrain with **[D]**.

### 9. Natural Walkable Surface Relief (3D Slopes & Elevation)
- The walkable surface features **natural 3D hills, slopes, ridges, dunes, and depressions**.
- **Gentle Slopes**: Height differences of $\pm 1$ level are passable slopes. Walking uphill costs slightly more energy/stamina; downhill is easier.
- **Steep Cliffs**: Cliffs with height differences $> 1$ level block direct movement.
- **Player Elevation**: As the character walks up and down hills, their `@` climbs and descends naturally through the Z-levels.

### 10. Full 100-Layer Vertical Simulation
Once embarked, the local world simulates a complete 100-layer volume:
- **Relief Surface (`Z = -2` to `+6`)**: Natural rolling landscape tailored to your chosen embark biome.
- **50 Depth Layers (`Z = -1` to `Z = -50`)**: Subsoil clay, subterranean caverns with bioluminescent fungi (`ψ`), underground lakes, iron (`$`) and coal (`¤`) veins, granite crust, and impenetrable bedrock at `Z = -50`.
- **49 Height Layers (`Z = +1` to `Z = +49`)**: Tree branches and canopies (`Z = +1..+3`), mountain summits, and drifting cloud banks (`Z = +20..+26`).

### 11. Multi-Layer Translucent Vision & Controls
- **Mouse Wheel**: Moves your visual inspection plane up or down through all 100 Z-layers.
- **Dynamic Translucency (Alpha Decay)**: When viewing open air from above, the engine casts downward through the atmosphere, rendering lower surface levels with depth fading and atmospheric haze.
- **Ctrl + Mouse Wheel**: Zooms and scales your visual field of view (FOV).

### 12. Point-and-Click A* Pathfinding along 3D Relief
- **Left Click** on any cell on the map: The A* pathfinding algorithm calculates the optimal route along the 3D surface relief, accounting for slopes, obstacles (trees, deep water, cliffs) and terrain costs.
- The player steps smoothly along the glowing path markers.
- Press **Escape** at any time to cancel movement.

### 13. Pure ASCII Aesthetic & Survival Vitals
- Every element—world map, borders, headers, buttons, stat bars (`[████████░░]`), and level elevators—is rendered in pure ASCII characters.
- **Vitals tracked**:
  - **Health (HP)**: Affected by extreme hunger, thirst, or animal retaliation; healable via Bandages and cooked meals.
  - **Hunger (HUN)**: Depletes over time; satiable by eating Raw Meat or Roasted Cooked Meat.
  - **Thirst (THI)**: Depletes over time; can be quenched by standing next to freshwater and pressing **[B]** or **[E]**.
  - **Energy (ENG)**: Depletes while walking; regenerates when resting.

---

## 🎮 Controls

### Main Menu
| Input | Action |
|---|---|
| **Arrow Up / Down** or **W / S** | Move menu cursor between options |
| **Enter / Space** | Select highlighted menu option |
| **N / 1** | **Start New Expedition** |
| **L / 2** | **Load Saved Expedition** (enabled if save exists) |
| **Q / ESC / 3** | **Exit Game** (clean desktop quit) |
| **Mouse Click / Hover** | Highlight and activate menu buttons |

### Exploration, Hunting, Building & Survival
| Input | Action |
|---|---|
| **I** | **Open / Close Inventory & Crafting Screen** |
| **C** (or UI button) | **Build / Place Wood Planks** on hovered cell |
| **V** (or UI button) | **Build / Place Cobblestone Block** on hovered cell |
| **X / T** (or UI button) | **Chop Tree / Mine Rock / Shovel Earth / Demolish Block** |
| **Right-Click** | **Quick-place active block** onto terrain (or drag to pan) |
| **F** (or UI button) | **Hunt Nearest Wild Animal** (Bow shoot / Melee strike / Track) |
| **E** | **Interact / Skin & Butcher Carcass** / Pick Up Items / Drink Water |
| **Tab** | **Toggle Tactical Radar HUD & Vision Dossier** |
| **D** | **Drop Selected Item** onto ground (when inventory open) |
| **Left Click** | **Travel via A* Pathfinding**, attack animal, or harvest adjacent tile |
| **Click Radar Entry** | **Auto-travel straight to detected item or animal** |
| **Mouse Wheel** | Cycle visual inspection layer across all 100 Z-levels |
| **Ctrl + Mouse Wheel** | Zoom field of view (FOV) in / out |
| **PageUp / PageDown** | Cycle Z-layer inspection via keyboard |
| **Walk Off Map Edge** | **Trigger Fast Travel to adjacent continental regions** |
| **M** (or UI button) | Open Fast Travel continent map |
| **F5** (or UI button) | **Quick Save expedition state to disk (`savegame.json`)** |
| **Escape** (in inventory) | Close Inventory screen |
| **Escape** (when idle) | **Save game & return to Main Menu** |
| **Escape** (moving) | Cancel active movement path |
| **Space** | Center camera and view layer onto player |
| **Right-Click Drag / Middle-Click Drag** | Pan camera view freely across the map |
| **Arrow Keys / WASD** | Pan camera view via keyboard |
| **`+` / `-`** (or numpad) | Zoom field of view in / out |
| **B** (or UI button) | Drink freshwater if adjacent to river/water |
| **1 .. 6** (World Map) | Quick-select starting biome on Overworld screen (New Game) |
| **R** (World Map) | Regenerate continents and islands with a new seed |
| **Enter / Space** (World Map) | Embark into the chosen region |
| **ESC** (World Map) | Return to Main Menu (or cancel fast travel) |

---

## 🚀 Running the Project

### 1. Local Windows Execution (Escritorio Local)
You can launch the game on Windows directly:

- **Single-Click Launcher**: Double-click [`run_local.bat`](file:///a:/die-by-glory/run_local.bat)
- **Cargo Run (Development)**:
  ```powershell
  cargo run
  ```
- **Cargo Run (Release)**:
  ```powershell
  cargo run --release
  ```
- **Run Unit & Simulation Test Suite**:
  ```powershell
  cargo test
  ```

---

### 2. Browser & itch.io WebAssembly Execution (Navegador Web / itch.io)
*Die by Glory* compiles into a lightweight (~510 KB zipped) WebAssembly binary that runs in modern browsers with zero external plugins.

#### Automated Build & itch.io Packaging
To build the WebAssembly version and generate the itch.io package:
- **Windows Batch**: Double-click or run [`build_web.bat`](file:///a:/die-by-glory/build_web.bat)
- **PowerShell**: Run [`build_web.ps1`](file:///a:/die-by-glory/build_web.ps1)
- **Manual Cargo Command**:
  ```powershell
  cargo build --target wasm32-unknown-unknown --release
  ```

This generates:
- `dist_web/`: Complete standalone HTML5 package:
  - `index.html`: Responsive ASCII container, auto-focus canvas, prevents itch.io iframe scrolling.
  - `mq_js_bundle.js`: High-performance WebGL/WebAudio glue.
  - `die_by_glory.wasm`: Optimized WebAssembly game binary.
- `dist_web.zip`: Direct upload package formatted for itch.io.

#### Testing Locally in Browser
You can test the web version locally with Python or Node:
```powershell
python -m http.server 8080 -d dist_web
# Open http://localhost:8080 in your browser
```

#### Uploading to itch.io
1. Go to your game project dashboard on **itch.io**.
2. Under **Kind of project**, select **HTML**.
3. Under **Uploads**, click **Upload files** and select `dist_web.zip`.
4. Check the box **"This file will be played in the browser"**.
5. Under **Embed options**:
   - Set viewport dimensions (e.g. `1280 x 720` or check **Fullscreen button** / **Responsive**).
   - Check **"Automatically start on page load"**.
6. Save and view your page. The game will run directly inside the browser with complete LocalStorage persistent saving!
