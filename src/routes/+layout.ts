// Tauri doesn't have a Node.js server to do proper SSR
// so we use adapter-static with a fallback to index.html to put the site in SPA mode
// See: https://svelte.dev/docs/kit/single-page-apps

import { jump, moveWindowBy } from "$lib/window";
import { PhysicalPosition } from "@tauri-apps/api/window";
import { moveWindow, Position } from "@tauri-apps/plugin-positioner";

// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
export const ssr = false;

moveWindow(Position.BottomRight);
await moveWindowBy(new PhysicalPosition(400, 0));
await jump();

// setInterval(moveSlightly, 10)

// setInterval(() => {
//   getCurrentWindow().setPosition(
//     new PhysicalPosition(
//       Math.round(Math.random() * 100),
//       Math.round(Math.random() * 100),
//     ),
//   );
// }, 10);

