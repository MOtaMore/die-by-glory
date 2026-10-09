========================================================================
                      DIE BY GLORY - ASCII SURVIVAL RPG
                          (WebAssembly / itch.io)
========================================================================

This ZIP archive contains everything required to host Die by Glory on
itch.io as a playable browser HTML5/WebAssembly game.

------------------------------------------------------------------------
ITCH.IO UPLOAD INSTRUCTIONS
------------------------------------------------------------------------
1. Go to your project page on itch.io (Edit game).
2. In the "Kind of project" section, choose "HTML".
3. Under "Uploads", upload this ZIP file directly.
4. Check the box: [x] "This file will be played in the browser".
5. Under "Embed options":
   - Viewport dimensions: 1280 x 720 (or default)
   - [x] Automatically start on page load
   - [x] Fullscreen button
6. Click "Save & view page". The game will run directly in the browser!

------------------------------------------------------------------------
INCLUDED FILES
------------------------------------------------------------------------
  - index.html        : Responsive ASCII canvas container & LocalStorage sync
  - mq_js_bundle.js   : WebGL/WebAudio engine glue
  - die_by_glory.wasm : Optimized standalone WebAssembly game binary

------------------------------------------------------------------------
STORAGE & SAVING
------------------------------------------------------------------------
Saving in the browser uses window.localStorage.
Players can close the browser and return to resume their saved expedition.
========================================================================
