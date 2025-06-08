// Tauri doesn't have a Node.js server to do proper SSR
// so we use adapter-static with a fallback to index.html to put the site in SPA mode
// See: https://svelte.dev/docs/kit/single-page-apps

import { jump, moveWindowBy } from "$lib/window";
import { getCurrentWindow, PhysicalPosition, PhysicalSize } from "@tauri-apps/api/window";
import { moveWindow, Position } from "@tauri-apps/plugin-positioner";

// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
export const ssr = false;

// getCurrentWindow().setIgnoreCursorEvents(true);

moveWindow(Position.BottomRight);
// await moveWindowBy(new PhysicalPosition(400, 0));
// await jump();

// console.log(await getCurrentWindow().innerSize())

// await getCurrentWindow().setSize(
//   new PhysicalSize(30, 50)
// )
// setInterval(moveSlightly, 10)

// setInterval(() => {
//   getCurrentWindow().setPosition(
//     new PhysicalPosition(
//       Math.round(Math.random() * 100),
//       Math.round(Math.random() * 100),
//     ),
//   );
// }, 10);

